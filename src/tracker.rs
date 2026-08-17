use std::collections::HashMap;
use crate::orders::OrderSide;
use crate::portfolio::Position;


#[derive(Debug, Default)]
pub struct PositionTracker {
    pub positions: HashMap<String, Position>,
    pub realized_pnl: f64,
}

impl PositionTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_fill(&mut self, side: OrderSide, symbol: &str, qty: f64, price:f64) {

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
            let market_price = current_prices.get(&pos.symbol).copied().unwrap_or(pos.avg_cost);
            total += pos.unrealized_pnl(market_price);
        }
        total
    }
}