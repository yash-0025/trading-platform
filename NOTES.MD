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


- `&str` **[String Literal / Slice ]** => Like a billboard painted directly onto a building wall. we didn't buy the building and we can't erase or add new words to the wall. But pointing to it `(&)` is instantaneous and costs zero money (zero allocations)
- `String` **(Heap Allocated String)** => Like a spiral notepad in our backpack. We own it we can tear out pages or write 100 new lines `(push_str)`but carrying around physical notebooks required effort and memory management overhead

- Asset Side - Buy || Sell
- Order type = Market || Limit || StopLoss
- Order Status
- Order
- Price 
- Quantity

- In traditional langugaes like Js or python developers ofter pass plain strings like "BUY" or "LIMIT" around. If someone makes a type "buy" or "LMIT" the error happens at runtime in production
- In rust system engineering the type system makes invalid states unrepresentable at compile time

+-------------------------------------------------------------------------+
|                         src/models/ (Domain Types)                      |
|                                                                         |
|  +------------------------+          +-------------------------------+  |
|  |     Side (Enum)        |          |       OrderType (Enum)        |  |
|  |   - Buy                |          |   - Market                    |  |
|  |   - Sell               |          |   - Limit { price: i64 }      |  |
|  +------------------------+          |   - StopLoss { trigger: i64 } |  |
|               \                      +-------------------------------+  |
|                \                                     /                  |
|                 v                                   v                   |
|  +-------------------------------------------------------------------+  |
|  |                           Order (Struct)                          |  |
|  |   id: u64, asset: String, side: Side, order_type: OrderType, ...  |  |
|  +-------------------------------------------------------------------+  |
+-------------------------------------------------------------------------+

