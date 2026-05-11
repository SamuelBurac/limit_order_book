// This will hold all the orders and execute trades?

use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    ops::Bound::{Included, Unbounded},
};

use skiplist::SkipMap;

use crate::orderbook::{
    order::LimitOrder, price_level::PriceLevel, side::Side, transaction::Transaction,
};

pub struct OrderBook {
    buy_orders: SkipMap<u64, PriceLevel>,
    sell_orders: SkipMap<u64, PriceLevel>,
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
                writeln!(&mut file, "|  Price level | orders |").unwrap();
                writeln!(&mut file, "|--------|----------|").unwrap();
                for order in self.buy_orders.values() {
                    writeln!(
                        &mut file,
                        "|  ${:.2} | {:?} |",
                        order.price as f64 / 100.0,
                        order.orders
                    )
                    .unwrap();
                }
            }

            // Write active sell orders
            writeln!(&mut file, "\n## Active Sell Orders\n").unwrap();
            if self.sell_orders.is_empty() {
                writeln!(&mut file, "*No active sell orders*").unwrap();
            } else {
                writeln!(&mut file, "|  Price level | orders |").unwrap();
                writeln!(&mut file, "|-------|----------|").unwrap();
                for order in self.sell_orders.values() {
                    writeln!(
                        &mut file,
                        "| {:.2} | {:?} |",
                        order.price as f64 / 100.0,
                        order.orders
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
                        order.quantity.borrow()
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

            let starting_quantity = *order.quantity.borrow();
            let mut quantity_sold = 0;
            let mut quantity_left = starting_quantity;

            let mut price_levels_to_remove: Vec<u64> = Vec::new();

            // Find the orders that are matching
            for (_, buy_level) in order_book
                .buy_orders
                .range(Included(&order.price), Unbounded)
            {
                let mut by_order = buy_level.orders.borrow_mut().pop_front();
                while by_order.is_some() {
                    if let Some(mut buy_order) = by_order {
                        if quantity_left == 0 {
                            break;
                        }

                        let buy_order_quantity = *buy_order.quantity.borrow();

                        if quantity_left >= buy_order_quantity {
                            // Fullfilling the whole buy order
                            quantity_sold += buy_order_quantity;
                            quantity_left -= buy_order_quantity;

                            // fulfill order
                            transaction.buy_order_ids.push(buy_order.order_id);
                            order_book.completed_orders.push(buy_order);
                        } else if quantity_left < buy_order_quantity {
                            // Fulfilling it partially
                            // one sell order can only partially fill one buy order
                            // otherwise it would've fulfilled the whole order.
                            quantity_sold += quantity_left;

                            // add to transaction
                            transaction.buy_order_ids.push(buy_order.order_id);

                            //update order to remove the remaining quantities to round
                            //out the order
                            *buy_order.quantity.get_mut() -= quantity_left;

                            // since the full buy order wasn't completed
                            // put back to the front
                            buy_level.orders.borrow_mut().push_front(buy_order);

                            quantity_left = 0;
                        }
                    }
                    by_order = buy_level.orders.borrow_mut().pop_front();
                }

                if buy_level.orders.borrow().len() == 0 {
                    price_levels_to_remove.push(buy_level.price);
                }
                if quantity_left == 0 {
                    break;
                }
            }

            // remove the price levels that have been exhausted
            for price in price_levels_to_remove {
                order_book.buy_orders.remove(&price);
            }

            // if the sell order completed add it to the compelted transactions
            if !transaction.buy_order_ids.is_empty() {
                order_book
                    .completed_transactions
                    .insert(transaction.transaction_id, transaction);
            }

            if starting_quantity == quantity_sold {
                // completed this sell order fully
                order_book.completed_orders.push(order);
            } else {
                // if the order hasn't been fulfilled then we add it to the book
                // first look for a price level if it doesnt exist create one
                if let Some(level) = order_book.sell_orders.get_mut(&order.price) {
                    level.orders.borrow_mut().push_back(order);
                } else {
                    let pl = PriceLevel::new(order.price);
                    pl.orders.borrow_mut().push_back(order);
                    order_book.sell_orders.insert(pl.price, pl);
                }
            }
        }
        Side::Buy => {
            let mut transaction = Transaction::new();
            transaction.buy_order_ids.push(order.order_id);

            let starting_quantity = *order.quantity.borrow();
            let mut quantity_sold = 0;
            let mut quantity_left = starting_quantity;

            let mut price_levels_to_remove: Vec<u64> = Vec::new();

            let sell_levels = order_book
                .sell_orders
                .range(Unbounded, Included(&order.price));

            for (_, sell_level) in sell_levels {
                let mut s_order = sell_level.orders.borrow_mut().pop_front();
                while s_order.is_some() {
                    if let Some(sell_order) = s_order {
                        if quantity_left == 0 {
                            break;
                        }
                        let buy_order_quantity = *sell_order.quantity.borrow();

                        // Fullfilling the whole buy order
                        if quantity_left >= buy_order_quantity {
                            quantity_sold += buy_order_quantity;
                            quantity_left -= buy_order_quantity;

                            transaction.buy_order_ids.push(sell_order.order_id);
                            order_book.completed_orders.push(sell_order);
                        } else if quantity_left < buy_order_quantity {
                            quantity_sold += quantity_left;

                            // add to transaction
                            transaction.sell_order_ids.push(order.order_id);

                            //update order
                            *sell_order.quantity.borrow_mut() -= quantity_left;

                            // need to add the order backt to the level
                            sell_level.orders.borrow_mut().push_front(sell_order);

                            // A sell order can only be fulfilled partially
                            // if if fulfills the whole buy order quantity
                            quantity_left = 0;
                        }
                    }
                    s_order = sell_level.orders.borrow_mut().pop_front();
                }

                // if the whole level has been exhausted remove it
                // from the book
                if sell_level.orders.borrow().len() == 0 {
                    price_levels_to_remove.push(sell_level.price);
                }

                if quantity_left == 0 {
                    break;
                }
            }

            // remove the price levels that have been exhausted
            for price in price_levels_to_remove {
                order_book.buy_orders.remove(&price);
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
                // if the order hasn't been fulfilled then we add it to the book
                // first look for a price level if it doesnt exist create one
                if let Some(level) = order_book.buy_orders.get_mut(&order.price) {
                    level.orders.borrow_mut().push_back(order);
                } else {
                    let pl = PriceLevel::new(order.price);
                    pl.orders.borrow_mut().push_back(order);
                    order_book.buy_orders.insert(pl.price, pl);
                }
            }
        }
    }
}
