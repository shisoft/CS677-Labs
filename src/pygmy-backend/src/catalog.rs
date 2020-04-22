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

mod configs;
mod data;
mod models;
mod schema;

use crate::configs::*;
use crate::data::establish_connection;
use crate::models::{Item, LookupRes, Topic};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use dotenv::dotenv;
use log::*;
use std::collections::HashMap;

use bifrost::raft;
use bifrost::raft::client::RaftClient;
use bifrost::raft::state_machine::StateMachineCtl;
use bifrost::raft::*;
use futures::FutureExt;
use futures::executor::block_on;

struct ReplicatedCatalog;
const STATE_MACHINE_ID: u64 = 50;

lazy_static! {
    // Pre-initialize topics from database
    static ref TOPICS: HashMap<i32, Topic> = {
        use schema::topic::dsl::*;
        topic.load::<Topic>(&establish_connection())
        .unwrap()
        .into_iter()
        // take its id as key, topic content as value
        .map(|t| (t.id, t))
        .collect()
    };
}

// Define the interface of replicated state machine for catalog server here
// The state machine responsible for both read (query, search) and write (update stock) operations
// Note that for read operation, the framework enables read on replicas in round-robin fashion
// In the mean while, it will compare the log term of the replica with last seen term to ensure read
// linearizability
// For write, will replicated in the same order
raft_state_machine! {
    // Define search query
    def qry search(topic: String) -> LookupRes<Vec<Item>>;
    // Define lookup query
    def qry lookup(item_id: i32) -> LookupRes<Item>;
    // Define list a;; query
    def qry list_all() -> LookupRes<Vec<Item>>;
    // Define update command
    def cmd update_stock_deduct(item_id: i32, stock_deduct: i32) -> bool;
}

impl StateMachineCmds for ReplicatedCatalog {
    fn search(&self, topic_qry: String) -> BoxFuture<LookupRes<Vec<Item>>> {
        info!("Searching {}", topic_qry);
        use schema::item::dsl::*;
        // Extract topics matches query
        let topic_matched = TOPICS
            .values()
            .filter_map(|matching_topic| {
                // By checking the topic lowercase string contains searching string in lowercase
                if matching_topic
                    .name
                    .to_lowercase()
                    .contains(&topic_qry.to_lowercase())
                {
                    Some(matching_topic)
                } else {
                    None
                }
            })
            .collect::<Vec<&Topic>>();
        // Query by SQL statement, finding items in the matching topics
        // The ORM we are using diesel does not support IN statement for query, we compile it by ourselves
        let items = diesel::sql_query(format!(
            "SELECT * FROM item WHERE topic IN ({})",
            topic_matched
                .iter()
                .map(|i| format!("{}", i.id))
                .collect::<Vec<_>>()
                .join(", ")
        ))
            .load::<Item>(&establish_connection())
            .unwrap();
        // Compose a structure indicates the status of the result
        let mut res = LookupRes::from_lookup::<()>(Ok(items));
        // Provide topic name and it as one of the field in result because topic in item is topic id
        res.topics = topic_matched.into_iter().cloned().collect();
        future::ready(res).boxed()
    }

    fn lookup(&self, item_id: i32) -> BoxFuture<LookupRes<Item>> {
        info!("Lookup {}", item_id);
        use schema::item::dsl::*;
        let mut res = LookupRes::from_lookup(
            // Get the item from database by its id
            // Using diesel query DSL
            item.filter(id.eq(item_id))
                .get_result::<Item>(&establish_connection()),
        );
        // Check if we can find the item. If yes, attach the topic information
        if res.ok {
            res.topics = vec![TOPICS[&res.result.as_ref().unwrap().topic].clone()]
        }
        future::ready(res).boxed()
    }

    fn list_all(&self) -> BoxFuture<LookupRes<Vec<Item>>> {
        info!("List all");
        use schema::item::dsl::*;
        // Get all items
        let all = item.load::<Item>(&establish_connection());
        let mut res = LookupRes::from_lookup(all);
        // Attach all topics
        res.topics = TOPICS.values().cloned().collect();
        future::ready(res).boxed()
    }

