use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use crate::orderbook::side::Side;

pub struct LimitOrder {
    pub order_id: u64,
    pub side: Side,
    pub price: u128,
    pub quantity: AtomicU64,
}
impl LimitOrder {
    pub fn reduce_quantity(&mut self, amount: u64) {
        self.quantity.fetch_sub(amount, Ordering::Relaxed);
    }
}

impl Clone for LimitOrder {
    fn clone(&self) -> Self {
        Self {
            order_id: self.order_id,
            side: self.side.clone(),
            price: self.price,
            quantity: AtomicU64::new(self.quantity.load(Ordering::Relaxed)),
        }
    }
}

impl Eq for LimitOrder {}
impl PartialEq for LimitOrder {
    fn eq(&self, other: &Self) -> bool {
        self.order_id == other.order_id
            && self.side == other.side
            && self.price == other.price
            && self.quantity.load(Ordering::Relaxed) == other.quantity.load(Ordering::Relaxed)
    }
}

impl Ord for LimitOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price.cmp(&other.price)
    }
}
impl PartialOrd for LimitOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
