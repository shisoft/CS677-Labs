// Define all models here for database and data transmission

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub stock: i32,
    pub price: f32,
    pub topic: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub item: i32,
    pub amount: i32,
    pub total: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Topic {
    pub id: i32,
    pub name: String,
}

// A generic wrapper for HTTP response, will encode to json and decode at client
#[derive(Serialize, Deserialize)]
pub struct LookupRes<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub topics: Vec<Topic>,
}

impl<T> LookupRes<T> {
    // Generate the object from generic typed result
    pub fn from_lookup<E>(res: Result<T, E>) -> Self {
        res.map(|i| LookupRes {
            ok: true,
            result: Some(i),
            topics: vec![],
        })
        .unwrap_or_else(|_| LookupRes {
            ok: false,
            result: None,
            topics: vec![],
        })
    }
}
