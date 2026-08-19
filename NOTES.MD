# Rust Trading Platform

##### Commands
``` bash
 cargo new trading_platform
 cargo run 
 cargo check
 cargo test
 cargo test --test integration_test{filename in tests folder}
 cargo fmt --check
 cargo test --doc
 cargo doc --open
```


1. Order book & Matching Engine - Price time priority matching with sub microsecond in memory order routing
2. Per Trade Risk engine - Real time position limit, buying power and fat finger checks
3. Wallets & Portfolio manager - Multi currency ledger with real time mark to market P&L calculations
4. Market Data Streaming - Low latency Web Socket feeds and custom binary TCP protocols
5. Production Backend - Axum REST API, PostgresSQL (sqlx), Redis session cache, Tokio async runtime, Docker containerization.

- To learn Rust in conext. Ownership,borrowing, lifetimes, traits, generics , unsafe memory and async tasks exist because high throughput financial software demands deterministic memory usage without garbage collection pauses

## System Architecture and Component flow
- Single Binary Monolith Architecture - This ensures we master Rust's core memmory rules before adding multi-crate workspace overhead
```
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
```

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
```
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
```

### Method Receivers (&self, &mut, self )& Constructors (Self::new())

- Merthod Receiver mechanics (self parameter) - In Rust impl blocks, methods take selfas their first parameter , desugaring to self:Self, self:&Self or self:&mut Self
- Constructor Pattern - Self::new() - Associated function no self parameter. Acts as a factory returning an initialized Self on the stack. Self is an alias for the type being implemented 
- Ownership and Lifetime impace 
- &self : Borrows immutable stack reference(&Order) allowing concurrent readers
- self: takes owned move of Order, dropping or consuming the struct at the end of the method body
- &mut self : borrows exclusive mutable stack reference(&mut Order), enabling state mutation while preventing data races.

### Configuration System - Layered settings
- A trading system needs configurable parameters exchange name, base ,currency, maximum order size limits and log verbosity

- In production systems (Zerodha, Binance ) we cannot hardcode setting into binary executables .. Staging, dev, and production environment require different config values. A robust configuration manager loads setting from files (config.toml) overrides them with environment variables [EXCHANGE_NAME=ApexExchange] and falls back to safe default values
```
+-------------------------------------------------------------------------+
|                  CONFIGURATION HIERARCHY & RESOLUTION                   |
|                                                                         |
|  +---------------------------+                                          |
|  |  Environment Variables    |  (Highest priority: OVERRIDES file/def)  |
|  |  EXCHANGE_NAME=ApexEx...  |                                          |
|  +---------------------------+                                          |
|               |                                                         |
|               v                                                         |
|  +---------------------------+                                          |
|  |   File: config.toml       |  (Medium priority: reads disk settings)  |
|  |   currency = "USD"        |                                          |
|  +---------------------------+                                          |
|               |                                                         |
|               v                                                         |
|  +---------------------------+                                          |
|  |   Hardcoded Defaults      |  (Fallback: guarantees safe startup)     |
|  |   max_order_size = 1000   |                                          |
|  +---------------------------+                                          |
+-------------------------------------------------------------------------+
```

### Ownership and Borrowing 
1. Move Semanticsa and Stack Value Copying
- String is an uncopyable stack struct (pointer len cap). Assigning a = b performs a shallow 24 bytes memcpy of the stack fields, then invalidates b at compile time . No heap reallocation happens but b can no longer be accessed
2. Borrow Checker and Lifetime safety - &T borrows a pointer without moving ownership . The Rust borrow checker enforces that the refernce &T cannot outlive the owned value T(preventing dangling pointer vulnerabilities)
3. Option<T> and Fallback Combination - (unwrap_or , unwrap_or_else) : Option<T> is an enum (Some(T) || None). .unwrap_or(default) eagerly evaluates a default value while .unwrap_or_else(|| compute_default()) lazily evaluates teh closure only when None saving unnecessary heap allocations in config fallbacks.

#### Using unwrap_or_else **Idiomatic Functional Pattern**
- Use std::env::var("KEY_NAME") to look up an environment variable. It returns a Result<String, VarError>


#### Using **`match`** Explicit Control flow
-  use a match expression to explicitly handle Ok vs Err

#### File I/O (std::fs::read_to_string)


