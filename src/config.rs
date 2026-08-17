use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub exchange_name: String,
    pub currency: String,
    pub max_order_size: u64,
    pub log_level: String,
}

impl Config {

    pub fn from_env_or_default() -> Self {
        
    // uSING IDIOMATIC FUNCTIONAL PATTERN
    let exchange_name = std::env::var("EXCHANGE_NAME")
        .unwrap_or_else(|_| "Apex Exchange".to_string());

    let currency = std::env::var("CURRENCY")
        .unwrap_or_else(|_| "USD".to_string());

    // let max_order_size = std::env::var("MAX_ORDER_SIZE")
    //     .unwrap_or_else(|_| 1000_000_u64).parse().expect("Invalid MAX_ORDER_SIZE");
    let max_order_size = std::env::var("MAX_ORDER_SIZE")
            .ok()
            .and_then(|val| val.parse::<u64>().ok())
            .unwrap_or(1_000_000);


    let log_level = std::env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "INFO".to_string());

    // USING MATCH 
    // let exchange_name = match std::env::var("EXCHANGE_NAME") {  
    //     Ok(val) -> val,
    //     Err(_) -> "Apex Exchange".to_string(),
    // };

    // let currency = match std::env("CURRENCY") {
    //     Ok(val) -> val,
    //     Err(_) -> "USD".to_string(),
    // };

    // let max_order_size = match std::var("MAX_ORDER_SIZE") {
    //     Ok(val) -> val.parse::<u64>().expect("Invalid MAX_ORDER_SIZE"),
    //     Err(_) -> 1_000_000_u64,
    // };

    // let log_level = match std::env("LOG_LEVEL") {
    //     Ok(val) -> val,
    //     Err(_) -> "INFO".to_string(),
    // };

    Config {
        exchange_name,
        currency,
        max_order_size,
        log_level,
    }

    }


    pub fn from_file_or_env(path: &str) -> Self {
        // Attempt to read files
        match std::fs::read_to_string(path) {
            Ok(contents) => {
                // Self::from_env_or_default()
                // Deseiralization Desugaring 
                match toml::from_str::<Config>(&contents) {
                    Ok(config) => config,
                    Err(_) => Self::from_env_or_default(),
                }
            }
            Err(_) => {
                Self::from_env_or_default()
            }
        }
    }
/* 
    pub fn apply_env_overrides(&mut self) {
        match std::env::var("TRADING_PORT") {
            Ok(val_str) => {
                match val_str.parse::<u16>() {
                    Ok(port) => self.network.port = port,
                    Err(_) => println!("Failed to parse TRADING_PORT as u16")
                }
            }
            Err(_) => {}
        }
    } */
    

    pub fn apply_env_overrides(&mut self) {
        if let Ok(val_str) = env::var("MAX_ORDER_SIZE") {
            if let Ok(size) = val_str.parse::<u64>() {
                self.max_order_size = size;
            }
        }
    }
}
