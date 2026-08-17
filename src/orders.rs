use chrono::{DateTime, Utc};
use crate::errors::TradingError;
use serde::{Serialize, Deserialize};


#[derive(Debug, Default)]
pub struct OrderBuilder {
    pub symbol: Option<String>,
    pub side: Option<OrderSide>,
    pub qty: Option<u64>,
    pub price: Option<u64>,
}

impl OrderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn side(mut self, side: OrderSide)-> Self {
        self.side = Some(side);
        self
    }

    pub fn qty(mut self, qty: u64) -> Self {
        self.qty = Some(qty);
        self
    }

    pub fn price(mut self, price: u64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn build(self, id: u64) -> Result<Order, TradingError> {

        let symbol = self.symbol.ok_or_else(|| TradingError::InvalidOrder{ message : "Missing symbol".into()})?;
        if symbol.is_empty() {
            return Err(TradingError::InvalidOrder{ message : "Missing symbol".into(),
        });
        }

        let side = self.side.ok_or_else(|| TradingError::InvalidOrder{message:"Missing order side".into()})?;

        let qty = self.qty.ok_or_else(|| TradingError::InvalidOrder{message : "Quantity must be greater than zero".into()})?;
        if qty == 0 {
            return Err(TradingError::InvalidOrder{ message : "Quantity must be greater than zero".into()});
        }
        
        let price = self.price.ok_or_else(|| TradingError::InvalidOrder{message : "Price must be greater than zero".into()})?;
        if price == 0 {
            return Err(TradingError::InvalidOrder{message : "Price must be greater than zero".into()});
        }
        Ok(Order::new(id,symbol, side, qty, price))

    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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