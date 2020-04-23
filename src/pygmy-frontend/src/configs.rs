use dotenv::dotenv;
use std::env;

// Preload all configurations from environment variable
lazy_static! {
    pub static ref FRONT_SERVER_PORT: String =
        env::var("FRONT_SERVER_PORT").unwrap_or("34800".to_string());
    pub static ref CAT_SERVER_PORT: String =
        env::var("CAT_SERVER_PORT").unwrap_or("34801".to_string());
    pub static ref ORDER_SERVER_PORT: String =
        env::var("ORDER_SERVER_PORT").unwrap_or("34802".to_string());
    pub static ref CAT_SERVER_LIST: Vec<String> = env::var("CATALOG_SERVER_LIST")
        .unwrap_or("127.0.0.1".to_string())
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();
    pub static ref ORDER_SERVER_LIST: Vec<String> = env::var("ORDER_SERVER_LIST")
        .unwrap_or("127.0.0.1".to_string())
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();
    pub static ref CAT_SERVER_ADDR: Vec<String> = 
        CAT_SERVER_LIST.iter().map(|addr| format!("http://{}:{}", addr, *CAT_SERVER_PORT)).collect();
    pub static ref ORDER_SERVER_ADDR: Vec<String> = 
        ORDER_SERVER_LIST.iter().map(|addr| format!("http://{}:{}", addr, *ORDER_SERVER_PORT)).collect();
}
