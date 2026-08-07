#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
}

#[derive(Debug, Clone, ParitalEq)]
pub struct Order {
    pub id: u64,
    pub asset: String,
    pub side: Side,
    pub order_type: OrderType,
    pub qty: Quantity,
    pub status: OrderStatus,
}


#[derive(Debug, PartialEq, Clone,  Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Market,
    Limit { price: i64},
    StopLoss { trigger_price: i64},
}