#### CLI Interface
```
+-------------------------------------------------------------------------+
|                  CLI COMMAND PARSING & DISPATCH FLOW                    |
|                                                                         |
|  $ cargo run -- buy --symbol BTC --qty 2 --price 5000000                |
|               |                                                         |
|               v                                                         |
|  +---------------------------+                                          |
|  |  clap::Parser             |  (Parses std::env::args_os tokens)       |
|  +---------------------------+                                          |
|               |                                                         |
|               v                                                         |
|  +---------------------------+                                          |
|  |  cli::Commands Enum       |  (Commands::Buy { symbol, qty, price })  |
|  +---------------------------+                                          |
|               |                                                         |
|               v                                                         |
|  +---------------------------+                                          |
|  |  Trading Engine Dispatch  |  (Executes order logic on models)        |
|  +---------------------------+                                          |
+-------------------------------------------------------------------------+
```

#### Erro handling
- IN real world trading engines invalid trade attemps (insufficient funds, nonexistent order IDs , bad order quantities) happen constantly
- Crashing the entire paltform process with panic! or unwrap() during live trading is catastrophic. Instead , production sytems use typed domain errors (Result<T, E>) to reject individual incalid operation cleanly, returning descriptive failure causes while keeping the exchange engine running at 100% uptime.
```
+-------------------------------------------------------------------------+
|                PRODUCTION ERROR PROPAGATION (& RESULT<T, E>)            |
|                                                                         |
|  Order Request (Buy 10 BTC, $50k)                                       |
|          |                                                              |
|          v                                                              |
|  +-------------------------------------+                                |
|  | Engine Validation                   |                                |
|  | Account Balance Check: $100 vs $50k |                                |
|  +-------------------------------------+                                |
|          |                                                              |
|          +---> Err(TradingError::InsufficientFunds { .. })              |
|                     |                                                   |
|                     v  (? Operator)                                     |
|  +-----------------------------------+                                  |
|  | CLI / API Handler                 |                                  |
|  | Catch Err & Format Display Output |  (No panic! Engine stays alive)  |
|  +-----------------------------------+                                  |
+-------------------------------------------------------------------------+
```

- `Result<T,E> , thiserror, ? operator`

#### Automatic Error conversions and Result Alias
- We enhance TradingError with automatic conversion of standard library and third-party errors (std::io::Error, toml::de::Error) using #[from] and define a crate-wide Result<T> type alias
- `? Operator , #[from], Result<T>`


- In any trading exchange user identity is the core pillar of auditability regulatory compliance KYC and portfolio isolation
- Every buy/sell order, balance update and trade execution must be immutably attributed to a unique trader. To achieve this securely
1. Every user receives a gloablly unique Uuid.
2. Raw password are NEVER stored in plain text. Instead passwords are run through one-way-cryptographic hash function (SHA-256) before persistence

```
+-------------------------------------------------------------------------+
|                  USER AUTHENTICATION ARCHITECTURE                       |
|                                                                         |
|  Registration:                                                          |
|  CLI Input ("alice", "SecretPass123")                                   |
|          |                                                              |
|          v                                                              |
|  +-------------------------------------+                                |
|  | SHA-256 One-Way Hash Engine         |                                |
|  | Sha256("SecretPass123")             |                                |
|  +-------------------------------------+                                |
|          |                                                              |
|          v                                                              |
|  +-------------------------------------------------------------------+  |
|  | User Struct { id: Uuid, username, password_hash, created_at }     |  |
|  +-------------------------------------------------------------------+  |
|                                                                         |
|  Login Verification:                                                    |
|  Input Password -> SHA-256 -> Compare Hash against Stored User Hash     |
+-------------------------------------------------------------------------+
``` 

#### Domain Identity, Trait contracts and Password hashing (User Struct , sha2, uuid::Uuid, chrono::Utc)
- `Uuid::new_v4()`
- `sha2::Sha256` - `Sha256::digest()`
- `chrono::DateTime<Utc>`- `chrono` [`created_at, last_login`]

#### In Memory `UserManager` & Dual Index - Authentication
- we build the UserManager service to maintain registered users in memory with fast $O(1)$ dual index lookup `HashMap<Uuid, User> and HashMap<String, Uuid>`

#### Wallet system - Money Management

- In trading platform , the Wallet Subsystem is the financial ledger. before a trader can place a buy order for 2 BTC and 50,000 the exchange wallet must verify that the user holds at least $100,000 USD in available funds . To guarantee balance integrity
    1. Operations on currency balances must be atomic (avoiding race conditions or double lookups)
    2. Withdrawals must enforce overdraft protection (rejecting attempts that exceed available balance)

