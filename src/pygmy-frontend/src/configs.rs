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
    pub static ref CAT_SERVER_HOST: String =
        env::var("CAT_SERVER_ADDR").unwrap_or("127.0.0.1".to_string());
    pub static ref ORDER_SERVER_HOST: String =
        env::var("ORDER_SERVER_ADDR").unwrap_or("127.0.0.1".to_string());
    pub static ref CAT_SERVER_ADDR: String = format!("http://{}:{}", *CAT_SERVER_HOST, *CAT_SERVER_PORT);
    pub static ref ORDER_SERVER_ADDR: String = format!("http://{}:{}", *ORDER_SERVER_HOST, *ORDER_SERVER_PORT);
}
