use crate::orderbook::side::Side;

pub struct LimitOrder {
    pub order_id: u64,
    pub side: Side,
    pub price: u128,
    pub quantity: u64,
}
impl LimitOrder {}

impl Clone for LimitOrder {
    fn clone(&self) -> Self {
        Self {
            order_id: self.order_id,
            side: self.side.clone(),
            price: self.price,
            quantity: self.quantity,
        }
    }
}

impl Eq for LimitOrder {}
impl PartialEq for LimitOrder {
    fn eq(&self, other: &Self) -> bool {
        self.order_id == other.order_id
            && self.side == other.side
            && self.price == other.price
            && self.quantity == other.quantity
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