```
+-------------------------------------------------------------------------+
|                    MULTI-CURRENCY WALLET ARCHITECTURE                   |
|                                                                         |
|  Deposit Request (e.g. Deposit $10,000 USD)                             |
|          |                                                              |
|          v                                                              |
|  +-------------------------------------------------------------------+  |
|  | HashMap Entry API: balances.entry("USD").or_insert(0)             |  |
|  | *balance += 10_000                                                |  |
|  +-------------------------------------------------------------------+  |
|                                                                         |
|  Withdrawal Request (e.g. Withdraw $15,000 USD)                         |
|          |                                                              |
|          v                                                              |
|  +-------------------------------------------------------------------+  |
|  | Overdraft Check: available ($10,000) < required ($15,000)         |  |
|  +-------------------------------------------------------------------+  |
|          |                                                              |
|          +---> Err(TradingError::InsufficientFunds { .. })              |
+-------------------------------------------------------------------------+
```

#### Transaction history and Iterator based Filtering
- We will add transaction history audit logging to wallet  and implement iterator filter pipelines
- .iter().filter(...).collect() to query history by currency

- Calling `.clone()` tries to clone the iterator adapter struct itself 
- Calling `.cloned()` clones the items inside the iterator(converting &TransactionRecord) into TransactionRecord
- .collect() then gathers those owned TransactionRecord items into a Vec<TransactionRecord>

#### Wallet Accumulation turbofish `::<>` ,&Closure Generics

```
+-------------------------------------------------------------------------+
|                  WALLET ACCUMULATION & CLOSURE FLOW                     |
|                                                                         |
|  balances: HashMap<String, u64> -> .values().sum::<u64>()               |
|                                         |                               |
|                                         v                               |
|                                 Total Portfolio Sum                     |
|                                                                         |
|  history: Vec<TransactionRecord> -> .iter()                             |
|                                       |                                 |
|                                       v                                 |
|                  predicate(&record) matching filter                     |
|                                       |                                 |
|                                       v                                 |
|                  Turbofish .collect::<Vec<TransactionRecord>>()         |
+-------------------------------------------------------------------------+

```

#### Portfolio management - your holdings
- In stock and crpto trading platforms , A portfolio tracks an investor's net market positions
- When a trader buys 1 BTC at $40000 and later buys 1 more BTC at $60000.
    1. they don't just hold two isolated tickets, the broker tracks their aggregate Position (2 BTC) with a Weighted Average cost Basis ($50,000 per BTC)
    2. As market prices fluctuate, the broker computes Unrealized P&L to display live profit/loss percentage and total portfolio valuation

```
+-------------------------------------------------------------------------+
|                  PORTFOLIO POSITION TRACKER ARCHITECTURE                |
|                                                                         |
|  Buy Order Executed (e.g., Buy 1 BTC @ $60,000)                         |
|          |                                                              |
|          v                                                              |
|  +-------------------------------------------------------------------+  |
|  | Position State Update:                                            |  |
|  | new_qty = old_qty + add_qty                                       |  |
|  | new_avg = (old_qty * old_avg + add_qty * buy_price) / new_qty     |  |
|  +-------------------------------------------------------------------+  |
|          |                                                              |
|          v                                                              |
|  +-------------------------------------------------------------------+  |
|  | Real-Time P&L Valuation:                                          |  |
|  | unrealized_pnl = qty * (current_market_price - avg_cost)          |  |
|  +-------------------------------------------------------------------+  |
+-------------------------------------------------------------------------+
```

#### Portfolio Tracker Engine and Custom Position Sorting
#### Data Structure Ordering and Custom Vector sorting
- `BTreeMap` vs `HashMap` , `.sort_by()`, `PartialOrd::partial_cmp`

#### Order management - Basic OMS engine
- In real financial exchanges the order management system OMS coordinate trade execution . When a trader submits an order
    1. The exchange assigns a type-sage globally unique ID (OrderId(u64))
    2. The order enters the system in a Pending state
    3. Orders move through a strict State Maching Lifecycle: Pending -> Filled -> Cancelled/Rejected. Invalid state transitions are strictly rejected by type invariants

