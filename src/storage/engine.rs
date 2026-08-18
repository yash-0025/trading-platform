use crate::errors::TradingError;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageMetadata<'a> {
    #[serde(borrow)]
    pub filename: &'a Path,
    pub author: &'a str,
    #[serde(default)]
    pub version: u32,
    #[serde(skip)]
    pub runtime_cache: u64,
}

#[deny(unused_variables)]
pub struct StorageEngine;

impl StorageEngine {
    // The ? operator in serde_json::to_string_pretty(data)? and serde_json::from_str::<T>(&json_str)? nneds TradingError to know how to automatically convert serde_json::Error

    // flexible path borrowing using &Path
    // generic bounds T: Serialize and T: DeserializeOwned support serializing any domain struct in our platform.
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

    pub fn load_json_or_default<T: DeserializeOwned + Default>(path: &Path) -> T {
        Self::load_json::<T>(path).unwrap_or_default()
    }

    pub fn save_json_atomic<T: Serialize>(path: &Path, data: &T) -> Result<(), TradingError> {
        let tmp_path = path.with_extension("tmp");
        Self::save_json(&tmp_path, data)?;
        fs::rename(&tmp_path, path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::portfolio::{Portfolio, Position};
    use std::path::PathBuf;

    #[test]
    fn test_storage_rountrip() {
        let mut portfolio = Portfolio::new();
        portfolio.add_position("BTC".into(), 1.5, 40000.0);

        let test_path = PathBuf::from("test_portoflio.json");

        StorageEngine::save_json(&test_path, &portfolio).unwrap();

        let load_portfolio: Portfolio = StorageEngine::load_json(&test_path).unwrap();

        assert_eq!(
            load_portfolio.positions.get("BTC"),
            portfolio.positions.get("BTC")
        );
        std::fs::remove_file(&test_path).unwrap();
    }
}
