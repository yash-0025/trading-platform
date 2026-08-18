use thiserror::Error;

#[derive(Error, Debug)]
pub enum TradingError {
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },

    #[error("Order not found with ID {order_id}")]
    OrderNotFound { order_id: u64 },

    #[error("Invalid order Quantity: {message}")]
    InvalidQuantity { message: String },

    #[error("Invalid order: {message}")]
    InvalidOrder { message: String },

    #[error("I/O error: {0} ")]
    Io(#[from] std::io::Error),

    #[error("Serde JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Config parse error: [0]")]
    ConfigParse(#[from] toml::de::Error),
}

// Crate level Result<T> type alias
pub type Result<T> = std::result::Result<T, TradingError>;
