#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
extern crate dotenv;

use crate::configs::*;
use crate::data::establish_connection;
use crate::models::{Item, LookupRes, NewOrder, Order, Topic};
use actix_web::middleware::Logger;
use actix_web::web::{Json, Query};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use log::*;
use serde::{Deserialize, Serialize};

mod configs;
mod data;
mod models;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initialize configure reader
    dotenv().ok();
    // Initialize logger
    simple_logger::init().unwrap();
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
                use schema::order::dsl::*;
                let conn = establish_connection();
                diesel::insert_into(order)
                    .values(&NewOrder {
                        item: item_id,
                        amount: order_amount,
                        total: lookup_item.price * order_amount as f32,
                    })
                    .execute(&conn);
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
