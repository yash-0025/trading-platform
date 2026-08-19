# 🔐 SOLUTIONS.md — Gated Reference Solutions

> **Read this only after you've attempted the matching exercise in `EXERCISES.md` and explicitly asked to see the solution.**
> Reading ahead defeats the point — the exercise is where the actual learning happens, not the solution.
>
> The AI will not open or paste from this file until both gate conditions in `.agents/workflow/next.md` STEP 3.5-C are met:
> 1. You've pasted/described a real attempt (even a broken or partial one).
> 2. You've explicitly asked to see the solution.

**Entry numbering matches `EXERCISES.md` exactly** — Exercise 1.1-1 → Solution 1.1-1, etc.

---

## Entry Format

```
### Solution <module#>.<n> — <short title>

**Reference implementation:**
​```rust
fn example() -> Result<(), TradingError> {
    if quantity == 0 {
        return Err(TradingError::InvalidQuantity);
    }
    // ...
    Ok(())
}
​```

**Line-by-line:**
- `if quantity == 0 { ... }` — why this check, why here, what it prevents.
- `return Err(TradingError::InvalidQuantity)` — why this error variant, how `?` would propagate it upstream.
- ...

**Compared to your attempt:**
- **Matches**: Perfect enum structures, visibility `pub`, correct payload types (`i64`), and derive attributes (`Debug`, `Clone`, `PartialEq`, `Eq`).
- **Difference**: You named the variant `Stoploss` (lowercase `l`), while idiomatic Rust uses CamelCase `StopLoss`. Both compile and are functionally equivalent.

---

### Solution 1.2-2 — Structs & Newtype Pattern (`Price`, `Quantity`, `Order`)

**Reference Implementation:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub id: u64,
    pub asset: String,
    pub side: Side,
    pub order_type: OrderType,
    pub qty: Quantity,
    pub status: OrderStatus,
}
```

**Line-by-Line Breakdown:**
- `pub struct Price(pub i64);` — Newtype pattern tuple struct wrapping `i64`. Derives `Copy` for zero-cost stack copying.
- `pub struct Quantity(pub u64);` — Newtype pattern tuple struct wrapping `u64`.
- `pub struct Order { ... }` — Named-field struct aggregating fields into a cohesive order entity.

**Compared to your attempt:**
- **Matches**: Perfect field layout, newtype definitions, visibility `pub`, and enum variants.
- **Compiler Fix Needed**: You typed `ParitalEq` (typo: `i` before `t`) twice on `Quantity` and `Order`. Changing `ParitalEq` → `PartialEq` will fix compiler errors.

---

### Solution 1.2-3 — Impl Blocks, Constructors (`Self::new()`), and Method Mutability (`&mut self`)

**Reference Implementation:**
```rust
impl Order {
    pub fn new(id: u64, asset: String, side: Side, order_type: OrderType, qty: Quantity) -> Self {
        Order {
            id,
            asset,
            side,
            order_type,
            qty,
            status: OrderStatus::Pending,
        }
    }

    pub fn fill(&mut self) {
        self.status = OrderStatus::Filled;
    }

    pub fn is_pending(&self) -> bool {
        self.status == OrderStatus::Pending
    }
}
```

**Line-by-Line Breakdown:**
- `impl Order { ... }` — Implementation block attaching functions and methods to the `Order` struct.
- `pub fn new(...) -> Self` — Associated function constructor. `Self` is a type alias for `Order`. Field init shorthand (`id`, `asset`, etc.) maps parameter names directly to struct fields. Sets default `status: OrderStatus::Pending`.
- `pub fn fill(&mut self)` — Method taking an exclusive mutable borrow (`&mut self`) to mutate `self.status = OrderStatus::Filled`.
- `pub fn is_pending(&self) -> bool` — Method taking an immutable shared borrow (`&self`) to inspect status without mutating memory.

**Compared to your attempt:**
- **Exact Match!**: Your implementation used field initialization shorthand, correct `&mut self` mutation, and `&self` equality comparison. Flawless!

---

### Solution 1.3-1 — Config Struct & Env Fallback (`Option<T>` & `unwrap_or_else`)

**Reference Implementation:**
```rust
#[derive(Debug, Clone)]
pub struct Config {
    pub exchange_name: String,
    pub currency: String,
    pub max_order_size: u64,
    pub log_level: String,
}

impl Config {
    pub fn from_env_or_default() -> Self {
        let exchange_name = std::env::var("EXCHANGE_NAME")
            .unwrap_or_else(|_| "ApexExchange".to_string());

        let currency = std::env::var("CURRENCY")
            .unwrap_or_else(|_| "USD".to_string());

        let max_order_size = std::env::var("MAX_ORDER_SIZE")
            .ok()
            .and_then(|val| val.parse::<u64>().ok())
            .unwrap_or(1_000_000);

        let log_level = std::env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "INFO".to_string());

        Config {
            exchange_name,
            currency,
            max_order_size,
            log_level,
        }
    }
}
```

**Line-by-Line Breakdown:**
- `pub fn from_env_or_default() -> Self` — Associated constructor function returning a `Config` instance.
- `std::env::var("EXCHANGE_NAME")` — Queries environment table, returning `Result<String, VarError>`.
- `.unwrap_or_else(|_| "ApexExchange".to_string())` — Lazily falls back to `"ApexExchange"` if the environment variable is missing.
- `Config { exchange_name, currency, max_order_size, log_level }` — Field init shorthand returning `Self`.

**Compared to your attempt:**
- **Matches**: Excellent logic! You wrote both the `unwrap_or_else` functional pattern and the `match` expression pattern.
- **Adjustments Needed**:
  1. Move the statements inside the body of `pub fn from_env_or_default() -> Self { ... }`.
  2. Change `match` arm syntax from `->` to `=>` (e.g. `Ok(val) => val`).
  3. Change `std::env("CURRENCY")` and `std::var(...)` to `std::env::var(...)`.

---

### Solution 1.3-2 — File Parsing & Layered Fallback (`config.toml`, `std::fs::read_to_string`)

**Reference Implementation:**
```rust
impl Config {
    pub fn from_file_or_env(path: &str) -> Self {
        match std::fs::read_to_string(path) {
            Ok(_contents) => {
                Self::from_env_or_default()
            }
            Err(_) => {
                Self::from_env_or_default()
            }
        }
    }
}
```

**Line-by-Line Breakdown:**
- `pub fn from_file_or_env(path: &str) -> Self` — Associated function taking a string slice path to `config.toml`.
- `std::fs::read_to_string(path)` — Reads raw UTF-8 file contents into a heap `String`, returning `io::Result<String>`.
- `match` arms — Handles `Ok(_contents)` and `Err(_)`, safely falling back to `Self::from_env_or_default()`.

**Compared to your attempt:**
- **Exact Match!**: Your implementation used `match std::fs::read_to_string(path)` with `Ok(_contents)` and `Err(_)` fallbacks flawlessly!

---

### Solution 1.3-3 — Serde TOML Deserialization (`serde`, `toml::from_str`)

