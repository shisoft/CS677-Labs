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
        println!("5. Test");
        println!("6. Leave");
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
                            buy_book(server, item_id, amount).await.unwrap()
                        } else {
                            println!("Please input a number for amount");
                        }
                    } else {
                        println!("Don't know what is that book, please try again with a number");
                    }
                    wait_for_return_key();
                },
                5 => {
                    println!("Run tests to see if everything is working");
                    test(server).await;
                    wait_for_return_key();
                },
                6 => {
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

async fn list_all_books(server: &String) -> io::Result<LookupRes<Vec<Item>>> {
    query_list(&format!("{}/lookup", server)).await
}

async fn search_book_by_topic(server: &String, topic: &str) -> io::Result<LookupRes<Vec<Item>>> {
    query_list(&format!("{}/search/{}", server, topic)).await
}

async fn query_list(addr: &String) -> io::Result<LookupRes<Vec<Item>>> {
    let lookup: LookupRes<Vec<Item>> = reqwest::get(addr)
        .await?
        .json()
        .await?;
    let topics = topic_map(&lookup);
    for item in lookup.result.as_ref().unwrap() {
        pretty_print_item(item, &topics);
    }
    if lookup.result.as_ref().unwrap().is_empty() {
        println!("Noting to show about that");
    }
    Ok(lookup)
}

async fn lookup_one(server: &String, id: i32) {
    let lookup = book_by_id(server, id).await.unwrap();
    if !lookup.ok {
        println!("Cannot find the book");
    } else {
        let topics = topic_map(&lookup);
        pretty_print_item(lookup.result.as_ref().unwrap(), &topics);
    }
}

async fn buy_book(server: &String, id: i32, amount: i32) -> io::Result<bool> {
    let book = book_by_id(server, id).await.unwrap();
    if !book.ok {
        println!("Cannot find the book to buy, id {}", id);
        return Ok(false);
    }
    let success: bool = reqwest::get(&format!("{}/order/{}?amount={}", server, id, amount))
        .await?
        .json()
        .await?;
    if success {
        println!("Bought book {}, amount {}", book.result.as_ref().unwrap().name, amount);
    }
    Ok(success)
}

async fn book_by_id(server: &String, id: i32) -> io::Result<LookupRes<Item>> {
    reqwest::get(&format!("{}/lookup/{}", server, id))
        .await?
        .json()
        .await?
}

async fn test(server: &String) {
    let num_tests = 5;
    println!("Running test 1 of {}", num_tests);
    print!("Getting all books in store and verify the number...");
    let lookup = list_all_books(server).await.unwrap();
    // Response should be ok
    assert!(lookup.ok);
    // Should have 4 books
    assert_eq!(lookup.result.as_ref().unwrap().len(), 4);
    // Should have 2 topics
    assert_eq!(lookup.topics.len(), 2);
    println!("PASSED");
    println!("Running test 2 of {}", num_tests);
    let book_1_name = "How to get a good grade in 677 in 20 minutes a day";
    print!("Get one book, number 1, should be the '{}'...", book_1_name);
    let book_1 = book_by_id(server, 1).await.unwrap();
    // Verify response is ok
    assert!(book_1.ok());
    // There should be only one topic
    assert_eq!(book_1.topics.len(), 1);
    // It should have the right name
    assert_eq!(book_1.result.unwrap().name, book_1_name);
    println!("PASSED");
    println!("Running test 3 of {}", num_tests);
    println!("Search topic related books 'sys', should support fuzzy matching and match 1 topic and 2 books");
    let lookup = search_book_by_topic(server, "sys").await.unwrap();
    // Response should be ok
    assert!(lookup.ok);
    // Should have 2 books
    assert_eq!(lookup.result.as_ref().unwrap().len(), 2);
    // Should have 1 topic
    assert_eq!(lookup.topics.len(), 1);
    println!("PASSED");
    println!("Running test 4 of {}", num_tests);
    println!("Buy a book, number 2, amount 1...", num_tests);
    let book_2 = book_by_id(server, 2).await.unwrap();
    let item = book_2.result.as_ref().unwrap();
    assert!(book_2.ok());
    // There should be only one topic
    assert_eq!(book_2.topics.len(), 1);
    // It should have stock. Else, skip
    if item.stock > 0 {
        assert!(buy_book(server, 2, 1).await.unwrap());
        println!("PASSED");
    } else {
        println!("Book 2 out of stock, skipped");
    }
    println!("Running test 5 of {}", num_tests);
    println!("Checking remaining stock of book 2...");
    let book_2_re = book_by_id(server, 2).await.unwrap();
    let item_re = book_2.result.as_ref().unwrap();
    assert!(book_2_re.ok());
    // There should be only one topic
    assert_eq!(book_2_re.topics.len(), 1);
    // Check stock
    assert_eq!(item_re.stock, item.stock - 1);
    println!("PASSED");
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