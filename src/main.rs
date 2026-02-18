pub mod orderbook;

use chrono::{DateTime, Local};
use rand::{RngExt, random_bool};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

use crate::orderbook::{book::OrderBook, side::Side};

fn main() {
    let starting_price = 54321; //$543.21
    let start_time = SystemTime::now();
    let date_time: DateTime<Local> = start_time.into();
    //Timestamp when starting
    println!("Starting simuliation {}", date_time);

    //Create orderbook

    let mut orderbook = OrderBook::new();

    //add 1 million orders and see what happens I guess
    // super basic but we can do multi threaded later
    let mut rng = rand::rng();

    for i in 0..1_000_000 {
        // randomly add an order for some price
        let quantity = rng.random_range(10..100_000);

        let neg = random_bool(0.5);
        let random_price = rng.random_range(0..70);
        let price = if neg {
            starting_price - random_price
        } else {
            starting_price + random_price
        };

        let side = if rng.random_bool(0.5) {
            Side::Sell
        } else {
            Side::Buy
        };

        orderbook.add_order(orderbook::order::LimitOrder {
            order_id: i,
            side,
            price,
            quantity,
        });
        info!("Adding order {i}");
    }

    let end_time = SystemTime::now();
    let time_since = end_time.duration_since(start_time);
    //Timestamp when ending
    println!("Ending simuliation {:?}", time_since);

    // write orderbook state to file with timestamp
    let timestamp = end_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let filename = format!("orderbook_state_{}.md", timestamp);
    orderbook.write_state_to_file(&filename);
    println!("Order book state written to {}", filename);
}