**Reference Implementation:**
```rust
// Cargo.toml:
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// toml = "0.8"

// src/config.rs:
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub exchange_name: String,
    pub currency: String,
    pub max_order_size: u64,
    pub log_level: String,
}

impl Config {
    pub fn from_file_or_env(path: &str) -> Self {
        match std::fs::read_to_string(path) {
            Ok(contents) => match toml::from_str::<Config>(&contents) {
                Ok(config) => config,
                Err(_) => Self::from_env_or_default(),
            },
            Err(_) => Self::from_env_or_default(),
        }
    }
}
```

**Line-by-Line Breakdown:**
- `use serde::Deserialize;` — Imports Serde's `Deserialize` derive macro.
- `#[derive(Debug, Clone, Deserialize)]` — Instructs compiler to generate a Serde deserializer for `Config`.
- `toml::from_str::<Config>(&contents)` — Deserializes raw UTF-8 string `&contents` into a `Config` struct instance.
- Nested `match` — Safely handles TOML parse errors, falling back to `Self::from_env_or_default()`.

**Compared to your attempt:**
- **Exact Match!**: You declared `serde` and `toml` in `Cargo.toml`, derived `Deserialize` on `Config`, and wrote the nested `match toml::from_str::<Config>(&contents)` flawlessly!

---

### Solution 1.4-1 — CLI Commands & Subcommands (`clap`, `Parser`, `Subcommand`)

**Reference Implementation:**
```rust
// Cargo.toml:
// [dependencies]
// clap = { version = "4.4", features = ["derive"] }

// src/cli.rs:
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "trading-platform", author, version, about = "CLI Trading Terminal")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Buy {
        symbol: String,
        qty: u64,
        price: i64,
    },
    Sell {
        symbol: String,
        qty: u64,
        price: i64,
    },
    Balance,
    Orders,
}
```

**Line-by-Line Breakdown:**
- `use clap::{Parser, Subcommand};` — Imports `clap`'s derive macros for command line parsing.
- `#[derive(Parser, Debug)]` — Generates a top-level CLI argument parser.
- `#[command(name = "trading-platform", ...)]` — Sets application metadata shown in `--help` output.
- `#[derive(Subcommand, Debug)]` — Generates subcommand parsing for enum variants.
- `Buy { symbol, qty, price }` / `Sell { symbol, qty, price }` — Enum variants carrying named argument payloads.

**Compared to your attempt:**
- **Exact Match!**: Your implementation in `src/cli.rs` and dependency declaration in `Cargo.toml` were flawless!

---

### Solution 1.4-2 — Command Parsing & Dispatching (`Cli::parse()`, `match cli.command`)

**Reference Implementation:**
```rust
// src/main.rs:
use clap::Parser;

mod cli;
mod config;
mod models;

use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Buy { symbol, qty, price } => {
            println!("[ORDER SUBMITTED] BUY {} shares of {} at ${}", qty, symbol, price);
        }
        Commands::Sell { symbol, qty, price } => {
            println!("[ORDER SUBMITTED] SELL {} shares of {} at ${}", qty, symbol, price);
        }
        Commands::Balance => {
            println!("[ACCOUNT BALANCE] $100,000.00 USD");
        }
        Commands::Orders => {
            println!("[OPEN ORDERS] No open orders.");
        }
    }
}
```

**Line-by-Line Breakdown:**
- `use clap::Parser;` — Brings `Parser` trait into scope to call `Cli::parse()`.
- `let cli = Cli::parse();` — Reads OS command-line arguments, validates subcommands/flags, and parses into `Cli`.
- `match &cli.command` — Borrows `cli.command` to exhaustively handle every subcommand variant.
- `Commands::Buy { symbol, qty, price } => ...` — Destructures payload fields for clean string formatting output.

**Compared to your attempt:**
- **Exact Match!**: Your implementation in `src/main.rs` destructured fields and handled all subcommand variants cleanly and correctly!

---

### Solution 1.4-3 — Environment Variable Overrides (`std::env::var`, `TRADING_MAX_ORDER_SIZE`)

**Reference Implementation:**
```rust
// src/config.rs:
use std::env;

impl Config {
    pub fn apply_env_overrides(&mut self) {
        if let Ok(val_str) = env::var("TRADING_MAX_ORDER_SIZE") {
            if let Ok(size) = val_str.parse::<u64>() {
                self.max_order_size = size;
            }
        }
    }
}
```

**Line-by-Line Breakdown:**
- `if let Ok(val_str) = env::var("TRADING_MAX_ORDER_SIZE")` — Checks if `TRADING_MAX_ORDER_SIZE` exists in host OS environment table.
- `if let Ok(size) = val_str.parse::<u64>()` — Parses environment string into numeric `u64` size value.
- `self.max_order_size = size;` — Dynamically overrides `max_order_size` configuration field.

**Compared to your attempt:**
- **Easy Fix**: In `src/config.rs`, `Config` has fields `exchange_name`, `currency`, `max_order_size`, and `log_level` (without a nested `network` field). Updating line 97 to `self.max_order_size = port as u64;` (or parsing `u64` into `self.max_order_size`) makes it compile cleanly!

---


### Solution 1.5-1 — Custom `TradingError` Enum (`thiserror`, `#[derive(Error)]`)


**Reference Implementation:**
```rust
// Cargo.toml:
// [dependencies]
// thiserror = "1.0"

// src/errors.rs:
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TradingError {
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },

    #[error("Order not found with ID {order_id}")]
    OrderNotFound { order_id: u64 },

    #[error("Invalid order quantity: {message}")]
    InvalidQuantity { message: String },
}
```

**Line-by-Line Breakdown:**
- `use thiserror::Error;` — Imports Serde's/thiserror's procedural derive macro.
- `#[derive(Error, Debug)]` — Generates implementations of `std::fmt::Display` and `std::error::Error` for `TradingError`.
- `#[error("Insufficient funds...")]` — Macro attribute specifying exact user-facing formatting rules using named struct fields.
- `InsufficientFunds { required, available }` — Typed enum variant carrying precise numeric diagnostic payload.

**Compared to your attempt:**
- **Exact Match!**: Your implementation in `src/errors.rs` and dependency declaration in `Cargo.toml` were flawless!

---

### Solution 1.5-2 — Automatic Error Conversions (`#[from]`) & Custom `Result` Type Alias

**Reference Implementation:**
```rust
// src/errors.rs:
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TradingError {
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },

    #[error("Order not found with ID {order_id}")]
    OrderNotFound { order_id: u64 },

    #[error("Invalid order quantity: {message}")]
    InvalidQuantity { message: String },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config parse error: {0}")]
    ConfigParse(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, TradingError>;
```

**Line-by-Line Breakdown:**
- `Io(#[from] std::io::Error)` — Generates `impl From<std::io::Error> for TradingError`, converting I/O failures automatically when using `?`.
- `ConfigParse(#[from] toml::de::Error)` — Generates `impl From<toml::de::Error> for TradingError`, converting TOML deserialization errors automatically.
- `pub type Result<T> = std::result::Result<T, TradingError>;` — Custom crate-wide type alias defaulting error parameter `E` to `TradingError`.

**Compared to your attempt:**
- **Great Job!**: Your implementation in `src/errors.rs` successfully derived `#[from]` for `std::io::Error` and `toml::de::Error` and declared the `Result<T>` type alias! (Note: change `[0]` to `{0}` in `ConfigParse` display string so `thiserror` formats the error message dynamically instead of printing literal `[0]`).

