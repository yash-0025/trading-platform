use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub symbol: String,
    pub side: OrderSide,
    pub qty: u64,
    pub price: u64,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}

impl Order {
    pub fn new(id: u64, symbol: String, side: OrderSide, qty: u64, price: u64) -> Self {
        Order {
            id: OrderId(id),
            symbol,
            side,
            qty,
            price,
            status: OrderStatus::Pending,
            created_at: Utc::now(),
        }
    }

    pub fn cancel(&mut self) -> bool {
        if self.status == OrderStatus::Pending {
            self.status = OrderStatus::Cancelled;
            true
        } else {
            false
        }
    }
}