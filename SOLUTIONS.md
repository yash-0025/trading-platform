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

*(Additional solutions will be added as exercises get gated open.)*