---

### Solution 1.6-1 — `User` Domain Model & Password Hashing (`uuid`, `sha2`, `chrono`)

**Reference Implementation:**
```rust
// Cargo.toml:
// [dependencies]
// uuid = { version = "1.6", features = ["v4"] }
// sha2 = "0.10"
// chrono = "0.4"

// src/user.rs (or src/users.rs):
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, password: &str) -> Self {
        let id = Uuid::new_v4();
        let password_hash = Self::hash_password(password);
        let created_at = Utc::now();

        User {
            id,
            username,
            password_hash,
            created_at,
        }
    }

    pub fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_password(&self, password: &str) -> bool {
        Self::hash_password(password) == self.password_hash
    }
}
```

**Line-by-Line Breakdown:**
- `Uuid::new_v4()` — Generates a 128-bit cryptographically unique user identity identifier.
- `Sha256::new()` & `hasher.update(...)` — Feeds password bytes into the SHA-256 cryptographic hash function.
- `format!("{:x}", hasher.finalize())` — Formats the 32-byte binary hash digest as a 64-character lowercase hex string.
- `verify_password(&self, password: &str)` — Computes candidate password hash and verifies equality against stored hash.

**Compared to your attempt:**
- **Exact Match!**: Your implementation in `src/users.rs` correctly integrated `Uuid`, `Sha256`, `Utc`, and password hashing logic!

---

### Solution 1.6-2 — In-Memory `UserManager` & Authentication Service (`HashMap`, Registration, Authentication)

**Reference Implementation:**
```rust
// src/users.rs:
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use crate::errors::{TradingError, Result};

#[derive(Debug, Default)]
pub struct UserManager {
    pub users: HashMap<Uuid, User>,
    pub username_index: HashMap<String, Uuid>,
}

impl UserManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, username: String, password: &str) -> Result<&User> {
        if self.username_index.contains_key(&username) {
            return Err(TradingError::InvalidQuantity {
                message: format!("Username '{}' already exists", username),
            });
        }

        let user = User::new(username.clone(), password);
        let user_id = user.id;

        self.users.insert(user_id, user);
        self.username_index.insert(username, user_id);

        Ok(self.users.get(&user_id).unwrap())
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<&User> {
        let user_id = self.username_index.get(username).ok_or_else(|| {
            TradingError::InvalidQuantity {
                message: "Invalid credentials".into(),
            }
        })?;

        let user = self.users.get(user_id).ok_or_else(|| {
            TradingError::InvalidQuantity {
                message: "User record missing".into(),
            }
        })?;

        if user.verify_password(password) {
            Ok(user)
        } else {
            Err(TradingError::InvalidQuantity {
                message: "Invalid credentials".into(),
            })
        }
    }
}
```

**Line-by-Line Breakdown:**
- `if self.username_index.contains_key(&username)` — Checks for duplicate username before inserting.
- `let user = User::new(username.clone(), password);` — Constructs new `User` with auto-generated `Uuid`, SHA-256 password hash, and `Utc::now()`.
- `self.users.insert(user_id, user);` — Stores primary record in `users` map by `Uuid`.
- `self.username_index.insert(username, user_id);` — Maps username to `Uuid` in secondary fast-lookup index.
- `Ok(self.users.get(&user_id).unwrap())` — Retrieves borrowed reference `&User` from `self.users` to return back to caller.
- `.get(username).ok_or_else(...)` — Converts `Option<&Uuid>` to `Result<&Uuid, TradingError>`.
- `if user.verify_password(password)` — Verifies candidate password against stored SHA-256 hash.

**Compared to your attempt:**
- **What matched:** You wrote the duplicate check `self.username_index.contains_key(&username)` and constructed the `User` struct correctly!
- **What differed:**
  1. In `TradingError::InvalidQuantity { message: ... }`, `TradingError` variants are named struct fields `{ message: ... }`, so you write `TradingError::InvalidQuantity { message: "..." }` instead of tuple `TradingError::InvalidQuantity(...)`.
  2. After creating `user`, you need to store it in both collections using `self.users.insert(user_id, user)` and `self.username_index.insert(username, user_id)`.
  3. In Rust, returning a reference `&User` from `&mut self` requires looking up the stored value from `self.users.get(&user_id)` so the reference lifetime is tied to `&self.users`.

---

### Solution 1.7-1 — Multi-Currency `Wallet` Engine (`HashMap::entry`, Overdraft Protection)

**Reference Implementation:**
```rust
// src/wallet.rs:
use std::collections::HashMap;
use crate::errors::{TradingError, Result};

#[derive(Debug, Default)]
pub struct Wallet {
    pub balances: HashMap<String, u64>,
}

impl Wallet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn deposit(&mut self, currency: String, amount: u64) -> Result<()> {
        *self.balances.entry(currency).or_insert(0) += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, currency: &str, amount: u64) -> Result<()> {
        match self.balances.get_mut(currency) {
            Some(bal) if *bal >= amount => {
                *bal -= amount;
                Ok(())
            }
            Some(bal) => Err(TradingError::InsufficientFunds {
                required: amount,
                available: *bal,
            }),
            None => Err(TradingError::InsufficientFunds {
                required: amount,
                available: 0,
            }),
        }
    }

    pub fn get_balance(&self, currency: &str) -> u64 {
        self.balances.get(currency).copied().unwrap_or(0)
    }
}
```

**Line-by-Line Breakdown:**
- `use std::collections::HashMap;` — Imports standard library `HashMap` (in `std::collections`, with an `s`).
- `*self.balances.entry(currency).or_insert(0) += amount;` — Uses `Entry` API to lookup or initialize balance bucket to 0, dereferencing `*` to add `amount`.
- `match self.balances.get_mut(currency)` — Looks up mutable reference `Option<&mut u64>` for the currency.
- `Some(bal) if *bal >= amount` — Matches when funds are sufficient, deducting `*bal -= amount`.
- `Some(bal)` — Matches when currency exists but funds are insufficient, returning `available: *bal`.
- `None` — Matches when currency is not in map, returning `available: 0`.
- `self.balances.get(currency).copied().unwrap_or(0)` — Converts `Option<&u64>` to `u64`, defaulting missing currencies to 0.

**Compared to your attempt:**
- **What matched:** Your `deposit` method with `*...or_insert(0) += amount` and `get_balance` with `.copied().unwrap_or(0)` were spot-on!
- **What differed:**
  1. Module path: `std::collections::HashMap` instead of `std::collection::HashMap`.
  2. In `withdraw`, `match self.balances.get_mut(currency)` creates inner binding `bal: &mut u64` in `Some(bal)`, which you dereference as `*bal`. In `None`, no inner `bal` exists, so `available: 0`.

---

### Solution 1.7-3 — Wallet Accumulation & Closure Trait Queries (`.sum()`, Turbofish `::<>`, `Fn`)

**Reference Implementation:**
```rust
// src/wallet.rs:
impl Wallet {
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
```

