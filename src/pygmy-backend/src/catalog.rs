#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use crate::models::{Topic, Item};
use actix_web::web::Json;

pub mod schema;
pub mod models;

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
        .bind("127.0.0.1:8000")?
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
                .load::<Item>(&establish_connection())
                .unwrap()
        )
}

async fn update_stock(req: HttpRequest) -> impl Responder {
    use schema::item::dsl::*;
    let item_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let stock_deduct: i32 = req.match_info().get("stock").unwrap().parse().unwrap();
    diesel::update(item).filter(id.eq(item_id)).set(stock.eq(stock - stock_deduct));
    HttpResponse::Ok().json(())
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}



