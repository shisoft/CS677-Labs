use std::{env, io};
use std::process::exit;
use crate::models::{LookupRes, Item};
use std::collections::HashMap;
use std::io::Read;

mod models;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let server = &args[1];
    println!("Welcome to Pygmy.com - the World’s smallest book store");
    println!("Server URL is: {}", server);
    println!("You can");
    loop {
        println!("1. Search books by topic ('Distributed systems' and 'Graduate School')");
        println!("2. Get available book list");
        println!("3. Get information about one book");
        println!("4. Buy a book, for free!!!");
        println!("5. Leave");
        println!("Select by input the number: ");
        if let Some(num) = read_num() {
            match num {
                1 => {
                    println!("What is the topic? Any keywords in the topics will be fine (try 'sys'):");
                    let mut str_in = String::new();
                    io::stdin().read_line(&mut str_in).unwrap();
                    let topic_kw = str_in.trim();
                    search_book_by_topic(server, topic_kw).await;
                    wait_for_return_key();
                },
                2 => {
                    list_all_books(server).await;
                    println!("All books listed.");
                    wait_for_return_key();
                },
                3 => {
                    println!("Which book do you want to look at? Input the number: ");
                    if let Some(id) = read_num() {
                        lookup_one(server, id).await
                    } else {
                        println!("Don't know what is that book, please try again with a number");
                    }
                    wait_for_return_key();
                },
                4 => {
                    list_all_books(server).await;
                    println!("Which book would you like: ");
                    if let Some(item_id) = read_num() {
                        println!("amount: ");
                        if let Some(amount) = read_num() {
                            buy_book(server, item_id, amount).await
                        } else {
                            println!("Please input a number for amount");
                        }
                    } else {
                        println!("Don't know what is that book, please try again with a number");
                    }
                    wait_for_return_key();
                },
                5 => {
                    println!("You want to leave. Bye bye.");
                    exit(0);
                },
                _ => {
                    println!("Don't know that number, please try again");
                    continue;
                }
            }
        } else {
            println!("Don't know what is that, please try again by inputting a number");
            continue;
        }
    }
}

async fn list_all_books(server: &String) {
    query_list(&format!("{}/lookup", server)).await
}

async fn search_book_by_topic(server: &String, topic: &str) {
    query_list(&format!("{}/search/{}", server, topic)).await
}

async fn query_list(addr: &String) {
    let lookup: LookupRes<Vec<Item>> = reqwest::get(addr)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let topics = topic_map(&lookup);
    for item in lookup.result.as_ref().unwrap() {
        pretty_print_item(item, &topics);
    }
    if lookup.result.as_ref().unwrap().is_empty() {
        println!("Noting to show about that");
    }
}

async fn lookup_one(server: &String, id: i32) {
    let lookup = book_by_id(server, id).await;
    if !lookup.ok {
        println!("Cannot find the book");
    } else {
        let topics = topic_map(&lookup);
        pretty_print_item(lookup.result.as_ref().unwrap(), &topics);
    }
}

async fn buy_book(server: &String, id: i32, amount: i32) {
    let book = book_by_id(server, id).await;
    if !book.ok {
        println!("Cannot find the book to buy, id {}", id);
        return;
    }
    let success: bool = reqwest::get(&format!("{}/order/{}?amount={}", server, id, amount))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    if success {
        println!("Bought book {}, amount {}", book.result.as_ref().unwrap().name, amount);
    }
}

async fn book_by_id(server: &String, id: i32) -> LookupRes<Item> {
    reqwest::get(&format!("{}/lookup/{}", server, id))
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

fn topic_map<T>(lookup: &LookupRes<T>) -> HashMap<i32, String> {
    lookup.topics.iter().map(|t| (t.id, t.name.clone())).collect()
}

fn wait_for_return_key() {
    println!("Press return key to continue");
    io::stdin().read_line(&mut String::new());
}

fn pretty_print_item(item: &Item, topics: &HashMap<i32, String>) {
    println!("{}. {} - Price: {}, Topic: {}, Stock: {}",
             item.id,
             item.name,
             item.price,
             topics[&item.topic],
             item.stock
    );
}

fn read_num() -> Option<i32> {
    let mut str_in = String::new();
    io::stdin().read_line(&mut str_in).unwrap();
    let item_id = str_in.trim();
    item_id.parse().ok()
}