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

### Exercise 1.7-2 — Transaction Audit History & Iterator Filtering (`TransactionRecord`, `.filter()`, `.collect()`)
**Status:** open
**Goal:** Add `TransactionRecord` struct, store transaction history in `Wallet`, log records during deposits/withdrawals, and implement `get_history` using iterator filter chains.

**Skeleton:**
```rust
// In src/wallet.rs:
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
}

#[derive(Debug, Clone)]
pub struct TransactionRecord {
    pub tx_type: TransactionType,
    pub currency: String,
    pub amount: u64,
    pub timestamp: DateTime<Utc>,
}

// Update Wallet struct:
#[derive(Debug, Default)]
pub struct Wallet {
    pub balances: HashMap<String, u64>,
    pub history: Vec<TransactionRecord>,
}

impl Wallet {
    // TODO(1): In deposit and withdraw methods, push a TransactionRecord into self.history on success.
    // Set timestamp to Utc::now().

    // TODO(2): Implement get_history(&self, currency: &str) -> Vec<TransactionRecord>
    // Filter self.history using `.iter().filter(|tx| tx.currency == currency).cloned().collect()`.
}
```

**Constraints:** Log all successful deposits and withdrawals into `self.history`, and use an iterator chain (`.iter().filter(...).cloned().collect()`) for filtering.
**Hints used:** 0/3
**My attempt:** *(paste here when ready, even if broken/partial)*

---


## Solved

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

