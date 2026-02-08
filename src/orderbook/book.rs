// This will hold all the orders and execute trades?

use std::collections::HashMap;
use std::sync::atomic::Ordering;

use skiplist::OrderedSkipList;

use crate::orderbook::{order::LimitOrder, side::Side, transaction::Transaction};

pub struct OrderBook {
    buy_orders: OrderedSkipList<LimitOrder>,
    sell_orders: OrderedSkipList<LimitOrder>,
    completed_transactions: HashMap<String, Transaction>,
    completed_orders: Vec<LimitOrder>,
}

impl Default for OrderBook {
    fn default() -> Self {
        OrderBook::new()
    }
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            buy_orders: OrderedSkipList::new(),
            sell_orders: OrderedSkipList::new(),
            completed_transactions: HashMap::new(),
            completed_orders: Vec::new(),
        }
    }
    pub fn add_order(&mut self, mut order: LimitOrder) {
        // look for matching orders
        let fully_matcted = match_orders(self, &mut order);
        if (!fully_matcted) {
            // add order to orders if not matched
            match order.side {
                Side::Sell => self.sell_orders.insert(order),
                Side::Buy => self.buy_orders.insert(order),
            }
        }
    }
}

fn match_orders(order_book: &mut OrderBook, order: &mut LimitOrder) -> bool {
    //TODO:

    match order.side {
        Side::Sell => {
            let mut transaction = Transaction::new();

            let starting_quantity = order.quantity.load(Ordering::Relaxed);
            let mut quantity_sold = 0;

            // I assume they're ordered in most to least?
            // Also if I'm doing filtering why even use a skiplist?
            // would probably be better if I also did binary search to have match_orders be O(log n)
            let buy_order_ids = order_book
                .buy_orders
                .iter()
                .filter(|&x| x.ge(order))
                .cloned();

            for buy_order_id in buy_order_ids {
                let quantity_left = starting_quantity - quantity_sold;
                let buy_order_quantity = order_book.buy_orders..quantity.load(Ordering::Relaxed);

                if quantity_left >= buy_order_quantity {
                    quantity_sold += buy_order_quantity;

                    if let Some(actual_buy_order) = order_book.buy_orders.remove(buy_order_id) {
                        order_book.completed_orders.push(actual_buy_order);
                    } else {
                        tracing::error!("That's weird buy_orders didn't have it's own order");
                    }
                } else if quantity_left < buy_order_quantity {
                    quantity_sold += quantity_left;

                    buy_order_id
                        .quantity
                        .fetch_sub(quantity_left, Ordering::Relaxed);
                }

                if quantity_sold == starting_quantity {
                    return true;
                }
            }

            false
        }
        Side::Buy => {
            order_book.sell_orders.iter().rev();

            true //TODO: real
        }
    }
}
