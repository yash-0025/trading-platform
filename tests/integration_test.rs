use std::collections::HashMap;
use trading_platform::models::Wallet;
use trading_platform::services::{OrderManager, OrderSide, OrderType, PositionTracker};

#[test]
fn test_end_to_end_trading_flow() -> Result<(), String> {
    // Initialize wallet and deposit funds
    let mut wallet = Wallet::new();
    wallet.deposit("USD".to_string(), 100_000);
    if wallet.get_balance("USD") != 100_000 {
        return Err("Wallet deposit failed".to_string());
    }

    //  Initialize Ordermanager and storage and submit a Buy Order for BTC
    let mut order_mgr = OrderManager::new();
    let order_id = order_mgr.submit(
        "BTC".to_string(),
        OrderSide::Buy,
        OrderType::Limit { limit_price: 40000 },
        2,
    );
    if order_id.0 != 1 {
        return Err("Order ID auto-increment failed".to_string());
    }

    // Initialize the positioin tracker
    let mut tracker = PositionTracker::new();
    tracker.process_fill(OrderSide::Buy, "BTC", 2.0, 40000.0);
    if tracker
        .positions
        .get("BTC")
        .ok_or("Misssing position")?
        .quantity
        != 2.0
    {
        return Err("Position fill quantity mismatch".to_string());
    }

    // Verify mark-to-market total P&L at BTC = 45000
    let prices = HashMap::from([("BTC".to_string(), 45000.0)]);
    if tracker.total_pnl(&prices) != 10000.0 {
        return Err("Total mark-to-market P&L mismatch".to_string());
    }

    Ok(())
}
