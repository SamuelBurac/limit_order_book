pub struct Transaction {
    pub transaction_id: u64,
    pub buy_order_ids: Vec<u64>,
    pub sell_order_ids: Vec<u64>,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            transaction_id: rand::random(),
            buy_order_ids: Vec::new(),
            sell_order_ids: Vec::new(),
        }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction::new()
    }
}
