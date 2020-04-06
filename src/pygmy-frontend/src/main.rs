#[macro_use]
extern crate lazy_static;

use actix_web::{App, HttpServer, web, HttpRequest, Responder, HttpResponse};
use actix_web::middleware::Logger;
use actix_web::web::Query;
use actix_web::http::header;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use crate::configs::*;

mod configs;

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
            // Search route. Topic is a string that does not require exact matching
            .route("/search/{topic}", web::get().to(search_handler))
            // Lookup an item with exact item id
            .route("/lookup/{id}", web::get().to(lookup_handler))
            // List all items in the database
            .route("/lookup", web::get().to(list_all))
            // Set route for order server
            .route("/order/{id}", web::post().to(order_handler))
    })
        // Set binding port
        .bind(format!("0.0.0.0:{}", *FRONT_SERVER_PORT))?
        // Run the server
        .run()
        .await
}

async fn search_handler(req: HttpRequest) -> impl Responder {
    // Get topic string
    let topic_query = req.match_info().get("topic").unwrap_or("");
    response_with(reqwest::get(&format!("{}/search/{}", *CAT_SERVER_ADDR, topic_query)).await.unwrap().text().await.unwrap())
}

async fn lookup_handler(req: HttpRequest) -> impl Responder {
    // Get item it from url
    let item_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    response_with(reqwest::get(&format!("{}/lookup/{}", *CAT_SERVER_ADDR, item_id)).await.unwrap().text().await.unwrap())
}

async fn list_all(req: HttpRequest) -> impl Responder {
    // Get all items
    response_with(reqwest::get(&format!("{}/lookup", *CAT_SERVER_ADDR)).await.unwrap().text().await.unwrap())
}


async fn order_handler(req: HttpRequest) -> impl Responder {
    #[derive(Deserialize)]
    struct QueryFormat {
        amount: i32,
    }
    let item_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let query = Query::<QueryFormat>::from_query(req.query_string()).unwrap();
    let order_amount: i32 = if query.amount > 0 { query.amount } else { 1 };
    response_with(
        reqwest::Client::new()
            .post(&format!(
                "{}/order/{}?amount={}",
                *ORDER_SERVER_ADDR, item_id, order_amount
            ))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap())
}

fn response_with(res_text: String) -> impl Responder {
    let mut res = HttpResponse::Ok();
    res.header(header::CONTENT_TYPE, "application/json");
    res.body(res_text)
}