```
+-------------------------------------------------------------------------+
|                  ORDER MANAGEMENT ENGINE & STATE MACHINE                |
|                                                                         |
|  Order Creation (Order::new)                                            |
|          |                                                              |
|          v                                                              |
|  +-------------------------------------------------------------------+  |
|  | State: OrderStatus::Pending                                       |  |
|  +-------------------------------------------------------------------+  |
|          |                                     |                        |
|          | (Fill Match)                        | (User Cancel)          |
|          v                                     v                        |
|  +---------------------------+       +-------------------------------+  |
|  | OrderStatus::Filled       |       | OrderStatus::Cancelled        |  |
|  +---------------------------+       +-------------------------------+  |
+-------------------------------------------------------------------------+

```

- `Newtype Pattern` and `Order Lifecycle state machine`
- The Builder Pattern and method => Chaining OrderBuilder , Fluent Interface, Validation
- Encapsulation and Staging Incomplete state

#### File Persistence - Saving State
- In real financial infrastructure in-memory data must survive system crashes and server restarts. If an exchange restarts or power is lost:
    1. User registration, multi-currency wallet balances and open orderbooks cannot be wiped out
    2. The exchange uses Serialization(serde / serde_json) to convert in-memory Rust structs into JSON file on disk
    3. On startup, the system Deserializes the files to reconstruct identical in-memory state

```
+-------------------------------------------------------------------------+
|                  EXCHANGE PERSISTENCE ENGINE & DATA FLOW                |
|                                                                         |
|  In-Memory Struct (User, Wallet, Order)                                 |
|          |                                                              |
|          | serde_json::to_string_pretty(&data)                          |
|          v                                                              |
|  +-------------------------------------------------------------------+  |
|  | Serialized JSON String                                            |  |
|  +-------------------------------------------------------------------+  |
|          |                                                              |
|          | std::fs::write(&path, json_str)                              |
|          v                                                              |
|  +-------------------------------------------------------------------+  |
|  | Disk Storage: data/users.json, data/wallets.json                  |  |
|  +-------------------------------------------------------------------+  |
|          |                                                              |
|          | std::fs::read_to_string(&path)                               |
|          v                                                              |
|  +-------------------------------------------------------------------+  |
|  | serde_json::from_str::<T>(&json_str)                              |  |
|  +-------------------------------------------------------------------+  |
|          |                                                              |
|          v                                                              |
|  Restored In-Memory Struct                                              |
+-------------------------------------------------------------------------+

```

- `#[derive(Serialize, Deserialize)]` - Auto generates serialization code for custom structs and enums at compile time without reflection overhead
- PathBuf vs &Path - `std::path::PathBuf` owns its underlying path buffer on the heap and support mutation (path.push("data.json"))
- `std::path::Path` is an unsized path slice reference (&Path) used in function parameter signatures for flexible borrowing.


#### Deriving serde traits and serde field customization attributes (#[defive(Serialize, Deserialize)], #[serde(rename_all = "...")], #[serde(skip)], Serde Round-Trip Testing)

- Serialize and Deserialize - Stamping every item in our store with a universal bar code tag so any automatic scanning robot can instatnly register pack or unpack it without needing custom instructions
- Applying #[derive(Serialize,Deserialize)] to struct and enums invokes Rust procedural macros to generate Serializer and Deserializer trait implementations at compile time, maintaining zero-overhead performance 


#### Positions Tracking and Order execution
- IN real brokerage and exchnage systems. 
    1. Opening Fills - When a buy order executes an existing positions quantity and weighted average cost basis increase
    2. Closing fills - When a sell order executes , the trader closes part or all of their long positions . The system locks in realized P&L `(sell_price - avg_cost_basis) * qty_sold` , updates remaining quantity and remove the position if quantity reaches zero
    3. Mark-to-Market Accounting - Total P&L is the sum of Realized P&L(locked-in cash profit/loss) + Unrealized P&L (paper profit/loss of currently open positions)

```
+-------------------------------------------------------------------------+
|              POSITION TRACKING & SMART POINTER ARCHITECTURE             |
|                                                                         |
|  Order Execution Handler              Portfolio Manager                 |
|         \                                   /                           |
|          \  Rc::clone(&pos_ref)            /  Rc::clone(&pos_ref)       |
|           v                               v                             |
|  +-------------------------------------------------------------------+  |
|  | Rc<RefCell<Position>>  (Shared Heap Ownership & Interior Mutability)||
|  |   - RefCell.borrow_mut() -> Mutate qty & avg_cost_basis           |   |
|  +-------------------------------------------------------------------+  |
|                                                                         |
|  Mark-to-Market P&L Calculation:                                        |
|  Total P&L = Realized P&L (closed fills) + Unrealized P&L (open pos)    |
+-------------------------------------------------------------------------+

```

