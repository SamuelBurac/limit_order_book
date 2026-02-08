pub enum Side {
    Sell,
    Buy,
}

impl Clone for Side {
    fn clone(&self) -> Self {
        match self {
            Self::Sell => Self::Sell,
            Self::Buy => Self::Buy,
        }
    }
}
impl PartialEq for Side {
    fn eq(&self, other: &Side) -> bool {
        matches!(
            (self, other),
            (Side::Buy, Side::Buy) | (Side::Sell, Side::Sell)
        )
    }
}