**Line-by-Line Breakdown:**
- `self.balances.values().sum::<u64>()` — Calls `.sum::<u64>()` with Turbofish syntax to sum all balances in `HashMap`.
- `where F: Fn(&TransactionRecord) -> bool` — Generic closure trait bound accepting immutable references to `TransactionRecord`.
- `self.history.iter().filter(|rec| predicate(rec))` — Applies closure predicate to filter transaction history records.
- `.cloned().collect::<Vec<_>>()` — Clones matching records and uses Turbofish `.collect::<Vec<_>>()` to return a `Vec<TransactionRecord>`.

**Compared to your attempt:**
- **Great Job!**: Your `total_balance` implementation with `self.balances.values().sum::<u64>()` was 100% exact! For `filter_transactions`, completing the method body with `self.history.iter().filter(|rec| predicate(rec)).cloned().collect::<Vec<_>>()` makes it compile cleanly!

---

### Solution 1.8-1 — Portfolio Holdings & Weighted Average Cost Basis (`Position`, `unrealized_pnl`)


**Reference Implementation:**
```rust
// src/portfolio.rs:
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
```

**Line-by-Line Breakdown:**
- `let total_cost = (self.quantity * self.avg_cost) + (add_qty * buy_price);` — Computes total dollars invested across existing holdings and new buy fill.
- `self.avg_cost = total_cost / total_qty;` — Divides total cost by new total quantity to obtain weighted average cost basis per unit.
- `self.quantity * (current_price - self.avg_cost)` — Computes dollar gain/loss relative to current market price.

**Compared to your attempt:**
- **Exact Match!**: Your implementation in `src/portfolio.rs` correctly calculated weighted average cost basis and unrealized P&L!

---

### Solution 1.8-2 — `Portfolio` Tracker Engine & Custom Sorting (`HashMap`, `sort_by`, `PartialOrd`)

**Reference Implementation:**
```rust
// src/portfolio.rs:
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug, Default)]
pub struct Portfolio {
    pub positions: HashMap<String, Position>,
}

impl Portfolio {
    pub fn new() -> Self {
        Self::default()
    }

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
}
```

**Line-by-Line Breakdown:**
- `.and_modify(|pos| pos.update(quantity, price))` — Updates existing `Position` in-place when symbol exists.
- `.or_insert_with(|| Position::new(symbol, quantity, price))` — Lazily constructs new `Position` when symbol is vacant.
- `positions.sort_by(|a, b| ...)` — Sorts position vector in descending order by unrealized P&L.
- `b.unrealized_pnl(price_b).partial_cmp(&a.unrealized_pnl(price_a)).unwrap_or(Ordering::Equal)` — Safely handles floating-point comparisons.

**Compared to your attempt:**
- **Exact Match!**: Your implementation in `src/portfolio.rs` correctly used `.and_modify().or_insert_with()` and implemented descending P&L sorting!

---

### Solution 1.8-3 — `BTreeMap` Portfolio View, Advanced Iterator Chains & `Display` Trait (`.zip()`, `.enumerate()`, `.flat_map()`, `.chain()`, `fmt::Display`)

**Reference Implementation:**
```rust
// src/portfolio.rs:
use std::collections::{HashMap, BTreeMap};
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
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

    pub fn add_position(&mut self, symbol: String, quantity: f64, price: f64) {
        self.positions
            .entry(symbol.clone())
            .and_modify(|pos| pos.update(quantity, price))
            .or_insert_with(|| Position::new(symbol, quantity, price));
    }

    pub fn add_to_sorted(&mut self, symbol: String, quantity: f64, price: f64) {
        self.add_position(symbol.clone(), quantity, price);
        self.sorted_holdings
            .entry(symbol.clone())
            .and_modify(|pos| pos.update(quantity, price))
            .or_insert_with(|| Position::new(symbol, quantity, price));
    }

    pub fn portfolio_report(&self, _current_prices: &HashMap<String, f64>) -> Vec<String> {
        let lines = self.sorted_holdings.values().enumerate().map(|(idx, pos)| {
            format!("# {}: {}", idx + 1, pos)
        });
        let summary = std::iter::once(format!("Total Positions: {}", self.sorted_holdings.len()));
        lines.chain(summary).collect::<Vec<String>>()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:.2} shares @ avg ${:.2}", self.symbol, self.quantity, self.avg_cost)
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
```

**Line-by-Line Breakdown:**
- `pub sorted_holdings: BTreeMap<String, Position>` — BTreeMap guarantees keys (`symbol`) are stored and iterated in sorted alphabetical order.
- `self.sorted_holdings.entry(symbol.clone()).and_modify(...).or_insert_with(...)` — Applies the exact same entry mutation pattern to `sorted_holdings` as `positions`.
- `self.sorted_holdings.values().enumerate().map(|(idx, pos)| format!("# {}: {}", idx + 1, pos))` — Iterates sorted positions with 0-based indices, formats each using `Position`'s `Display` implementation.
- `lines.chain(summary).collect::<Vec<String>>()` — Concatenates formatted position lines with summary footer using `.chain()`.
- `impl fmt::Display for Position` — Custom `Display` formatting using `write!(f, "{}: {:.2} shares @ avg ${:.2}", ...)` for human-readable output.
- `impl fmt::Display for Portfolio` — Custom `Display` formatting using `writeln!(f, ...)?` over enumerated BTreeMap values.

**Compared to your attempt:**
- **Great Start!**: You correctly added `sorted_holdings: BTreeMap<String, Position>` to `Portfolio`, added imports for `BTreeMap` and `fmt`, and started `add_to_sorted`!
- **Key Completion**: `add_to_sorted` needs `.entry(symbol.clone()).and_modify(|pos| pos.update(quantity, price)).or_insert_with(|| Position::new(symbol, quantity, price))`, and `portfolio_report` uses `.enumerate()` + `.chain()`.

---

### Solution 1.9-1 — Newtype `OrderId` & `Order` Domain State Machine (`OrderId`, `OrderSide`, `OrderStatus`)

**Reference Implementation:**
```rust
// src/orders.rs:
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub symbol: String,
    pub side: OrderSide,
    pub qty: u64,
    pub price: u64,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}

impl Order {
    pub fn new(id: u64, symbol: String, side: OrderSide, qty: u64, price: u64) -> Self {
        Order {
            id: OrderId(id),
            symbol,
            side,
            qty,
            price,
            status: OrderStatus::Pending,
            created_at: Utc::now(),
        }
    }

    pub fn cancel(&mut self) -> bool {
        if self.status == OrderStatus::Pending {
            self.status = OrderStatus::Cancelled;
            true
        } else {
            false
        }
    }
}
```

**Line-by-Line Breakdown:**
- `pub struct OrderId(pub u64);` — Wraps `u64` in a type-safe tuple struct.
- `pub enum OrderSide { Buy, Sell }` — Closed domain set for order directions.
- `pub enum OrderStatus { Pending, Filled, Cancelled, Rejected }` — Order lifecycle states.
- `if self.status == OrderStatus::Pending` — Enforces state machine transition rules so only `Pending` orders can transition to `Cancelled`.

**Compared to your attempt:**
- **Exact Match!**: Your implementation in `src/orders.rs` cleanly defined all domain types and state machine transition rules!

---

### Solution 1.9-2 — The Builder Pattern for Order Creation (`OrderBuilder`, Method Chaining, Validation)