##### Heap Allocation and Shared Interior Mutability (Box<T>, RC<T>, RefCell<T>, Rc<RefCell<T>>)

##### System Architecture & Precedence Flow 

```
+-------------------------------------------------------------------------+
|                  CONFIGURATION PRECEDENCE ARCHITECTURE                  |
|                                                                         |
|  1. Config::default() -> Hardcoded fallback (port: 8080)                |
|           |                                                             |
|           v                                                             |
|  2. Config::from_file("config.toml") -> Overrides with file settings    |
|           |                                                             |
|           v                                                             |
|  3. Config::apply_env_overrides(&mut self)                              |
|         - Reads std::env::var("TRADING_PORT")                           |
|         - If present: overrides self.network.port                       |
+-------------------------------------------------------------------------+

```

##### Reading Environment Variables `std::env::var` , `VarError`

#### Testing Suite ( Integration and Result-Returning tests)

```
       Customer Interface / Integration Tests (tests/integration_test.rs)
                                  │
                                  ▼ (use trading_platform::*)
  ┌───────────────────────────────┴────────────────────────────────┐
  │                           src/lib.rs                           │
  └───────┬───────────────┬───────────────┬────────────────┬───────┘
          │               │               │                │
          ▼               ▼               ▼                ▼
     src/wallet.rs   src/orders.rs   src/tracker.rs   src/storage.rs

```


- `///` Outer doc comment attirbute macro targeting the function  immediately below.

##### Refactoring codebase to production build
```
                              src/lib.rs (Public API Facade)
                                          │ (pub use)
                  ┌───────────────────────┴───────────────────────┐
                  ▼                                               ▼
         src/models.rs (Domain Data)                  src/services.rs (Business Engines)
          ├── models/wallet.rs                         ├── services/order_manager.rs
          ├── models/portfolio.rs                      └── services/tracker.rs
          └── models/user.rs

```

#### Documentation and code quality
- In production financial software, undocumented API's lead to developer misuse, incorrect trade configurations, and compliance violations
- We will be turning our trading platform into a production grade library with
    1. Inner Module Documentation (//!) - Crate level architectural overviews generated by `cargo doc`
    2. Intra-Doc-Symbol Links - Hyperlinking code symbols directly inside Markdown documentation {`[Wallet]`}
    3. Strict doc Lints (`#![deny(missing_docs)]`) - Compiler enforced requirement that every public struct enum and function must be documented
    4. Clippy and Formatting standards - Enforcing production code quality rules via clippy.toml and rustfm.toml.

##### Production Code Quality and lint enforcement (`#![denu(missing_cos)], clipp.toml, rustfmt.toml`)
- Implement high-precision runtime execution latency benchmarking using `std::time::Instant`, Profile trade execution and persistence speeds, review our complete architecture and prepare the project .
```
                  Instant::now() (High-Precision Monotonic CPU Clock Start)
                                            │
                                            ▼
                    ┌──────────────────────────────────────────┐
                    │          Operation Execution             │
                    │   (Trade Fill / Persistence / P&L)       │
                    └───────────────────────┬──────────────────┘
                                            │
                                            ▼
                  start.elapsed().as_micros() (Nanosecond Resolution Delta)
                                            │
                                            ▼
                  [BENCHMARK] operation executed in X µs Banner

```

- <F , R> => Declares generic parameters F(closure type) and R(return type)
- op: F => Takes ownership of closure instance (op)
- where => Clause specifying generice trait bounds
- F: FnOnce() -> R => Constraints F to a closure that takes zero arguments can be called at least once and returns R.

#### Benchmarking Service Operation and Idiomatic Rust performance patterns

- Now that we have created the generic benchmark_operation function src/services/tracker.rs, we are going to instrument our core trading services (such as OrderManager::submit_order) to record real world execution latency without mutating our underlying domain logic

```
                                    submit_order_benchmarked(...)
                                                  │
                                                  ▼
                                    benchmark_operation("submit_order", || {
                                        self.submit_order(symbol, side, qty, price)
                                    })
                                                  │
                                 ┌────────────────┴────────────────┐
                                 ▼                                 ▼
                     (Executed Order Result)           (Elapsed Latency in µs)

```