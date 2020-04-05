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
use std::collections::HashMap;
use actix_web::error::ParseError::TooLarge;

lazy_static! {
    static ref TOPICS: HashMap<i32, Topic> = {
        use schema::topic::dsl::*;
        topic.load::<Topic>(&establish_connection())
        .unwrap()
        .into_iter()
        .map(|t| (t.id, t))
        .collect()
    };
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(||
        {
            App::new()
                .route("/search/{topic}", web::get().to(search_handler))
                .route("/lookup/{id}", web::get().to(lookup_handler))
                .route("/lookup", web::get().to(list_all))
                .route("/update/{id}/stock/deduct/{stock}", web::post().to(update_stock))
        })
        .bind(format!("0.0.0.0:{}", *CAT_SERVER_PORT))?
        .run()
        .await
}

async fn search_handler(req: HttpRequest) -> impl Responder {
    use schema::item::dsl::*;
    let topic_query = req.match_info().get("topic").unwrap_or("");
    let topic_matched = TOPICS.values().filter_map(|matching_topic| {
        if matching_topic.name.to_lowercase().contains(&topic_query.to_lowercase()) {
            Some(matching_topic)
        } else {
            None
        }
    }).collect::<Vec<&Topic>>();
    let items = diesel::sql_query(format!(
        "SELECT * FROM item WHERE topic IN ({})",
        topic_matched.iter().map(|i| format!("{}", i.id)).collect::<Vec<_>>().join(", ")
    )).load::<Item>(&establish_connection()).unwrap();
    let mut res = LookupRes::from_lookup::<()>(Ok(items));
    res.topics = topic_matched.into_iter().cloned().collect();
    HttpResponse::Ok().json(res)
}

async fn lookup_handler(req: HttpRequest) -> impl Responder {
    use schema::item::dsl::*;
    let item_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let mut res = LookupRes::from_lookup(
        item
        .filter(id.eq(item_id))
        .get_result::<Item>(&establish_connection())
    );
    if res.ok {
        res.topics = vec![TOPICS[&res.result.as_ref().unwrap().topic].clone()]
    }
    HttpResponse::Ok().json(res)
}

async fn list_all(req: HttpRequest) -> impl Responder {
    use schema::item::dsl::*;
    let all = item.load::<Item>(&establish_connection());
    let mut res = LookupRes::from_lookup(all);
    res.topics = TOPICS.values().cloned().collect();
    HttpResponse::Ok().json(res)
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


