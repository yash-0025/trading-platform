use clap::Parser;

mod models;
mod config;
mod cli;
mod errors;
mod users;

use cli::{Cli, Commands};


fn main() {
    println!("=======================================");
    println!("      TRADING PLATFORM ECOSYTEM        ");
    println!("=======================================");
    println!("System status: INITIALIZED");


    let cli = Cli::parse();

    match &cli.command {
        Commands::Buy { symbol, qty, price } => {
            println!("[ORDER SUBMITTED] BUY {} shares of {} at ${}", qty, symbol, price);
        }
        Commands::Sell { symbol, qty, price } => {
            println!("[ORDER SUBMITTED] SELL {} shares of {} at ${}", qty, symbol, price);
        }
        Commands::Balance => {
            println!("[ACCOUNT BALANCE] $100,000.00 USD");
        }
        Commands::Orders => {
            println!("[OPEN ORDERS] No open orders.");
        }
    }
}



