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
+-------------------------------------------------------------------------+
|                  CLI COMMAND PARSING & DISPATCH FLOW                    |
|                                                                         |
|  $ cargo run -- buy --symbol BTC --qty 2 --price 5000000                |
|               |                                                         |
|               v                                                         |
|  +---------------------------+                                          |
|  |  clap::Parser             |  (Parses std::env::args_os tokens)      |
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


#### Erro handling
- IN real world trading engines invalid trade attemps (insufficient funds, nonexistent order IDs , bad order quantities) happen constantly
- Crashing the entire paltform process with panic! or unwrap() during live trading is catastrophic. Instead , production sytems use typed domain errors (Result<T, E>) to reject individual incalid operation cleanly, returning descriptive failure causes while keeping the exchange engine running at 100% uptime.

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

- `Result<T,E> , thiserror, ? operator`

#### Automatic Error conversions and Result Alias
- We enhance TradingError with automatic conversion of standard library and third-party errors (std::io::Error, toml::de::Error) using #[from] and define a crate-wide Result<T> type alias
- `? Operator , #[from], Result<T>`


- In any trading exchange user identity is the core pillar of auditability regulatory compliance KYC and portfolio isolation
- Every buy/sell order, balance update and trade execution must be immutably attributed to a unique trader. To achieve this securely
1. Every user receives a gloablly unique Uuid.
2. Raw password are NEVER stored in plain text. Instead passwords are run through one-way-cryptographic hash function (SHA-256) before persistence

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

#### Transaction history and Iterator based Filtering
- We will add transaction history audit logging to wallet  and implement iterator filter pipelines
- .iter().filter(...).collect() to query history by currency

- Calling `.cone()` tries to clone the iterator adapter struct itself 
- Calling `.cloned()` clones the items inside the iterator(converting &TransactionRecord) into TransactionRecord
- .collect() then gathers those owned TransactionRecord items into a Vec<TransactionRecord>