**Reference Implementation:**
```rust
// src/orders.rs:
use crate::errors::TradingError;

#[derive(Debug, Default)]
pub struct OrderBuilder {
    pub symbol: Option<String>,
    pub side: Option<OrderSide>,
    pub qty: Option<u64>,
    pub price: Option<u64>,
}

impl OrderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn side(mut self, side: OrderSide) -> Self {
        self.side = Some(side);
        self
    }

    pub fn qty(mut self, qty: u64) -> Self {
        self.qty = Some(qty);
        self
    }

    pub fn price(mut self, price: u64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn build(self, id: u64) -> Result<Order, TradingError> {
        let symbol = self.symbol.ok_or_else(|| TradingError::InvalidOrder {
            message: "Missing symbol".into(),
        })?;
        if symbol.is_empty() {
            return Err(TradingError::InvalidOrder {
                message: "Missing symbol".into(),
            });
        }

        let side = self.side.ok_or_else(|| TradingError::InvalidOrder {
            message: "Missing order side".into(),
        })?;

        let qty = self.qty.ok_or_else(|| TradingError::InvalidOrder {
            message: "Quantity must be greater than zero".into(),
        })?;
        if qty == 0 {
            return Err(TradingError::InvalidOrder {
                message: "Quantity must be greater than zero".into(),
            });
        }

        let price = self.price.ok_or_else(|| TradingError::InvalidOrder {
            message: "Price must be greater than zero".into(),
        })?;
        if price == 0 {
            return Err(TradingError::InvalidOrder {
                message: "Price must be greater than zero".into(),
            });
        }

        Ok(Order::new(id, symbol, side, qty, price))
    }
}
```

**Line-by-Line Breakdown:**
- `pub fn symbol(mut self, symbol: String) -> Self` — Fluent method chaining returning `Self`.
- `.ok_or_else(...)` — Converts missing `Option` values into structured `TradingError::InvalidOrder` errors.
- `if qty == 0` / `if price == 0` / `if symbol.is_empty()` — Validates domain bounds before constructing `Order`.

**Compared to your attempt:**
- **Exact Match!**: Your implementation in `src/orders.rs` cleanly implemented method chaining, option unwrapping, and zero/empty bounds checks!

---

### Solution 1.9-3 — Data-Bearing Enums (`OrderType`), Auto-Incrementing IDs (`OrderId`), & `OrderManager` Query Engine (`OrderType`, `OrderManager`, `.filter()`)

**Reference Implementation:**
```rust
// src/orders.rs:
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit { limit_price: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: OrderId,
    pub symbol: String,
    pub side: OrderSide,
    pub qty: u64,
    pub price: u64,
    pub status: OrderStatus,
    pub order_type: OrderType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OrderManager {
    pub next_id: u64,
    pub orders: Vec<Order>,
}

impl OrderManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            orders: Vec::new(),
        }
    }

    pub fn submit(&mut self, symbol: String, side: OrderSide, order_type: OrderType, qty: u64) -> OrderId {
        let id = self.next_id;
        self.next_id += 1;
        let price = match order_type {
            OrderType::Market => 0,
            OrderType::Limit { limit_price } => limit_price,
        };
        let mut order = Order::new(id, symbol, side, qty, price);
        order.order_type = order_type;
        self.orders.push(order.clone());
        OrderId(id)
    }

    pub fn cancel(&mut self, id: OrderId) -> bool {
        if let Some(order) = self.orders.iter_mut().find(|o| o.id == id) {
            order.cancel()
        } else {
            false
        }
    }

    pub fn get_pending_orders(&self) -> Vec<Order> {
        self.orders
            .iter()
            .filter(|o| o.status == OrderStatus::Pending)
            .cloned()
            .collect()
    }

    pub fn filter_by_symbol(&self, symbol: &str) -> Vec<Order> {
        self.orders
            .iter()
            .filter(|o| o.symbol == symbol)
            .cloned()
            .collect()
    }
}
```

**Line-by-Line Breakdown:**
- `pub enum OrderType { Market, Limit { limit_price: u64 } }` — Defines data-bearing enum where `Limit` holds its target limit price.
- `let id = self.next_id; self.next_id += 1;` — Auto-increments next ID sequentially for each submitted order.
- `if let Some(order) = self.orders.iter_mut().find(|o| o.id == id)` — Finds mutable reference to order in vector by `OrderId` and invokes `.cancel()`.
- `self.orders.iter().filter(|o| o.status == OrderStatus::Pending).cloned().collect()` — Filters vector for pending orders and returns owned `Vec<Order>`.

**Compared to your attempt:**
- **Great Job!**: You correctly defined `OrderType::Limit { limit_price: u64 }`, added `order_type: OrderType` to `Order`, created `OrderManager`, and initialized `next_id: 1` and `orders: Vec::new()`!
- **Key Completion**: Notice spelling of `Deserialize` (typo `Deseiralize` in import), and complete `submit`, `cancel`, `get_pending_orders`, and `filter_by_symbol` methods!

---

### Solution 1.10-1 — Domain Model Serde Derive & Storage Persistence Engine (`Serialize`, `Deserialize`, `save`, `load`)

**Reference Implementation:**
```rust
// src/storage.rs:
use std::fs;
use std::path::Path;
use serde::{Serialize, de::DeserializeOwned};
use crate::errors::TradingError;

pub struct StorageEngine;

impl StorageEngine {
    pub fn save_json<T: Serialize>(path: &Path, data: &T) -> Result<(), TradingError> {
        let json_str = serde_json::to_string_pretty(data)?;
        fs::write(path, json_str)?;
        Ok(())
    }

    pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T, TradingError> {
        let json_str = fs::read_to_string(path)?;
        let data = serde_json::from_str::<T>(&json_str)?;
        Ok(data)
    }
}
```

**Line-by-Line Breakdown:**
- `path: &Path` — Borrows file paths flexibly (accepts `&PathBuf`, `&Path`, `Path::new(...)`).
- `serde_json::to_string_pretty(data)?` — Formats data to pretty-printed JSON string.
- `serde_json::from_str::<T>(&json_str)?` — Deserializes JSON text into target type `T`.
- `fs::write` / `fs::read_to_string` — Disk I/O operations with `?` desugaring into `TradingError`.

**Compared to your attempt:**
- **Exact Match!**: Your implementation in `src/storage.rs` cleanly implemented `save_json` and `load_json` with generic type bounds and `&Path` reference parameters!

---

### Solution 1.10-2 — Domain Struct Serde Derives & Round-Trip Persistence Testing (`#[derive(Serialize, Deserialize)]`, `#[test]`)

**Reference Implementation:**
```rust
// src/storage.rs unit test block:
#[cfg(test)]
mod tests {
    use super::*;
    use crate::portfolio::{Portfolio, Position};
    use std::path::PathBuf;

    #[test]
    fn test_storage_rountrip() {
        let mut portfolio = Portfolio::new();
        portfolio.add_position("BTC".into(), 1.5, 40000.0);

        let test_path = PathBuf::from("test_portfolio.json");

        StorageEngine::save_json(&test_path, &portfolio).unwrap();

        let load_portfolio: Portfolio = StorageEngine::load_json(&test_path).unwrap();

        assert_eq!(load_portfolio.positions.get("BTC"), portfolio.positions.get("BTC"));
        std::fs::remove_file(&test_path).unwrap();
    }
}
```

