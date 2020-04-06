use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// Establish connection for each request. Because we are using embedded SQLite, no bother caching it
pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
