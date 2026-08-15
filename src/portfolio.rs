
#[derive(Debug, Clone, PartialEq)]
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
