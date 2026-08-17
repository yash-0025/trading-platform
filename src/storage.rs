use std::fs;
use std::path::Path;
use serde::{Serialize,  de::DeserializeOwned};
use crate::errors::TradingError;

pub struct StorageEngine;

impl StorageEngine {

// The ? operator in serde_json::to_string_pretty(data)? and serde_json::from_str::<T>(&json_str)? nneds TradingError to know how to automatically convert serde_json::Error

// flexible path borrowing using &Path
// generic bounds T: Serialize and T: DeserializeOwned
    pub fn save_json<T: Serialize>(path: &Path, data: &T) -> Result<(), TradingError> {
        // Serialize data to Json string
        let json_str = serde_json::to_string_pretty(data)?;
        // write string to the disk
        fs::write(path, json_str)?;
        Ok(())
    }

    pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T, TradingError> {
        // Read json file from disk
        let json_str = fs::read_to_string(path)?;
        // Deserialize json string into struct T
        let data = serde_json::from_str::<T>(&json_str)?;
        Ok(data)
    }
}

