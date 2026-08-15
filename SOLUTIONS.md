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

*(Additional solutions will be added as exercises get gated open.)*












