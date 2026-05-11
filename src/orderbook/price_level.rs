use crate::orderbook::order::LimitOrder;
use std::cell::RefCell;

use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct PriceLevel {
    pub price: u64,
    pub orders: RefCell<VecDeque<LimitOrder>>,
}

impl PriceLevel {
    pub fn new(price: u64) -> PriceLevel {
        PriceLevel {
            price,
            orders: RefCell::new(VecDeque::new()),
        }
    }
}
