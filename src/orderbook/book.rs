// This will hold all the orders and execute trades?

use std::{collections::HashMap, fs::File, io::Write};

use skiplist::SkipMap;

use crate::orderbook::{order::LimitOrder, side::Side, transaction::Transaction};

pub struct OrderBook {
    buy_orders: SkipMap<u64, LimitOrder>,
    sell_orders: SkipMap<u64, LimitOrder>,
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
            buy_orders: SkipMap::new(),
            sell_orders: SkipMap::new(),
            completed_transactions: HashMap::new(),
            completed_orders: Vec::new(),
        }
    }
    pub fn add_order(&mut self, order: LimitOrder) {
        // look for matching orders
        match_orders(self, order);
    }

    pub fn write_state_to_file(&mut self, file_name: &str) {
        let res = File::create(file_name);
        if let Ok(mut file) = res {
            // Write header
            writeln!(&mut file, "# Order Book State Report\n").unwrap();
            writeln!(
                &mut file,
                "Generated at: {:?}\n",
                std::time::SystemTime::now()
            )
            .unwrap();

            // Write summary statistics
            writeln!(&mut file, "## Summary\n").unwrap();
            writeln!(
                &mut file,
                "- **Active Buy Orders**: {}",
                self.buy_orders.len()
            )
            .unwrap();
            writeln!(
                &mut file,
                "- **Active Sell Orders**: {}",
                self.sell_orders.len()
            )
            .unwrap();
            writeln!(
                &mut file,
                "- **Completed Orders**: {}",
                self.completed_orders.len()
            )
            .unwrap();
            writeln!(
                &mut file,
                "- **Completed Transactions**: {}\n",
                self.completed_transactions.len()
            )
            .unwrap();

            // Write active buy orders
            writeln!(&mut file, "## Active Buy Orders\n").unwrap();
            if self.buy_orders.is_empty() {
                writeln!(&mut file, "*No active buy orders*").unwrap();
            } else {
                writeln!(&mut file, "| Order ID | Side | Price | Quantity |").unwrap();
                writeln!(&mut file, "|----------|------|-------|----------|").unwrap();
                for order in self.buy_orders.values() {
                    writeln!(
                        &mut file,
                        "| {} | {:?} | ${:.2} | {} |",
                        order.order_id,
                        order.side,
                        order.price as f64 / 100.0,
                        order.quantity
                    )
                    .unwrap();
                }
            }

            // Write active sell orders
            writeln!(&mut file, "\n## Active Sell Orders\n").unwrap();
            if self.sell_orders.is_empty() {
                writeln!(&mut file, "*No active sell orders*").unwrap();
            } else {
                writeln!(&mut file, "| Order ID | Side | Price | Quantity |").unwrap();
                writeln!(&mut file, "|----------|------|-------|----------|").unwrap();
                for order in self.sell_orders.values() {
                    writeln!(
                        &mut file,
                        "| {} | {:?} | ${:.2} | {} |",
                        order.order_id,
                        order.side,
                        order.price as f64 / 100.0,
                        order.quantity
                    )
                    .unwrap();
                }
            }

            // Write completed orders
            writeln!(&mut file, "\n## Completed Orders\n").unwrap();
            if self.completed_orders.is_empty() {
                writeln!(&mut file, "*No completed orders*").unwrap();
            } else {
                writeln!(&mut file, "| Order ID | Side | Price | Quantity |").unwrap();
                writeln!(&mut file, "|----------|------|-------|----------|").unwrap();
                for order in &self.completed_orders {
                    writeln!(
                        &mut file,
                        "| {} | {:?} | ${:.2} | {} |",
                        order.order_id,
                        order.side,
                        order.price as f64 / 100.0,
                        order.quantity
                    )
                    .unwrap();
                }
            }

            // Write completed transactions
            writeln!(&mut file, "\n## Completed Transactions\n").unwrap();
            if self.completed_transactions.is_empty() {
                writeln!(&mut file, "*No completed transactions*").unwrap();
            } else {
                writeln!(
                    &mut file,
                    "| Transaction ID | Buy Order IDs | Sell Order IDs |"
                )
                .unwrap();
                writeln!(
                    &mut file,
                    "|----------------|---------------|----------------|"
                )
                .unwrap();

                for transaction in self.completed_transactions.values() {
                    let buy_ids = transaction
                        .buy_order_ids
                        .iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    let sell_ids = transaction
                        .sell_order_ids
                        .iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    writeln!(
                        &mut file,
                        "| {} | {} | {} |",
                        transaction.transaction_id, buy_ids, sell_ids
                    )
                    .unwrap();
                }
            }

            tracing::info!("Successfully wrote order book state to {}", file_name);
        } else {
            tracing::error!("Failed to create file {}", file_name);
        }
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
            let mut partially_filled_order_id: Option<u64> = None;
            let mut partial_quantity_fulfilled: u64 = 0;

            let mut order_ids_to_remove: Vec<u64> = Vec::new();

            // I assume they're ordered in most to least?
            // Also if I'm doing filtering why even use a skiplist?
            // would probably be better if I also did binary search to have match_orders be O(log n)
            let buy_orders = order_book
                .buy_orders
                .values()
                .filter(|buy| buy.price > order.price);

            // Find the orders that are matching
            for buy_order in buy_orders {
                let quantity_left = starting_quantity - quantity_sold;
                if quantity_left == 0 {
                    break;
                }

                let buy_order_quantity = buy_order.quantity;

                // Fullfilling the whole buy order
                if quantity_left >= buy_order_quantity {
                    quantity_sold += buy_order_quantity;

                    order_ids_to_remove.push(buy_order.order_id);

                    // Fulfilling it partially
                    // one sell order can only partially fill one buy order
                    // otherwise it would've fulfilled the whole order.
                } else if quantity_left < buy_order_quantity {
                    quantity_sold += quantity_left;
                    partial_quantity_fulfilled = quantity_left;
                    partially_filled_order_id = Some(buy_order.order_id);
                }
            }

            // update the partially filled order
            if let Some(order_id) = partially_filled_order_id {
                // Get order
                if let Some(mut order) = order_book.buy_orders.remove(&order_id) {
                    // add to transaction
                    transaction.buy_order_ids.push(order.order_id);
                    //update order
                    order.quantity -= partial_quantity_fulfilled;

                    // Add back to orders
                    order_book.buy_orders.insert(order.order_id, order);
                } else {
                    tracing::error!("Failed to remove order");
                }
            }

            // fulfill the order
            for order_id in order_ids_to_remove {
                if let Some(completed_order) = order_book.buy_orders.remove(&order_id) {
                    transaction.buy_order_ids.push(completed_order.order_id);
                    order_book.completed_orders.push(completed_order);
                } else {
                    tracing::error!("Failed to remove order");
                }
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
                order_book.sell_orders.insert(order.order_id, order);
            }
        }
        Side::Buy => {
            let mut transaction = Transaction::new();
            transaction.buy_order_ids.push(order.order_id);

            let starting_quantity = order.quantity;
            let mut quantity_sold = 0;

            // partially fulfilled order
            let mut partially_filled_order_id: Option<u64> = None;
            let mut partial_quantity_fulfilled: u64 = 0;

            let mut order_ids_to_remove: Vec<u64> = Vec::new();

            // I assume they're ordered in most to least?
            // Also if I'm doing filtering why even use a skiplist?
            // would probably be better if I also did binary search to have match_orders be O(log n)
            let sell_orders = order_book
                .sell_orders
                .values()
                .rev()
                .filter(|sell| sell.price < order.price);

            for sell_order in sell_orders {
                let quantity_left = starting_quantity - quantity_sold;
                if quantity_left == 0 {
                    break;
                }
                let buy_order_quantity = sell_order.quantity;

                // Fullfilling the whole buy order
                if quantity_left >= buy_order_quantity {
                    quantity_sold += buy_order_quantity;

                    order_ids_to_remove.push(sell_order.order_id);
                } else if quantity_left < buy_order_quantity {
                    quantity_sold += quantity_left;
                    partial_quantity_fulfilled = quantity_left;
                    partially_filled_order_id = Some(sell_order.order_id);
                }
            }

            // update the partially filled order
            if let Some(order_id) = partially_filled_order_id {
                // Get order
                if let Some(mut order) = order_book.sell_orders.remove(&order_id) {
                    // add to transaction
                    transaction.sell_order_ids.push(order.order_id);

                    //update order
                    order.quantity -= partial_quantity_fulfilled;

                    // Add back to orders
                    order_book.sell_orders.insert(order.order_id, order);
                }
            }

            // fulfill the order
            for order_id in order_ids_to_remove {
                if let Some(completed_order) = order_book.buy_orders.remove(&order_id) {
                    transaction.buy_order_ids.push(completed_order.order_id);

                    order_book.completed_orders.push(completed_order);
                }
            }

            // if there was no sell orders fulfilled don't record the transaction
            if !transaction.sell_order_ids.is_empty() {
                order_book
                    .completed_transactions
                    .insert(transaction.transaction_id, transaction);
            }

            if starting_quantity == quantity_sold {
                // completed this sell order fully
                order_book.completed_orders.push(order);
            } else {
                order_book.buy_orders.insert(order.order_id, order);
            }
        }
    }
}
