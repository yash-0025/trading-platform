use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fmt;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Portfolio {
    pub positions: HashMap<String, Position>,
    pub sorted_holdings: BTreeMap<String, Position>,
}

impl Portfolio {
    pub fn new() -> Self {
        Self::default()
    }

    // Idiomatic entry chain => .and_modify() + .or_insert_with()
    // .and_modify(|pos| ) - If the symbol already exists in positions it runs the closure on |pos| passing a mutable reference &mut Position so we can cal pos.update(quantity, price)
    // .or_insert_with(|| ...) If the symbol does not exist yetm it creates a new Position::new(...) and inserts it.

    // ----- Other way to write this function is this
    /*
        use std::collections::hash_map::Entry;

        match self.positions.entry(symbol.clone()) {
            Entry::Occupied(mut entry) = {
                entry.get_mut().update(quantity. price);
            }
            Entry::Vacant(entry) => {
                entry.insert(Position::new(symbol, quantity, price));
            }
        }

    */
    pub fn add_position(&mut self, symbol: String, quantity: f64, price: f64) {
        self.positions
            .entry(symbol.clone())
            .and_modify(|pos| pos.update(quantity, price))
            .or_insert_with(|| Position::new(symbol, quantity, price));
    }

    pub fn get_position(&self, symbol: &str) -> Option<&Position> {
        self.positions.get(symbol)
    }

    pub fn get_sorted_positions(&self, current_prices: &HashMap<String, f64>) -> Vec<Position> {
        let mut positions: Vec<Position> = self.positions.values().cloned().collect();
        positions.sort_by(|a, b| {
            let price_a = current_prices.get(&a.symbol).copied().unwrap_or(0.0);
            let price_b = current_prices.get(&b.symbol).copied().unwrap_or(0.0);
            b.unrealized_pnl(price_b)
                .partial_cmp(&a.unrealized_pnl(price_a))
                .unwrap_or(Ordering::Equal)
        });

        positions
    }

    pub fn add_to_sorted(&mut self, symbol: String, quantity: f64, price: f64) {
        self.add_position(symbol.clone(), quantity, price);
        self.sorted_holdings
            .entry(symbol.clone())
            .and_modify(|pos| pos.update(quantity, price))
            .or_insert_with(|| Position::new(symbol, quantity, price));
    }

    pub fn portfolio_report(&self, _current_prices: &HashMap<String, f64>) -> Vec<String> {
        let lines = self
            .sorted_holdings
            .values()
            .enumerate()
            .map(|(idx, pos)| format!("# {}: {}", idx + 1, pos));
        let summary = std::iter::once(format!("Total Positions: {}", self.sorted_holdings.len()));
        lines.chain(summary).collect::<Vec<String>>()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {:.2} shares @ avg ${:.2}",
            self.symbol, self.quantity, self.avg_cost
        )
    }
}

impl fmt::Display for Portfolio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, pos) in self.sorted_holdings.values().enumerate() {
            writeln!(f, "# {}: {}", idx + 1, pos)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub avg_cost: f64,
}

impl Position {
    pub fn new(symbol: String, quantity: f64, price: f64) -> Self {
        Position {
            symbol,
            quantity,
            avg_cost: price,
        }
    }

    pub fn update(&mut self, add_qty: f64, buy_price: f64) {
        let total_cost = (self.quantity * self.avg_cost) + (add_qty * buy_price);
        let total_qty = self.quantity + add_qty;
        self.avg_cost = total_cost / total_qty;
        self.quantity = total_qty;
    }

    pub fn unrealized_pnl(&self, current_price: f64) -> f64 {
        self.quantity * (current_price - self.avg_cost)
    }
}
