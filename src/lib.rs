#![warn(missing_docs)]
//! # Trading Platform Architecture
//!
//! A production-grade financial trading ecosystem
//!
//! ## Core Domain Subsystems
//! - Domain Models: [`Wallet`], [`Position`]
//! - Business Services: [`OrderManager`], [`PositionTracker`]

pub mod cli;
pub mod config;
pub mod errors;
pub mod models;
pub mod services;
pub mod storage;

pub use cli::{Cli, Commands};
pub use config::Config;
pub use errors::{Result, TradingError};
pub use models::{
    Portfolio, Position, TransactionRecord, TransactionType, User, UserManager, Wallet,
};
pub use services::{
    Order, OrderId, OrderManager, OrderSide, OrderStatus, OrderType, PositionTracker,
};
pub use storage::StorageEngine;
