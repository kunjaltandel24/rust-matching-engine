use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub id: u64,
    pub price: f64,
    pub quantity: u64,
    pub order_type: OrderType,
    pub timestamp: u64,
}

// Define an enum for order type (buy or sell)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

// Define ordering for the Order struct based on price-time priority
impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.price.partial_cmp(&other.price) {
            Some(Ordering::Equal) => self.timestamp.cmp(&other.timestamp),
            Some(x) => x,
            None => Ordering::Equal, // Handle NaN case
        }
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Order {}
