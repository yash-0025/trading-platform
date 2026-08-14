use std::collections::HashMap;
use crate::errors::{TradingError, Result};

#[derive(Debug, Default)]
pub struct Wallet {
    pub balances: HashMap<String, u64>
}

impl Wallet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn deposit(&mut self, currency: String, amount: u64) -> Result<()> {
        *self.balances.entry(currency).or_insert(0) += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, currency: &str, amount: u64) -> Result<()> {
        let bal = self.balances.get_mut(currency);

        match bal {
            // Currency exists and has enough money 
            Some(current_bal) if *current_bal >= amount => {
                *current_bal -= amount;
                Ok(())
            }
            // Currency exists but does not have enough money
            Some(current_bal) => 
            Err(TradingError::InsufficientFunds {
                    required: amount,
                    available: *current_bal,
                }),
            // Currency Does not exist in the map at all
            None => Err(TradingError::InsufficientFunds {
                    required: amount,
                    available: 0,
                }),
        }
    }

    pub fn get_balance(&self, currency: &str) -> u64 {
        // .copied() turn Option<&u64> into Option<u64> so unwrap directly with integer 0 so we didn't need &0
        self.balances.get(currency).copied().unwrap_or(0)
    }
}