    fn update_stock_deduct(&mut self, item_id: i32, stock_deduct: i32) -> BoxFuture<bool> {
        info!("Deduct item {} amount {}", item_id, stock_deduct);
        use schema::item::dsl::*;
        // Get connection
        let conn = establish_connection();
        // Run a transaction.
        // In this transaction, we need to check if there are enough stock for the transaction.
        // If not enough stock, do nothing and return false
        let txn_res: Result<_, diesel::result::Error> = conn.transaction(|| {
            // Get the item entity
            if let Ok(i) = item.filter(id.eq(item_id)).get_result::<Item>(&conn) {
                // Only update when there are enough stock to deduct
                if i.stock >= stock_deduct {
                    // Update stock number for the item
                    diesel::update(item)
                        .filter(id.eq(item_id))
                        .set(stock.eq(i.stock - stock_deduct))
                        .execute(&conn)
                        .unwrap();
                    return Ok(true);
                }
            }
            Ok(false)
        });
        // Return the transaction result
        future::ready(txn_res.unwrap()).boxed()
    }
}

lazy_static! {
    static ref SM_CLIENT: client::SMClient = {
        debug!("Construct state machine client from {:?}", &*CATALOG_RAFT_SERVER_LIST);
        block_on(async {
            // Create a client for raft service
            let raft_client = RaftClient::new(&*CATALOG_RAFT_SERVER_LIST, DEFAULT_SERVICE_ID)
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
    info!("Running Catalog server");
    // Start the raft state machine for catalog server
    info!("Catalog server have peers: {:?}", &*CATALOG_SERVER_LIST);
    start_raft_state_machine(Box::new(ReplicatedCatalog), &*CATALOG_RAFT_SERVER_LIST).await;

    HttpServer::new(|| {
        App::new()
            // Setup logging middleware for HTTP server
            .wrap(Logger::default())
            // Search route. Topic is a string that does not require exact matching
            .route("/search/{topic}", web::get().to(search_handler))
            // Lookup an item with exact item id
            .route("/lookup/{id}", web::get().to(lookup_handler))
            // List all items in the database
            .route("/lookup", web::get().to(list_all))
            // Update item stock
            .route(
                "/update/{id}/stock/deduct/{stock}",
                web::post().to(update_stock),
            )
    })
    // Set binding port
    .bind(format!("0.0.0.0:{}", *CAT_SERVER_PORT))?
    // Run the server
    .run()
    .await
}

async fn search_handler(req: HttpRequest) -> impl Responder {
    // Get topic string
    let topic_query = req.match_info().get("topic").unwrap_or("").to_string();
    let task = tokio::spawn(async move {
        SM_CLIENT.search(&topic_query).await.unwrap()
    });
    // Return the result to client in Json from state machine
    HttpResponse::Ok().json(task.await.unwrap())
}

async fn lookup_handler(req: HttpRequest) -> impl Responder {
    // Get item it from url
    let item_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let task = tokio::spawn(async move {
        SM_CLIENT.lookup(&item_id).await.unwrap()
    });
    // Return the result
    HttpResponse::Ok().json(task.await.unwrap())
}

async fn list_all(req: HttpRequest) -> impl Responder {
    let task = tokio::spawn(async move {
        SM_CLIENT.list_all().await.unwrap()
    });
    HttpResponse::Ok().json(task.await.unwrap())
}

async fn update_stock(req: HttpRequest) -> impl Responder {
    debug!("Trying to update stock by sending command to state machine");
    let item_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let stock_deduct: i32 = req.match_info().get("stock").unwrap().parse().unwrap();
    let task = tokio::spawn(async move {
        SM_CLIENT.update_stock_deduct(&item_id, &stock_deduct).await.unwrap()
    });
    let res: bool = task.await.unwrap();
    HttpResponse::Ok().json(res)
}

impl StateMachineCtl for ReplicatedCatalog {
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