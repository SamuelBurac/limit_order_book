pub struct Transaction {
    pub transaction_id: u64,
    pub buy_orders: Vec<u64>,
    pub sell_orders: Vec<u64>,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            transaction_id: rand::random(),
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
        }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction::new()
    }
}
