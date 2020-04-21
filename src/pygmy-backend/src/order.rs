#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bifrost;
extern crate dotenv;
#[macro_use]
extern crate log;

use crate::configs::*;
use crate::data::establish_connection;
use crate::models::{Item, LookupRes, NewOrder, Order, Topic};
use actix_web::middleware::Logger;
use actix_web::web::{Json, Query};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use bifrost::raft;
use bifrost::raft::client::RaftClient;
use bifrost::raft::state_machine::StateMachineCtl;
use bifrost::raft::*;
use diesel::prelude::*;
use dotenv::dotenv;
use future::FutureExt;
use log::*;
use serde::{Deserialize, Serialize};
use futures::executor::block_on;

mod configs;
mod data;
mod models;
mod schema;

struct ReplicatedOrderLog;
const STATE_MACHINE_ID: u64 = 100;

// Define the interface of replicated state machine for order server here
// The state machine only responsible for logging the order to the database, because replicas cannot
// check with catalog for stock on each of the nodes
raft_state_machine! {
    // Define log order as a command, with 3 parameters
    def cmd log_order(item: i32, amount: i32, total: f32);
}

// Define the implementation of the RSM, which is to write the log to database
impl StateMachineCmds for ReplicatedOrderLog {
    fn log_order(&mut self, item_id: i32, num_amount: i32, total_sum: f32) -> BoxFuture<()> {
        info!(
            "RSM replicating the order log, item {}, amount {}, total {}",
            item_id, num_amount, total_sum
        );
        use schema::order::dsl::*;
        let conn = establish_connection();
        diesel::insert_into(order)
            .values(&NewOrder {
                item: item_id,
                amount: num_amount,
                total: total_sum,
            })
            .execute(&conn)
            .unwrap();
        future::ready(()).boxed()
    }
}

lazy_static! {
    static ref SM_CLIENT: client::SMClient = {
        debug!("Construct state machine client from {:?}", &*ORDER_RAFT_SERVER_LIST);
        block_on(async {
            // Create a client for raft service
            let raft_client = RaftClient::new(&*ORDER_RAFT_SERVER_LIST, DEFAULT_SERVICE_ID)
                .await
                .unwrap();
            // Create a client for the state machine on the raft service
            client::SMClient::new(STATE_MACHINE_ID, &raft_client)
        })
    };
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    simple_logger::init_with_level(Level::Debug);

    info!("Running Order server");
    // Initialize raft server and their service
    // The TCP server responsible for Raft use a dedicated binary protocol, require its own server
    // apart from the HTTP Restful server
    start_raft_state_machine(Box::new(ReplicatedOrderLog), &*ORDER_RAFT_SERVER_LIST).await;

    HttpServer::new(|| {
        App::new()
            // Setup logging middleware for HTTP server
            .wrap(Logger::default())
            // Set route. Order server have only one
            .route("/order/{id}", web::post().to(order_handler))
    })
    // Bind port
    .bind(format!("0.0.0.0:{}", *ORDER_SERVER_PORT))?
    // Run the server
    .run()
    .await
}

async fn order_handler(req: HttpRequest) -> impl Responder {
    #[derive(Deserialize)]
    struct QueryFormat {
        amount: i32,
    }
    let item_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let query = Query::<QueryFormat>::from_query(req.query_string()).unwrap();
    let order_amount: i32 = if query.amount > 0 { query.amount } else { 1 };
    info!(
        "Ordering {}, amount {}. Checking stock on catalog server.",
        item_id, order_amount
    );
    // First query the catalog server for item information and check stock
    let cat_server = format!("http://{}:{}", *CAT_SERVER_ADDR, *CAT_SERVER_PORT);
    let cat_lookup: LookupRes<Item> = reqwest::get(&format!("{}/lookup/{}", cat_server, item_id))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    if cat_lookup.ok {
        let lookup_item = cat_lookup.result.unwrap();
        info!("Found item {:?}, start transaction", lookup_item);
        if lookup_item.stock >= order_amount {
            let update_res = reqwest::Client::new()
                .post(&format!(
                    "{}/update/{}/stock/deduct/{}",
                    cat_server, item_id, order_amount
                ))
                .send()
                .await
                .unwrap()
                .json::<bool>()
                .await
                .unwrap();
            if update_res {
                info!(
                    "Order transaction for {} successful, log transaction",
                    item_id
                );
                log_order(
                    item_id,
                    order_amount,
                    lookup_item.price * order_amount as f32,
                )
                .await;
                return HttpResponse::Ok().json(true);
            } else {
                info!("Order transaction for {} failed, aborted", item_id);
            }
        } else {
            info!("Don't have enough item {} in stock. Abort.", item_id);
        }
    } else {
        info!("Cannot find item {} in catalog server", item_id);
    }
    HttpResponse::Ok().json(false)
}

async fn log_order(item_id: i32, num_amount: i32, total_sum: f32) {
    // Invoke the state machine, like a RPC call
    SM_CLIENT
        .log_order(&item_id, &num_amount, &total_sum)
        .await
        .unwrap()
}

impl StateMachineCtl for ReplicatedOrderLog {
    // Auto generate stub dispatcher
    raft_sm_complete!();
    // Unique id for this state machine
    fn id(&self) -> u64 {
        STATE_MACHINE_ID
    }

    fn snapshot(&self) -> Option<Vec<u8>> {
        // We are not going to use this feature in the project
        unimplemented!();
    }

    fn recover(&mut self, data: Vec<u8>) -> BoxFuture<'_, ()> {
        // We are not going to use this feature in the project
        unimplemented!()
    }
}
