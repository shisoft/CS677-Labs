#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use actix_web::{App, HttpServer, web, HttpRequest, Responder, HttpResponse};
use actix_web::middleware::Logger;
use actix_web::web::Query;
use actix_web::http::header;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use crate::configs::*;
use log::Level;
use parking_lot::*;
use std::collections::HashMap;
use std::future::Future;
use std::sync::atomic::*;

mod configs;

lazy_static! {
    static ref SEARCH_CACHES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref LOOKUP_CACHES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref LIST_ALL_CACHE: Mutex<Option<String>> = Mutex::new(None);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    simple_logger::init_with_level(Level::Debug);
    
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
            // Set route for invalidation
            .route("/invalidate/item/{id}", web::post().to(invalidate_item))
    })
        // Set binding port
        .bind(format!("0.0.0.0:{}", *FRONT_SERVER_PORT))?
        // Run the server
        .run()
        .await
}

async fn search_handler(req: HttpRequest) -> impl Responder {
    // Get topic string
    let topic_query = req.match_info().get("topic").unwrap_or("").to_string();
    response_with(
        cached_future(
            &topic_query,
            &*SEARCH_CACHES,
            async {
                reqwest::get(&format!("{}/search/{}", next_catalog_server(), topic_query)).await.unwrap().text().await.unwrap()
            }).await)
}

async fn lookup_handler(req: HttpRequest) -> impl Responder {
    // Get item it from url
    let item_id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    response_with(
        cached_future(
            &format!("{}", item_id),
            &*LOOKUP_CACHES,
            async {
                reqwest::get(&format!("{}/lookup/{}", next_catalog_server(), item_id)).await.unwrap().text().await.unwrap()
            }).await)
}

async fn list_all(req: HttpRequest) -> impl Responder {
    // Get all items
    let res = if let Some(list) = LIST_ALL_CACHE.lock().clone() {
        debug!("Use cache for list all items");
        list
    } else {
        debug!("Getting data from down stream catalog server for list all");
        let response = reqwest::get(&format!("{}/lookup", next_catalog_server())).await.unwrap().text().await.unwrap();
        *LIST_ALL_CACHE.lock() = Some(response.clone());
        response
    };
    response_with(res)
}

async fn invalidate_item(req: HttpRequest) -> impl Responder {
    let item_id = req.match_info().get("id").unwrap();
    info!("Invalidate item {}", item_id);
    // Clear search and list all cache
    SEARCH_CACHES.lock().clear();
    *LIST_ALL_CACHE.lock() = None;
    // Invalidate cache for this item
    LOOKUP_CACHES.lock().remove(&item_id.to_string());
    HttpResponse::Ok().json(true)
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
                next_order_server(), item_id, order_amount
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

async fn cached_future<F: Future<Output = String>>(key: &String, cache: &Mutex<HashMap<String, String>>, fut: F) -> String {
    let cached = cache.lock().get(key).cloned();
    if let Some(val) = cached {
        debug!("Using cached result for {}", key);
        val
    } else {
        debug!("Using remote result for {}", key);
        let remote = fut.await;
        cache.lock().insert(key.clone(), remote.clone());
        remote
    }
}

// Pick up servers in round-robin fashion
lazy_static! {
    static ref CATALOG_CLOCK: AtomicUsize = AtomicUsize::new(0);
    static ref ORDER_CLOCK: AtomicUsize = AtomicUsize::new(0);
}
fn next_catalog_server() -> &'static String {
    let len = CAT_SERVER_ADDR.len();
    &CAT_SERVER_ADDR[CATALOG_CLOCK.fetch_add(1, Ordering::Relaxed) % len]
}

fn next_order_server() -> &'static String {
    let len = CAT_SERVER_ADDR.len();
    &ORDER_SERVER_ADDR[ORDER_CLOCK.fetch_add(1, Ordering::Relaxed) % len]
}
