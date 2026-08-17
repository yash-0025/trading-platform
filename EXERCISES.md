# ✍️ EXERCISES.md — Hands-On Exercises (Skeleton + Hints)

> Every hands-on part of a module lands here as an exercise BEFORE any full solution exists.
> You write the code. The AI gives you a skeleton with the taught concept blanked out, plus tiered hints on request.
> Full solutions live in `SOLUTIONS.md` and are gated — see `.agents/workflow/next.md` STEP 3.5 for the exact flow.

**Status legend:** `open` (not attempted yet) · `attempted` (you've had a go, not yet checked) · `solved` (compared against SOLUTIONS.md and understood)

**Rules for this file**
1. Skeleton code only has the CURRENT concept blanked out (`todo!()` / `// TODO(n)`). Everything else — imports, unrelated function bodies, struct defs — is pre-filled so you're never blocked by unrelated syntax.
2. Hints are tiered (conceptual → structural → near-solution) and only given one at a time, on request. "Hints used" gets bumped each time.
3. Nobody (including the AI) opens `SOLUTIONS.md` for an exercise until you've made an actual attempt AND asked to see it.

---

## Entry Format

```
### Exercise <module#>.<n> — <short title>
**Status:** open
**Goal:** one sentence — what this proves you can do.

**Skeleton:**
​```rust
// pre-filled context
fn example() -> Result<(), TradingError> {
    // TODO(1): ...
    todo!()
}
​```

**Constraints:** what NOT to change (signatures, imports, deps).
**Hints used:** 0/3
**My attempt:** *(paste here when ready, even if broken/partial)*
```

---

## Open / In-Progress

### Exercise 1.11-1 — Realized & Unrealized P&L Accounting Engine (`PositionTracker`, `Order` Fill Execution)
**Status:** open
**Goal:** Create `src/tracker.rs` with `PositionTracker` storing `realized_pnl: f64` and positions map, supporting `process_fill(&mut self, side: OrderSide, symbol: &str, fill_qty: f64, fill_price: f64)` to calculate realized P&L on closing/partial-closing fills and update weighted cost basis on opening fills.

**Skeleton:**
```rust
// Create src/tracker.rs:
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

    // TODO(1): Implement process_fill(&mut self, side: OrderSide, symbol: &str, qty: f64, price: f64)
    // 1. Check existing position in self.positions.get_mut(symbol).
    // 2. If Buy (opening long / adding position):
    //    pos.update(qty, price) or insert new Position if vacant.
    // 3. If Sell (closing/reducing long position):
    //    Calculate realized P&L: (price - pos.avg_cost_basis) * qty_sold.
    //    Add calculated value to self.realized_pnl.
    //    Reduce pos.quantity by qty_sold.
    //    If pos.quantity <= 0.0, remove position from self.positions map.

    // TODO(2): Implement total_pnl(&self, current_prices: &HashMap<String, f64>) -> f64
    // Sum self.realized_pnl + all unrealized P&L from self.positions.
}
```

**Constraints:** Maintain exact realized P&L accounting when closing positions, and update weighted average cost basis when opening positions.
**Hints used:** 0/3
**My attempt:** *(paste here when ready, even if broken/partial)*

---

## Solved

### Exercise 1.10-2 — Domain Struct Serde Derives & Round-Trip Persistence Testing (`#[derive(Serialize, Deserialize)]`, `#[test]`)
**Status:** solved
**Goal:** Add `#[derive(Serialize, Deserialize)]` to all domain types (`User`, `UserManager`, `Wallet`, `TransactionRecord`, `TransactionType`, `Position`, `Portfolio`, `OrderId`, `OrderSide`, `OrderStatus`, `Order`), and write a unit test in `src/storage.rs` verifying round-trip JSON serialization.
**Note:** Solved across domain models and `src/storage.rs`. Checked against `SOLUTIONS.md` — exact match on `test_storage_rountrip` and Serde derives.


### Exercise 1.10-1 — Domain Model Serde Derive & Storage Persistence Engine (`Serialize`, `Deserialize`, `save`, `load`)
**Status:** solved
**Goal:** Create `src/storage.rs` with `StorageEngine` supporting `save_json<T: Serialize>(path: &Path, data: &T)` and `load_json<T: DeserializeOwned>(path: &Path) -> Result<T, TradingError>`.
**Note:** Solved in `src/storage.rs`. Checked against `SOLUTIONS.md` — exact match on `save_json` and `load_json` with `&Path` slice references and generic type bounds.



### Exercise 1.9-2 — The Builder Pattern for Order Creation (`OrderBuilder`, Method Chaining, Validation)
**Status:** solved
**Goal:** In `src/orders.rs`, implement `OrderBuilder` struct supporting method chaining (`symbol`, `side`, `qty`, `price`) and `build(self, id: u64) -> Result<Order, TradingError>`.
**Note:** Solved in `src/orders.rs`. Checked against `SOLUTIONS.md` — exact match on `OrderBuilder` method chaining, zero/empty checks, and `TradingError::InvalidOrder` return.


### Exercise 1.9-1 — Newtype `OrderId` & `Order` Domain State Machine (`OrderId`, `OrderSide`, `OrderStatus`)
**Status:** solved
**Goal:** Create `src/orders.rs` with `OrderId` newtype struct, `OrderSide` enum (`Buy`, `Sell`), `OrderStatus` enum (`Pending`, `Filled`, `Cancelled`, `Rejected`), and `Order` struct with `new` and `cancel` state transition methods.
**Note:** Solved in `src/orders.rs`. Checked against `SOLUTIONS.md` — exact match on `OrderId`, `OrderSide`, `OrderStatus`, and `cancel`.


### Exercise 1.8-2 — `Portfolio` Tracker Engine & Custom Sorting (`HashMap`, `sort_by`, `PartialOrd`)
**Status:** solved
**Goal:** In `src/portfolio.rs`, implement `Portfolio` storing `positions: HashMap<String, Position>`, providing `add_position`, `get_position`, and `get_sorted_positions`.
**Note:** Solved in `src/portfolio.rs`. Checked against `SOLUTIONS.md` — exact match on `and_modify().or_insert_with()` entry upserts and P&L `sort_by`.


### Exercise 1.8-1 — Portfolio Holdings & Weighted Average Cost Basis (`Position`, `unrealized_pnl`)
**Status:** solved
**Goal:** Create `src/portfolio.rs` and implement `Position` with `update` (recalculating quantity and weighted average cost basis) and `unrealized_pnl(market_price)`.
**Note:** Solved in `src/portfolio.rs`. Checked against `SOLUTIONS.md` — exact match on `Position`, `update` weighted average cost basis, and `unrealized_pnl`.


### Exercise 1.7-2 — Transaction Audit History & Iterator Filtering (`TransactionRecord`, `.filter()`, `.collect()`)
**Status:** solved
**Goal:** Add `TransactionRecord` struct, store transaction history in `Wallet`, log records during deposits/withdrawals, and implement `get_history` using iterator filter chains.
**Note:** Solved in `src/wallet.rs`. Checked against `SOLUTIONS.md` — exact match on `TransactionRecord`, `.cloned()`, and timestamp logging.


### Exercise 1.7-1 — Multi-Currency `Wallet` Engine (`HashMap::entry`, Overdraft Protection)
**Status:** solved
**Goal:** Create `src/wallet.rs` and implement `Wallet` supporting `deposit`, `withdraw`, and `get_balance` using the `HashMap` Entry API and overdraft protection.
**Note:** Solved in `src/wallet.rs`. Checked against `SOLUTIONS.md` — compared attempt, explained `Option<&mut u64>` destructuring in `match` branches.

### Exercise 1.6-2 — In-Memory `UserManager` & Authentication Service (`HashMap`, Registration, Authentication)
**Status:** solved
**Goal:** In `src/users.rs`, implement `UserManager` storing `users: HashMap<Uuid, User>` and `username_index: HashMap<String, Uuid>`, providing `register` and `authenticate` methods.
**Note:** Solved in `src/users.rs`. Checked against `SOLUTIONS.md` — compared attempt, explained map insertion and reference return mechanics.


### Exercise 1.6-1 — `User` Domain Model & Password Hashing (`uuid`, `sha2`, `chrono`)
**Status:** solved
**Goal:** Add `uuid`, `sha2`, and `chrono` to `Cargo.toml`, create `src/user.rs`, and implement `User::new`, `hash_password`, and `verify_password`.
**Note:** Solved in `src/users.rs`. Checked against `SOLUTIONS.md` — exact match on `Uuid`, `Sha256`, `Utc`, and password hashing logic.



### Exercise 1.5-2 — Automatic Error Conversions (`#[from]`) & Custom `Result` Type Alias
**Status:** solved
**Goal:** Add `#[from]` conversions for `std::io::Error` and `toml::de::Error` in `TradingError`, and define `pub type Result<T> = std::result::Result<T, TradingError>;`.
**Note:** Solved in `src/errors.rs`. Checked against `SOLUTIONS.md` — exact match on `#[from]` attributes and custom `Result<T>` type alias.

### Exercise 1.5-1 — Custom `TradingError` Enum (`thiserror`, `#[derive(Error)]`)
**Status:** solved
**Goal:** Add `thiserror = "1.0"` to `Cargo.toml`, create `src/errors.rs`, and define the `TradingError` enum with formatted error variants.
**Note:** Solved in `src/errors.rs`. Checked against `SOLUTIONS.md` — exact match on `thiserror` attributes and variants.



### Exercise 1.4-2 — Command Parsing & Dispatching (`Cli::parse()`, `match cli.command`)
**Status:** solved
**Goal:** In `src/main.rs`, parse command line args via `Cli::parse()` and dispatch each subcommand variant (`Buy`, `Sell`, `Balance`, `Orders`) using a `match` statement.
**Note:** Solved in `src/main.rs`. Checked against `SOLUTIONS.md` — exact match on `Cli::parse()` and exhaustive `match` dispatching.

### Exercise 1.4-1 — CLI Commands & Subcommands (`clap`, `Parser`, `Subcommand`)
**Status:** solved
**Goal:** Add `clap = { version = "4.4", features = ["derive"] }` to `Cargo.toml`, build `src/cli.rs`, and define `Cli` struct with subcommands (`Buy`, `Sell`, `Balance`, `Orders`).
**Note:** Solved in `src/cli.rs` & `Cargo.toml`. Checked against `SOLUTIONS.md` — exact match on `clap` derive attributes and `Commands` enum variants.


### Exercise 1.3-3 — Serde TOML Deserialization (`serde`, `toml::from_str`)
**Status:** solved
**Goal:** Add `serde` and `toml` dependencies to `Cargo.toml`, derive `Deserialize` on `Config`, and parse raw TOML strings in `from_file_or_env`.
**Note:** Solved in `src/config.rs` & `Cargo.toml`. Checked against `SOLUTIONS.md` — exact match on `serde` derives and `toml::from_str::<Config>(&contents)` deserialization.

### Exercise 1.3-2 — File Parsing & Layered Fallback (`config.toml`, `std::fs::read_to_string`)
**Status:** solved
**Goal:** Implement `Config::from_file_or_env(path: &str)` to attempt reading `config.toml` before falling back to env vars / defaults.
**Note:** Solved in `src/config.rs`. Checked against `SOLUTIONS.md` — exact match on `match std::fs::read_to_string(path)` file fallback logic.



### Exercise 1.3-1 — Config Struct & Env Fallback (`Option<T>` & `unwrap_or_else`)
**Status:** solved
**Goal:** Build a `Config` struct representing exchange settings and implement `from_env_or_default()` using `Option` combinators.
**Note:** Solved in `src/config.rs`. Checked against `SOLUTIONS.md` — learned `unwrap_or_else` vs `match` syntax and placing method bodies inside functions.


### Exercise 1.2-3 — Impl Blocks, Constructors (`Self::new()`), and Method Mutability (`&mut self`)
**Status:** solved
**Goal:** Implement the `new()` constructor and `fill()` method on `Order`.
**Note:** Solved in `src/models.rs`. Checked against `SOLUTIONS.md` — exact match on struct field init shorthand, `&mut self` mutation, and `&self` status checking.

### Exercise 1.2-2 — Structs & Newtype Pattern (`Price`, `Quantity`, `Order`)
**Status:** solved
**Goal:** Implement the newtype wrapper types `Price` and `Quantity`, and construct the `Order` struct.
**Note:** Solved in `src/models.rs`. Checked against `SOLUTIONS.md` — fixed `ParitalEq` typo to `PartialEq`.

### Exercise 1.2-1 — Defining Core Trading Enums (`Side` and `OrderType`)
**Status:** solved
**Goal:** Define the `Side` and `OrderType` enums to represent trading sides and order types as algebraic data types.
**Note:** Solved in `src/models.rs`. Checked against `SOLUTIONS.md` — learned CamelCase convention for variants (`StopLoss`).

