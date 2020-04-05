use serde::{Serialize, Deserialize};
use crate::schema::item;

#[derive(Queryable, QueryableByName, Serialize, Deserialize)]
#[table_name = "item"]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub stock: i32,
    pub price: f32,
    pub topic: i32
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub item: i32,
    pub amount: i32,
    pub total: i32
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Topic {
    pub id: i32,
    pub name: String
}