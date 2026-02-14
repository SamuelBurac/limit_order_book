pub mod orderbook;

use rand::{RngExt, random_bool};
use std::time::SystemTime;
use tracing::info;

use crate::orderbook::{book::OrderBook, side::Side};

fn main() {
    let starting_price = 54321; //$543.21
    //Timestamp when starting
    println!("Starting simuliation {:?}", SystemTime::now());

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

    //Timestamp when ending
    println!("Ending simuliation {:?}", SystemTime::now());

    println!("Added Orders I guess we're done at ");
}
