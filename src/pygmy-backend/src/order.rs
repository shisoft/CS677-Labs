#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::web::{Json, Query};
use crate::data::establish_connection;
use crate::models::{Topic, Item, LookupRes, Order, NewOrder};
use serde::{Serialize, Deserialize};
use crate::configs::*;

mod data;
mod models;
mod schema;
mod configs;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        {
            App::new()
                .route("/order/{id}", web::get().to(order_handler))
        })
        .bind(format!("0.0.0.0:{}", *ORDER_SERVER_PORT))?
        .run()
        .await
}

async fn order_handler(req: HttpRequest) -> impl Responder {
    #[derive(Deserialize)]
    struct QueryFormat {
        amount: i32
    }
    let item_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let query = Query::<QueryFormat>::from_query(req.query_string()).unwrap();
    let order_amount: i32 = query.amount;
    // First query the catalog server for item information and check stock
    let cat_server = format!("http://{}:{}/", *CAT_SERVER_ADDR, *CAT_SERVER_PORT);
    let cat_lookup: LookupRes<Item> =
        reqwest::get(
            &format!("{}/lookup/{}", cat_server, item_id))
            .await.unwrap()
            .json()
            .await
            .unwrap();
    if cat_lookup.ok {
        let lookup_item =  cat_lookup.result.unwrap();
        if lookup_item.stock >= order_amount {
            let update_res = reqwest::Client::new().post(
                &format!("{}/update/{}/stock/deduct/{}", cat_server, item_id, order_amount)
            ).send().await.unwrap().json::<bool>().await.unwrap();
            if update_res {
                use schema::order::dsl::*;
                let conn = establish_connection();
                diesel::insert_into(order)
                    .values(&NewOrder {
                        item: item_id,
                        amount: order_amount,
                        total: lookup_item.price * order_amount as f32
                    })
                    .execute(&conn);
            }
        }
    }
    HttpResponse::Ok().json(false)
}