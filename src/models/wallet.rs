use crate::errors::{Result, TradingError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRecord {
    pub tx_type: TransactionType,
    pub currency: String,
    pub amount: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Wallet {
    pub balances: HashMap<String, u64>,
    pub history: Vec<TransactionRecord>,
}

impl Wallet {
    pub fn new() -> Self {
        Self::default()
    }

    /// # Example
    /// ```
    /// use trading_platform::wallet::Wallet;
    /// let mut wallet = Wallet::new();
    /// wallet.deposit("USD".to_string(), 500);
    /// assert_eq!(wallet.get_balance("USD"), 500);
    /// ```
    pub fn deposit(&mut self, currency: String, amount: u64) -> Result<()> {
        *self.balances.entry(currency.clone()).or_insert(0) += amount;
        self.history.push(TransactionRecord {
            tx_type: TransactionType::Deposit,
            currency,
            amount,
            timestamp: Utc::now(),
        });
        Ok(())
    }

    pub fn withdraw(&mut self, currency: &str, amount: u64) -> Result<()> {
        let bal = self.balances.get_mut(currency);

        match bal {
            // Currency exists and has enough money
            Some(current_bal) if *current_bal >= amount => {
                *current_bal -= amount;
                self.history.push(TransactionRecord {
                    tx_type: TransactionType::Withdrawal,
                    currency: currency.to_string(),
                    amount,
                    timestamp: Utc::now(),
                });
                Ok(())
            }
            // Currency exists but does not have enough money
            Some(current_bal) => Err(TradingError::InsufficientFunds {
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

    pub fn get_history(&self, currency: &str) -> Vec<TransactionRecord> {
        self.history
            .iter()
            .filter(|tx| tx.currency == currency)
            .cloned()
            .collect()
    }

    pub fn total_balance(&self) -> u64 {
        self.balances.values().sum::<u64>()
    }

    pub fn filter_transactions<F>(&self, predicate: F) -> Vec<TransactionRecord>
    where
        F: Fn(&TransactionRecord) -> bool,
    {
        self.history
            .iter()
            .filter(|rec| predicate(rec))
            .cloned()
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "InsufficientFunds")]
    fn test_withdraw_insufficient_funds_panic() {
        let mut wallet = Wallet::new();
        wallet.deposit("USD".to_string(), 100);
        wallet.withdraw("USD", 500).unwrap();
    }
}
