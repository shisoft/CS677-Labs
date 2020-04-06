use std::{env, io};
use std::process::exit;
use crate::models::{LookupRes, Item};
use std::collections::HashMap;

mod models;

fn main() {
    let args: Vec<String> = env::args().collect();
    let server = argsp[0];
    println!("Welcome to Pygmy.com - the World’s smallest book store");
    println!("You can");
    loop {
        println!("1. Search books by topic ('Distributed systems' and 'Graduate School')");
        println!("2. Get available book list");
        println!("3. Get information about one book");
        println!("4. Buy a book, for free!!!");
        println!("5. Leave");
        println!("Select by input the number: ");
        let mut str_in = String::new();
        io::stdin().read_line(&mut str_in).unwrap();
        if let Ok(num) = str_in.parse() {
            match num {
                1 => {

                },
                2 => {

                },
                3 => {

                },
                4 => {

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

fn list_all_books(server: &String) {
    let lookup: LookupRes<Vec<Item>> = reqwest::get(&format!("{}/lookup", server))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let topics = topic_map(&lookup);
    for item in lookup.result.as_ref().unwrap() {
        println!("{}. {} - Price: {}, Topic: {}, Stock: {}",
                 item.id,
                 item.name,
                 item.price,
                 topics[item.topic],
                 item.stock
        );
    }
}

fn topic_map<T>(lookup: &LookupRes<T>) -> HashMap<i32, String> {
    lookup.topics.iter().map(|t| (t.id, t.name.clone())).collect()
}
