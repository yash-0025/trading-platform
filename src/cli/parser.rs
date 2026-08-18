use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "trading-platform",
    author,
    version,
    about = "CLI Trading Terminal"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/*
Enable --symbol, --qty, --price Flags
If you want named flags like --symbol BTC --qty 2 --price 50000, add #[arg(long)] above each field
#[derive(Subcommand, Debug)]
pub enum Commands {
    Buy {
        #[arg(long)]
        symbol: String,
        #[arg(long)]
        qty: u64,
        #[arg(long)]
        price: i64,
    },
    // ...
}
 */
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