**Line-by-Line Breakdown:**
- `#[cfg(test)]` — Compiles test module only when running `cargo test`.
- `StorageEngine::save_json(&test_path, &portfolio)` — Serializes portfolio to JSON file on disk.
- `StorageEngine::load_json(&test_path)` — Deserializes JSON string back into a `Portfolio` struct.
- `assert_eq!(...)` — Confirms that loaded position matches original in-memory data.
- `std::fs::remove_file(&test_path)` — Cleans up temporary test file.

**Compared to your attempt:**
- **Exact Match!**: Your implementation in `src/storage.rs` cleanly implemented the round-trip test and file cleanup!

---

### Solution 1.10-3 — Serde Field Attributes, Struct Lifetimes, `PathBuf` & Atomic Storage Writes (`#[serde(default)]`, `StorageMetadata<'a>`, `save_json_atomic`)

**Reference Implementation:**
```rust
// src/storage.rs:
use std::fs;
use std::path::Path;
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use crate::errors::TradingError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageMetadata<'a> {
    #[serde(borrow)]
    pub filename: &'a Path,
    pub author: &'a str,
    #[serde(default)]
    pub version: u32,
    #[serde(skip)]
    pub runtime_cache: u64,
}

pub struct StorageEngine;

impl StorageEngine {
    pub fn save_json<T: Serialize>(path: &Path, data: &T) -> Result<(), TradingError> {
        let json_str = serde_json::to_string_pretty(data)?;
        fs::write(path, json_str)?;
        Ok(())
    }

    pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T, TradingError> {
        let json_str = fs::read_to_string(path)?;
        let data = serde_json::from_str::<T>(&json_str)?;
        Ok(data)
    }

    pub fn load_json_or_default<T: DeserializeOwned + Default>(path: &Path) -> T {
        Self::load_json::<T>(path).unwrap_or_default()
    }

    pub fn save_json_atomic<T: Serialize>(path: &Path, data: &T) -> Result<(), TradingError> {
        let tmp_path = path.with_extension("tmp");
        Self::save_json(&tmp_path, data)?;
        fs::rename(&tmp_path, path)?;
        Ok(())
    }
}
```

**Line-by-Line Breakdown:**
- `pub struct StorageMetadata<'a>` — Struct declaring explicit lifetime `'a` for borrowed slice fields `&'a Path` and `&'a str`.
- `#[serde(rename_all = "camelCase")]` — Serde container attribute converting field names to `camelCase` in serialized JSON.
- `#[serde(borrow)]` — Borrow attribute allowing Serde to borrow slice references from input JSON without allocating string copies.
- `#[serde(default)]` — Field attribute using `Default::default()` for missing JSON fields during deserialization.
- `#[serde(skip)]` — Excludes field from serialization and deserialization entirely.
- `Self::load_json::<T>(path).unwrap_or_default()` — Calls `load_json` and returns `T::default()` gracefully if missing or corrupted.
- `let tmp_path = path.with_extension("tmp");` — Uses `PathBuf` extension method to construct temporary target path.
- `fs::rename(&tmp_path, path)?` — Atomically replaces the target file with the temp file in a single OS operation.

**Compared to your attempt:**
- **Great Start!**: You correctly defined `StorageMetadata<'a>` with all Serde attributes (`rename_all`, `borrow`, `default`, `skip`), and declared `load_json_or_default` inside `StorageEngine`!
- **Key Completion**: `load_json_or_default` can be written in 1 line using `.unwrap_or_default()`, and `save_json_atomic` uses `path.with_extension("tmp")` and `fs::rename`.

---

### Solution 1.11-1 — Realized & Unrealized P&L Accounting Engine (`PositionTracker`, `Order` Fill Execution)

**Reference Implementation:**
```rust
// src/tracker.rs:
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
                    let pnl = (price - pos.avg_cost_basis) * qty;
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
            let market_price = current_prices.get(&pos.symbol).copied().unwrap_or(pos.avg_cost_basis);
            total += pos.unrealized_pnl(market_price);
        }

        total
    }
}
```

**Line-by-Line Breakdown:**
- `match side` — Pattern matches order fill direction (`Buy` vs `Sell`).
- `self.positions.entry(...).and_modify(...).or_insert_with(...)` — Atomically updates position quantity and weighted cost basis on Buy fills.
- `(price - pos.avg_cost_basis) * qty` — Calculates locked-in realized P&L when selling shares.
- `pos.unrealized_pnl(market_price)` — Calls the `Position` method to compute paper profit/loss against live prices.
- `total` — Returns the combined sum of Realized P&L + Unrealized P&L.

**Compared to your attempt:**
- **Great Effort!**: Your logic for `OrderSide::Sell` was spot on (`if let Some(pos) = ...`, calculating `pnl`, updating `realized_pnl`, and removing empty positions)!
- **Key Adjustments Needed**:
  1. For `Buy`, `self.positions.entry(...)` returns an `Entry` enum — calling `.and_modify()` and `.or_insert_with()` properly executes the update/insert.
  2. For `total_pnl`, `pos.unrealized_pnl` is a method (`pos.unrealized_pnl(*price)`), so it needs method arguments `(*price)` and parenthetic call syntax `()`.
  3. Return `total` at the end of `total_pnl`.

---

### Solution 1.11-2 — Shared Position Mutability & Unit Test Suite (`Rc<RefCell<Position>>`, `#[test]`)

**Reference Implementation:**
```rust
// src/tracker.rs:
#[cfg(test)]
mod tests {
    use super::*;
    use crate::orders::OrderSide;

    #[test]
    fn test_position_tracker_buy_sell_pnl() {
        let mut tracker = PositionTracker::new();

        // 1. Buy 2.0 BTC @ $40,000
        tracker.process_fill(OrderSide::Buy, "BTC", 2.0, 40000.0);
        assert_eq!(tracker.positions.get("BTC").unwrap().quantity, 2.0);

        // 2. Sell 1.0 BTC @ $50,000 (locks in $10,000 realized P&L)
        tracker.process_fill(OrderSide::Sell, "BTC", 1.0, 50000.0);
        assert_eq!(tracker.realized_pnl, 10000.0);
        assert_eq!(tracker.positions.get("BTC").unwrap().quantity, 1.0);

        // 3. Verify total_pnl at market price $55,000
        let prices = HashMap::from([("BTC".to_string(), 55000.0)]);
        assert_eq!(tracker.total_pnl(&prices), 25000.0);
    }
}
```

**Line-by-Line Breakdown:**
- `#[cfg(test)] mod tests` — Defines inner test module conditionally compiled only for test builds.
- `use super::*;` — Imports outer module scope symbols (`PositionTracker`, `Position`) into test module.
- `tracker.process_fill(OrderSide::Sell, "BTC", 1.0, 50000.0)` — Simulates sell trade fill, updating realized P&L and remaining holdings.
- `assert_eq!(tracker.realized_pnl, 10000.0)` — Asserts cash profit locked in from sell trade equals $10,000.
- `let prices = HashMap::from([("BTC".to_string(), 55000.0)])` — Constructs live price map fixture.
- `assert_eq!(tracker.total_pnl(&prices), 25000.0)` — Asserts total mark-to-market portfolio P&L ($10k realized + $15k unrealized) equals $25,000.

