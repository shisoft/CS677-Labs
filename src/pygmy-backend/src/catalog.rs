#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
extern crate dotenv;

mod data;
mod models;
mod schema;
mod configs;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::web::Json;
use crate::data::establish_connection;
use crate::models::{LookupRes, Item, Topic};
use crate::configs::*;

lazy_static! {
    static ref TOPICS: Vec<Topic> = {
        use schema::topic::dsl::*;
        topic.load::<Topic>(&establish_connection()).unwrap()
    };
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        {
            App::new()
                .route("/search/{topic}", web::get().to(search_handler))
                .route("/lookup/{id}", web::get().to(lookup_handler))
                .route("/update/{id}/stock/deduct/{stock}", web::post().to(update_stock))
        })
        .bind(format!("0.0.0.0:{}", *CAT_SERVER_PORT))?
        .run()
        .await
}

async fn search_handler(req: HttpRequest) -> impl Responder {
    use schema::item::dsl::*;
    let topic_query = req.match_info().get("topic").unwrap_or("");
    let topic_matched = TOPICS.iter().filter_map(|matching_topic| {
        if matching_topic.name.to_lowercase().contains(&topic_query.to_lowercase()) {
            Some(format!("{}", matching_topic.id))
        } else {
            None
        }
    }).collect::<Vec<String>>();
    let res = diesel::sql_query(format!(
        "SELECT * FROM item WHERE topic IN ({})",
        topic_matched.join(", ")
    )).load::<Item>(&establish_connection()).unwrap();
    HttpResponse::Ok().json(res)
}

async fn lookup_handler(req: HttpRequest) -> impl Responder {
    use schema::item::dsl::*;
    let item_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    HttpResponse::Ok()
        .json(item
            .filter(id.eq(item_id))
            .get_result::<Item>(&establish_connection())
            .map(|i| LookupRes {
                ok: true,
                result: Some(i)
            })
            .unwrap_or_else(|_| LookupRes {
                ok: false,
                result: None
            })
        )
}

async fn update_stock(req: HttpRequest) -> impl Responder {
    use schema::item::dsl::*;
    let item_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let stock_deduct: i32 = req.match_info().get("stock").unwrap().parse().unwrap();
    let conn = establish_connection();
    let txn_res: Result<_, diesel::result::Error> = conn.transaction(|| {
        if let Ok(i) = item.filter(id.eq(item_id)).get_result::<Item>(&conn) {
            if i.stock >= stock_deduct {
                diesel::update(item)
                    .filter(id.eq(item_id))
                    .set(stock.eq(i.stock - stock_deduct))
                    .execute(&conn);
                return Ok(true);
            }
        }
        Ok(false)
    });
    HttpResponse::Ok().json(txn_res.unwrap())
}


