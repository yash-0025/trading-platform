use thiserror::Error;

#[derive(Error, Debug)]
pub enum TradingError {

    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64},

    #[error("Order not found with ID {order_id}")]
    OrderNotFound { order_id: u64},

    #[error("Invalid order Quantity: {message}")]
    InvalidQuantity { message: String},

}