**Compared to your attempt:**
- **Great Start!**: You correctly added `mod tests`, imported `OrderSide`, created `PositionTracker`, performed the buy fill, and added `process_fill(OrderSide::Sell, "BTC", 1.0, 50000.0)`!
- **Key Fixes Needed**:
  1. Typo `tracker.position` $\rightarrow$ `tracker.positions` (plural field name).
  2. `assert_eq!(left, right)` requires two arguments (`assert_eq!(tracker.realized_pnl, 10000.0)`).
  3. `HashMap::from([("BTC".to_string(), 55000.0)])` constructs price map to pass into `tracker.total_pnl(&prices)`.

---

### Solution 1.12-1 — Integration Testing & Result-Returning Tests (`tests/integration_test.rs`, `Result<(), String>`)

**Reference Implementation:**
```rust
// tests/integration_test.rs:
use trading_platform::wallet::Wallet;
use trading_platform::orders::{OrderManager, OrderSide};
use trading_platform::tracker::PositionTracker;
use std::collections::HashMap;

#[test]
fn test_end_to_end_trading_flow() -> Result<(), String> {
    // 1. Initialize Wallet and deposit funds
    let mut wallet = Wallet::new();
    wallet.deposit("USD".to_string(), 100_000);
    if wallet.get_balance("USD") != 100_000 {
        return Err("Wallet deposit failed".to_string());
    }

    // 2. Initialize OrderManager and submit a Buy order for BTC
    let mut order_mgr = OrderManager::new();
    let order = order_mgr.submit("BTC".to_string(), OrderSide::Buy, 2.0, 40000.0);
    if order.id != 1 {
        return Err("Order ID auto-increment failed".to_string());
    }

    // 3. Initialize PositionTracker and process buy fill of 2.0 BTC @ $40,000
    let mut tracker = PositionTracker::new();
    tracker.process_fill(OrderSide::Buy, "BTC", 2.0, 40000.0);
    if tracker.positions.get("BTC").ok_or("Missing position")?.quantity != 2.0 {
        return Err("Position fill quantity mismatch".to_string());
    }

    // 4. Verify mark-to-market total P&L at BTC = $45,000
    let prices = HashMap::from([("BTC".to_string(), 45000.0)]);
    if tracker.total_pnl(&prices) != 10000.0 {
        return Err("Total mark-to-market P&L mismatch".to_string());
    }

    Ok(())
}
```

**Line-by-Line Breakdown:**
- `use trading_platform::*` — Imports public library items exported from `src/lib.rs`.
- `fn test_end_to_end_trading_flow() -> Result<(), String>` — Returns `Result<(), String>` so setup failures yield `Err("reason")` instead of panicking.
- `if tracker.total_pnl(&prices) != 10000.0 { return Err(...); }` — Evaluates total paper profit against live $45,000 BTC price map and returns `Err` if it doesn't match expected $10,000.
- `Ok(())` — Returns unit success variant indicating test passed.

**Compared to your attempt:**
- **Excellent Work!**: You wrote 33 lines of clean integration test code including `Wallet`, `OrderManager`, `OrderSide`, `PositionTracker`, and constructing the `HashMap` price map!
- **Key Syntax Fixes Needed**:
  1. Line 13: Remove the extra semicolon inside `return Err("Wallet deposit failed".to_string());`.
  2. Line 2: Import `OrderSide` (`use trading_platform::orders::{OrderManager, OrderSide};`).
  3. Lines 33-35: Complete TODO 2 by checking `if tracker.total_pnl(&prices) != 10000.0 { return Err("Total mark-to-market P&L mismatch".to_string()); }` and returning `Ok(())` at the end!

---

### Solution 1.12-2 — Documentation Testing (`///`) & Panic Verification (`#[should_panic]`)

**Reference Implementation:**
```rust
// src/wallet.rs:
impl Wallet {
    /// Deposits a specified amount of currency into the wallet balance.
    ///
    /// # Example
    /// ```
    /// use trading_platform::wallet::Wallet;
    /// let mut wallet = Wallet::new();
    /// wallet.deposit("USD".to_string(), 500);
    /// assert_eq!(wallet.get_balance("USD"), 500);
    /// ```
    pub fn deposit(&mut self, currency: String, amount: u64) -> Result<()> { ... }
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
```

**Line-by-Line Breakdown:**
- `///` — Doc comment attribute compiled and executed by `cargo test --doc`.
- `#[should_panic(expected = "InsufficientFunds")]` — Asserts that thread panics with error substring `"InsufficientFunds"`.
- `wallet.withdraw("USD", 500).unwrap()` — Attempts withdrawal exceeding balance, unwrapping `Err(TradingError::InsufficientFunds)`, which triggers panic.

**Compared to your attempt:**
---

### Solution 1.13-1 — Sub-Module Tree Organization (`src/models.rs`, `src/models/`) & Re-exports (`pub use`)

**Reference Implementation:**
```rust
// src/models.rs:
pub mod portfolio;
pub mod users;
pub mod wallet;

pub use portfolio::{Portfolio, Position};
pub use users::{User, UserManager};
pub use wallet::{Wallet, TransactionRecord, TransactionType};
```

```rust
// src/lib.rs:
pub mod config;
pub mod errors;
pub mod models;
pub mod orders;
pub mod storage;
pub mod tracker;
pub mod cli;

pub use models::{Portfolio, Position, User, UserManager, Wallet, TransactionRecord, TransactionType};
```

**Line-by-Line Breakdown:**
- `pub mod portfolio;` — Declares child sub-module `src/models/portfolio.rs`.
- `pub mod users;` — Declares child sub-module `src/models/users.rs`.
- `pub mod wallet;` — Declares child sub-module `src/models/wallet.rs`.
- `pub use portfolio::{Portfolio, Position};` — Re-exports domain structs so consumers can import directly from `models::*`.
- `pub use models::{...};` — Top-level re-export in `src/lib.rs` granting zero-cost convenient paths for external users.

**Compared to your attempt:**
- **Exact Match!**: Your implementation in `src/models.rs` and `src/lib.rs` correctly declared all sub-modules and re-exported all domain models cleanly!

---

### Solution 1.13-2 — Services Subtree Organization (`src/services.rs`, `src/services/`) & Re-exports (`pub use`)

**Reference Implementation:**
```rust
// src/services.rs:
pub mod order_manager;
pub mod tracker;

pub use order_manager::{OrderManager, OrderId, OrderSide, OrderType, OrderStatus, Order};
pub use tracker::PositionTracker;
```

```rust
// src/lib.rs:
pub mod config;
pub mod errors;
pub mod models;
pub mod services;
pub mod storage;
pub mod cli;

pub use models::{Portfolio, Position, User, UserManager, Wallet, TransactionRecord, TransactionType};
pub use services::{OrderManager, OrderId, OrderSide, OrderType, OrderStatus, Order, PositionTracker};
```

**Line-by-Line Breakdown:**
- `pub mod order_manager;` — Declares business engine sub-module `src/services/order_manager.rs`.
- `pub mod tracker;` — Declares business engine sub-module `src/services/tracker.rs`.
- `pub use order_manager::{...};` — Re-exports order management engine and order types at `services` level.
- `pub use tracker::PositionTracker;` — Re-exports position tracking engine at `services` level.
- `pub use services::{...};` — Top-level library API re-export facade in `src/lib.rs`.

