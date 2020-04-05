use serde::{Serialize, Deserialize};
use crate::schema::{item, order};

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
    pub total: f32
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Topic {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct LookupRes {
    pub ok: bool,
    pub result: Option<Item>
}

#[derive(Insertable)]
#[table_name = "order"]
pub struct NewOrder {
    pub item: i32,
    pub amount: i32,
    pub total: f32
}