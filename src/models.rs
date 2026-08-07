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