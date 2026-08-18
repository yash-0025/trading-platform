//! # Trading Platform Architecture
//! 
//! A production-grade financial trading ecosystem
//! 
//! ## Core Domain Subsystems
//! - Domain Models: [`Wallet`], [`Position`]
//! - Business Services: [`OrderManager`], [`PositionTracker`]


pub mod config;
pub mod errors;
pub mod models;
pub mod storage;
pub mod services;
pub mod cli;


pub use config::Config;
pub use errors::{TradingError, Result};
pub use storage::StorageEngine;
pub use cli::{Cli, Commands};
pub use models::{Portfolio, Position, User, UserManager, Wallet, TransactionRecord, TransactionType};
pub use services::{OrderManager, OrderId, OrderSide, OrderType, OrderStatus, Order, PositionTracker};