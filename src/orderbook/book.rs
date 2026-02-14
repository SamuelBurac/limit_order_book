// This will hold all the orders and execute trades?

use std::collections::HashMap;

use skiplist::OrderedSkipList;

use crate::orderbook::{order::LimitOrder, side::Side, transaction::Transaction};

pub struct OrderBook {
    buy_orders: OrderedSkipList<LimitOrder>,
    sell_orders: OrderedSkipList<LimitOrder>,
    completed_transactions: HashMap<u64, Transaction>,
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
    pub fn add_order(&mut self, order: LimitOrder) {
        // look for matching orders
        match_orders(self, order);
    }
}

fn match_orders(order_book: &mut OrderBook, order: LimitOrder) {
    match order.side {
        Side::Sell => {
            let mut transaction = Transaction::new();
            transaction.sell_order_ids.push(order.order_id);

            let starting_quantity = order.quantity;
            let mut quantity_sold = 0;

            // partially fulfilled order
            let mut partially_filled_order_index: Option<usize> = None;
            let mut partial_quantity_fulfilled: u64 = 0;

            let mut order_idxs_to_remove: Vec<usize> = Vec::new();

            // I assume they're ordered in most to least?
            // Also if I'm doing filtering why even use a skiplist?
            // would probably be better if I also did binary search to have match_orders be O(log n)
            let buy_orders = order_book
                .buy_orders
                .iter()
                .enumerate()
                .filter(|(_, buy)| buy.price > order.price);

            // Find the orders that are matching
            for (idx, buy_order) in buy_orders {
                let quantity_left = starting_quantity - quantity_sold;
                let buy_order_quantity = buy_order.quantity;

                // Fullfilling the whole buy order
                if quantity_left >= buy_order_quantity {
                    quantity_sold += buy_order_quantity;

                    order_idxs_to_remove.push(idx);

                    // Fulfilling it partially
                    // one sell order can only partially fill one buy order
                    // otherwise it would've fulfilled the whole order.
                } else if quantity_left < buy_order_quantity {
                    quantity_sold += quantity_left;
                    partial_quantity_fulfilled = quantity_left;
                    partially_filled_order_index = Some(idx);
                }
            }

            // update the partially filled order
            if let Some(order_idx) = partially_filled_order_index {
                // Get order
                let mut order = order_book.buy_orders.remove_index(order_idx);

                // add to transaction
                transaction.buy_order_ids.push(order.order_id);

                //update order
                order.quantity -= partial_quantity_fulfilled;

                // Add back to orders
                order_book.buy_orders.insert(order);
            }

            // fulfill the order
            for order_idx in order_idxs_to_remove {
                let completed_order = order_book.buy_orders.remove_index(order_idx);
                transaction.buy_order_ids.push(completed_order.order_id);

                order_book.completed_orders.push(completed_order);
            }

            if !transaction.buy_order_ids.is_empty() {
                order_book
                    .completed_transactions
                    .insert(transaction.transaction_id, transaction);
            }

            if starting_quantity == quantity_sold {
                // completed this sell order fully
                order_book.completed_orders.push(order);
            } else {
                order_book.sell_orders.insert(order);
            }
        }
        Side::Buy => {
            let mut transaction = Transaction::new();
            transaction.buy_order_ids.push(order.order_id);

            let starting_quantity = order.quantity;
            let mut quantity_sold = 0;

            // partially fulfilled order
            let mut partially_filled_order_index: Option<usize> = None;
            let mut partial_quantity_fulfilled: u64 = 0;

            let mut order_idxs_to_remove: Vec<usize> = Vec::new();

            // I assume they're ordered in most to least?
            // Also if I'm doing filtering why even use a skiplist?
            // would probably be better if I also did binary search to have match_orders be O(log n)
            let buy_orders = order_book
                .buy_orders
                .iter()
                .enumerate()
                .filter(|(_, buy)| buy.price > order.price);
        }
    }
}
