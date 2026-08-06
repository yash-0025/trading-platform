# Rust Trading Platform

1. Order book & Matching Engine - Price time priority matching with sub microsecond in memory order routing
2. Per Trade Risk engine - Real time position limit, buying power and fat finger checks
3. Wallets & Portfolio manager - Multi currency ledger with real time mark to market P&L calculations
4. Market Data Streaming - Low latency Web Socket feeds and custom binary TCP protocols
5. Production Backend - Axum REST API, PostgresSQL (sqlx), Redis session cache, Tokio async runtime, Docker containerization.

- To learn Rust in conext. Ownership,borrowing, lifetimes, traits, generics , unsafe memory and async tasks exist because high throughput financial software demands deterministic memory usage without garbage collection pauses

## System Architecture and Component flow
- Single Binary Monolith Architecture - This ensures we master Rust's core memmory rules before adding multi-crate workspace overhead

+-----------------------------------------------------------------------+
|                    TRADING PLATFORM CLI (Phase 1)                     |
|                                                                       |
|  +-------------------+    +-------------------+    +---------------+  |
|  |     CLI / REPL    |--->|  Order Manager    |--->| Wallet System |  |
|  |  (Input / Flags)  |    |  (State Machine)  |    |  (Balances)   |  |
|  +-------------------+    +-------------------+    +---------------+  |
|            |                        |                      |          |
|            v                        v                      v          |
|  +-----------------------------------------------------------------+  |
|  |                    Domain Models (Structs/Enums)                |  |
|  +-----------------------------------------------------------------+  |
|            |                                                          |
|            v                                                          |
|  +-----------------------------------------------------------------+  |
|  |               File Persistence Manager (JSON / Serde)           |  |
|  +-----------------------------------------------------------------+  |
+-----------------------------------------------------------------------+

### Data & Memory Flow
1. Command input - User issues a CLI command (eg - buy --asset AAPL --qty 10 --price 15000)
2. Validation & State check - Order Manager validates quantity/price types, checks Wallet balances (&mut Wallet) and constructs an Order struct
3. Execution & Mutability - State updates happen synchronously in memory. Exclusinve mutable references (&mut) guarantee no two sub components mutate wallet balances simultaneously
4. Serialization & Peristence - State Changes serialize via serde to tasks.json / state.json


