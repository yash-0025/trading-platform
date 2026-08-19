use crate::models::portfolio::Position;
use crate::services::order_manager::OrderSide;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Default)]
pub struct PositionTracker {
    pub positions: HashMap<String, Position>,
    pub realized_pnl: f64,
}

impl PositionTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_fill(&mut self, side: OrderSide, symbol: &str, qty: f64, price: f64) {
        match side {
            OrderSide::Buy => {
                self.positions
                    .entry(symbol.to_string())
                    .and_modify(|pos| pos.update(qty, price))
                    .or_insert_with(|| Position::new(symbol.to_string(), qty, price));
            }

            OrderSide::Sell => {
                if let Some(pos) = self.positions.get_mut(symbol) {
                    let pnl = (price - pos.avg_cost) * qty;

                    self.realized_pnl += pnl;
                    pos.quantity -= qty;

                    if pos.quantity <= 0.0 {
                        self.positions.remove(symbol);
                    }
                }
            }
        }
    }

    pub fn total_pnl(&self, current_prices: &HashMap<String, f64>) -> f64 {
        let mut total = self.realized_pnl;

        for pos in self.positions.values() {
            let market_price = current_prices
                .get(&pos.symbol)
                .copied()
                .unwrap_or(pos.avg_cost);
            total += pos.unrealized_pnl(market_price);
        }
        total
    }

    /// Resets all open positions and realized P&L
    #[allow(dead_code)]
    pub fn clear_positions(&mut self) {
        self.positions.clear();
        self.realized_pnl = 0.0;
    }
}

pub fn benchmark_operation<F, R>(name: &str, op: F) ->(R, u128) where F: FnOnce() -> R , {
    let start = Instant::now();
    let result = op();
    let micros = start.elapsed().as_micros();
    println!("[BENCHMARK] {} executed in {} µs", name, micros);
    (result, micros)
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::order_manager::OrderSide;

    #[test]
    fn test_position_tracker_buy_sell_pnl() {
        let mut tracker = PositionTracker::new();

        // Buy 2.0 BTC @ $40,000
        tracker.process_fill(OrderSide::Buy, "BTC", 2.0, 40000.0);
        assert_eq!(tracker.positions.get("BTC").unwrap().quantity, 2.0);

        // Sell 1.0 BTC @ $50,000 (realized 10k)
        tracker.process_fill(OrderSide::Sell, "BTC", 1.0, 50000.0);
        assert_eq!(tracker.realized_pnl, 10000.0);
        assert_eq!(tracker.positions.get("BTC").unwrap().quantity, 1.0);

        // Verify total pnl at market price 55,000
        let prices = HashMap::from([("BTC".to_string(), 55000.0)]);
        assert_eq!(tracker.total_pnl(&prices), 25000.0);
    }
}