**Compared to your attempt:**
- **Exact Match!**: You successfully moved `orders.rs` and `tracker.rs` into `src/services/`, declared sub-modules in `src/services.rs`, and re-exported all engines and order types cleanly in `src/lib.rs`!

---

### Solution 1.13-3 — Complete Infrastructure Subtree Refactoring (`src/storage/`, `src/errors/`, `src/cli/`, `src/config/`)

**Reference Implementation:**
```rust
// src/storage.rs:
pub mod engine;
pub use engine::*;

// src/errors.rs:
pub mod trading_errors;
pub use trading_errors::*;

// src/cli.rs:
pub mod parser;
pub use parser::*;

// src/config.rs:
pub mod settings;
pub use settings::*;
```

**Line-by-Line Breakdown:**
- `pub mod engine; pub use engine::*;` — Re-exports storage engine sub-module types at `storage` level.
- `pub mod trading_errors; pub use trading_errors::*;` — Re-exports error enum types at `errors` level.
- `pub mod parser; pub use parser::*;` — Re-exports CLI parser types at `cli` level.
- `pub mod settings; pub use settings::*;` — Re-exports config settings types at `config` level.

**Compared to your attempt:**
- **Exact Match!**: You successfully refactored all 4 infrastructure subtrees, created parent root module files, and re-exported all types in `src/lib.rs`!

---

### Solution 1.14-1 — Crate-Level Documentation (`//!`) & Intra-Doc Links

**Reference Implementation:**
```rust
// src/lib.rs (at line 1 at the very top of the file):
//! # Trading Platform Architecture
//! 
//! A production-grade financial trading ecosystem.
//! 
//! ## Core Domain Subsystems
//! - Domain Models: [`Wallet`], [`Position`]
//! - Business Services: [`OrderManager`], [`PositionTracker`]
```

**Line-by-Line Breakdown:**
- `//!` — Inner doc comment attribute macro documenting the containing `lib.rs` crate root.
- `# Trading Platform Architecture` — Level 1 Markdown heading rendered at top of `cargo doc`.
- `[`Wallet`]` — Short intra-doc link resolving directly to `crate::models::Wallet` without redundant path targets.

**Compared to your attempt:**
- **Exact Match!**: Your implementation in `src/lib.rs` correctly added inner doc comments with short intra-doc links and compiled cleanly with 0 warnings under `cargo doc`!

---

### Solution 1.14-2 — Code Quality Configs (`rustfmt.toml`, `clippy.toml`) & Doc Warnings (`#![warn(missing_docs)]`)

**Reference Implementation:**

```toml
# rustfmt.toml:
max_width = 100
edition = "2024"
tab_spaces = 4
```

```toml
# clippy.toml:
cognitive-complexity-threshold = 25
```

```rust
// src/lib.rs (line 1):
#![warn(missing_docs)]
```

**Line-by-Line Breakdown:**
- `max_width = 100` — Configures rustfmt to wrap code lines exceeding 100 characters.
- `edition = "2024"` — Specifies Rust 2024 edition formatting rules.
- `tab_spaces = 4` — Sets standard 4-space indentations.
- `cognitive-complexity-threshold = 25` — Sets clippy threshold to flag functions exceeding cognitive complexity score of 25.
- `#![warn(missing_docs)]` — Directs rustc to emit compiler warnings for un-documented public items in crate root.

**Compared to your attempt:**
- **Exact Match!**: You successfully created `rustfmt.toml` and `clippy.toml` in project root, and added `#![warn(missing_docs)]` to `src/lib.rs`!

---

### Solution 1.14-3 — Item-Level Lint Control Attributes (`#[allow(...)]`)

**Reference Implementation:**
```rust
// In src/services/tracker.rs inside impl PositionTracker:
    /// Resets all open positions and realized P&L.
    #[allow(dead_code)]
    pub fn clear_positions(&mut self) {
        self.positions.clear();
        self.realized_pnl = 0.0;
    }
```

**Line-by-Line Breakdown:**
- `#[allow(dead_code)]` — Item-level outer attribute suppressing compiler unused code warnings for `clear_positions`.
- `pub fn clear_positions(&mut self)` — Helper method clearing all positions map entries and resetting realized P&L.

**Compared to your attempt:**
- **Exact Match!**: You successfully added `clear_positions` with `#[allow(dead_code)]` to `src/services/tracker.rs`!

---

### Solution 1.14-4 — Code Severity Attributes (`#[deny(...)]` & `#[warn(...)]`)

**Reference Implementation:**

```rust
// File: src/config/settings.rs (above struct Config):
#[warn(missing_docs)]
pub struct Config {
    pub exchange_name: String,
    pub currency: String,
    pub max_order_size: u64,
    pub log_level: String,
}
```

```rust
// File: src/storage/engine.rs (above struct StorageEngine):
#[deny(unused_variables)]
pub struct StorageEngine;
```

**Line-by-Line Breakdown:**
- `#[warn(missing_docs)]` — Outer item-level attribute emitting compiler warnings if `Config` or its fields lack doc comments.
- `#[deny(unused_variables)]` — Outer item-level attribute escalating unused variable warnings inside `StorageEngine` methods into fatal compilation errors.

**Compared to your attempt:**
- **Exact Match!**: You successfully applied `#[warn(missing_docs)]` in `src/config/settings.rs` and `#[deny(unused_variables)]` in `src/storage/engine.rs`!

---











### Solution 1.15-1 — Performance Latency Benchmarking (`std::time::Instant`)

**Reference Implementation:**
```rust
use std::time::Instant;

/// Measures the execution latency of a closure in microseconds.
pub fn benchmark_operation<F, R>(name: &str, op: F) -> (R, u128)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = op();
    let micros = start.elapsed().as_micros();
    println!("[BENCHMARK] {} executed in {} µs", name, micros);
    (result, micros)
}
```

**Line-by-Line Breakdown:**
- `let start = Instant::now();` — Captures the current high-precision monotonic start timestamp.
- `let result = op();` — Executes the closure `op` and binds its returned result `R`.
- `let micros = start.elapsed().as_micros();` — Calculates the elapsed duration since `start` and converts it to microseconds (`u128`).
- `println!("[BENCHMARK] {} executed in {} µs", name, micros);` — Prints the benchmark output banner to the terminal.
- `(result, micros)` — Returns the tuple containing both the closure output `R` and execution latency `u128`.

**Compared to your attempt:**
- **Matches**: You correctly used `Instant::now()`, called `op()`, and referenced `start.elapsed().as_micros()`.
- **Differences / Fixes Needed**:
  1. `let start = Instant::now();` — You called `Instant::now();` without binding it to a variable `start`. Without `let start =`, the variable `start` doesn't exist on line 65.
  2. `let result = op();` — You called `op();` without capturing its return value.
  3. `println!("[BENCHMARK] ...")` — You skipped printing the benchmark timing banner.
  4. Tuple Return: You returned `start.elapsed().as_micros()` directly instead of returning the tuple `(result, micros)` required by signature `-> (R, u128)`.

---

*(Additional solutions will be added as exercises get gated open.)*



















