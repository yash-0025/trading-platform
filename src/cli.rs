use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "trading-platform", author, version, about = "CLI Trading Terminal")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Buy {
        symbol: String,
        qty: u64,
        price: i64,
    },
    Sell {
        symbol: String,
        qty: u64,
        price: i64,
    },
    Balance,
    Orders,
}