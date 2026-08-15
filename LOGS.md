# 🗂️ LOGS.md — File Change Log

> Pure **file-diff log** — every file created, modified, or deleted in this workspace gets an entry here. Newest first.
> This is NOT a conversation log (that's `PROMPTS.md`). This is a `git log --patch` equivalent.

### Rules
1. **Every file change gets logged.** Any file created, modified, or deleted — source code, configs, governance files, tracking files — gets an entry.
2. **Show the FULL actual diff.** Every entry must include a `diff` code block showing the **exact lines** that were added (`+`), removed (`-`), or kept for context (` `). No abbreviations, no `...` ellipses, no summaries. Show the real content so anyone can reconstruct the change without opening the file.
3. **`ROADMAP.md` and `LEARNING.md`** are never modified without learner's explicit approval.
4. **LOGS.md is self-documenting.** When entries are added to LOGS.md itself, the entry IS the record — no infinite recursion needed.
5. **For new files**, show the key content added with `+` lines. For large files (100+ lines), show the structure/skeleton with a note about total lines.

**Entry format:**
```
<details>
<summary>YYYY-MM-DD HH:MM — [filename] — short description</summary>

- **File:** path/to/file
- **Change type:** Created / Modified / Deleted
- **Reason:** why this change was made

\```diff
- exact old line that was removed
+ exact new line that replaced it
  unchanged context line above/below for reference
\```

</details>
```

---

<!-- Newest entries go here, above older ones -->

<details>
<summary>2026-08-15 23:15 IST — [SOLUTIONS.md / EXERCISES.md] — Added Solution 1.8-2 & Exercise 1.9-1</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Opened Solution 1.8-2 after learner completed `Portfolio` in `src/portfolio.rs`. Moved Exercise 1.8-2 to solved in EXERCISES.md, and added Exercise 1.9-1 skeleton.

```diff
[SOLUTIONS.md]
+ ### Solution 1.8-2 — `Portfolio` Tracker Engine & Custom Sorting (`HashMap`, `sort_by`, `PartialOrd`)
+ **Reference Implementation:**
+ pub fn add_position(&mut self, symbol: String, quantity: f64, price: f64) { ... }
+ pub fn get_sorted_positions(&self, current_prices: &HashMap<String, f64>) -> Vec<Position> { ... }

[EXERCISES.md]
- ### Exercise 1.8-2 — `Portfolio` Tracker Engine & Custom Sorting (`HashMap`, `sort_by`, `PartialOrd`) [open]
+ ### Exercise 1.9-1 — Newtype `OrderId` & `Order` Domain State Machine (`OrderId`, `OrderSide`, `OrderStatus`) [open]
+ ### Exercise 1.8-2 — `Portfolio` Tracker Engine & Custom Sorting (`HashMap`, `sort_by`, `PartialOrd`) [solved]
```

</details>

<details>
<summary>2026-08-15 23:14 IST — [LEARNING.md / ROADMAP.md] — Marked Module 1.8 Completed</summary>

- **Files:** LEARNING.md, ROADMAP.md
- **Change type:** Modified
- **Reason:** Learner explicitly confirmed marking Module 1.8 (Portfolio Management: Your Holdings) complete `[x]`.

```diff
[LEARNING.md]
+ ### Module 1.8 — Portfolio Management: Your Holdings — 2026-08-15
+ **Status:** `[x]` completed & understood
+ **What I actually understood:**
+ - Weighted average cost basis state tracking across multiple position fills.
+ - `HashMap` vs `BTreeMap` ordering trade-offs.
+ - Safe floating-point (`f64`) sorting using `PartialOrd::partial_cmp` and `unwrap_or(Ordering::Equal)`.
+ - Entry upserting via `.and_modify()` and `.or_insert_with()`.
+ **Code I wrote / project progress:**
+ - Created `src/portfolio.rs` with `Position` and `Portfolio`.
+ - Linked `mod portfolio;` in `src/main.rs`.

[ROADMAP.md]
- - [ ] **You build:** Portfolio tracker showing: current holdings, average buy price...
- - [ ] **Concepts:** `BTreeMap` vs `HashMap` — sorted vs unsorted, when each fits...
- - [ ] **Architecture:** Portfolio management at scale. How brokers calculate P&L...
- - [ ] **Deliverable:** Portfolio view with real-time P&L calculation. Multiple sort options...
+ - [x] **You build:** Portfolio tracker showing: current holdings, average buy price...
+ - [x] **Concepts:** `BTreeMap` vs `HashMap` — sorted vs unsorted, when each fits...
+ - [x] **Architecture:** Portfolio management at scale. How brokers calculate P&L...
+ - [x] **Deliverable:** Portfolio view with real-time P&L calculation. Multiple sort options...
```

</details>

<details>
<summary>2026-08-15 19:09 IST — [EXERCISES.md] — Incremented Hints Used (1/3) for Exercise 1.8-2</summary>

- **File:** EXERCISES.md
- **Change type:** Modified
- **Reason:** Learner asked how to write occupied/vacant handling on `HashMap::entry` in `add_position`. Incremented Hints used to 1/3.

```diff
[EXERCISES.md]
- **Hints used:** 0/3
+ **Hints used:** 1/3
```

</details>

<details>
<summary>2026-08-15 17:34 IST — [SOLUTIONS.md / EXERCISES.md] — Added Solution 1.8-1 & Exercise 1.8-2</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Opened Solution 1.8-1 after learner completed `Position` in `src/portfolio.rs`. Moved Exercise 1.8-1 to solved in EXERCISES.md, and added Exercise 1.8-2 skeleton.

```diff
[SOLUTIONS.md]
+ ### Solution 1.8-1 — Portfolio Holdings & Weighted Average Cost Basis (`Position`, `unrealized_pnl`)
+ **Reference Implementation:**
+ pub fn update(&mut self, add_qty: f64, buy_price: f64) { ... }
+ pub fn unrealized_pnl(&self, current_price: f64) -> f64 { ... }

[EXERCISES.md]
- ### Exercise 1.8-1 — Portfolio Holdings & Weighted Average Cost Basis (`Position`, `unrealized_pnl`) [open]
+ ### Exercise 1.8-2 — `Portfolio` Tracker Engine & Custom Sorting (`HashMap`, `sort_by`, `PartialOrd`) [open]
+ ### Exercise 1.8-1 — Portfolio Holdings & Weighted Average Cost Basis (`Position`, `unrealized_pnl`) [solved]
```

</details>

<details>
<summary>2026-08-15 16:50 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 22 & Exercise 1.8-1</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Added Concept 22 (Position Cost Basis & P&L Mechanics - Inventory Bucket & Weighted Average Price Scale analogy) word-for-word to EXAMPLES.md per Rule 8, and added Exercise 1.8-1 skeleton to EXERCISES.md per STEP 3.5.

```diff
[EXAMPLES.md]
+ ### 22. Position Cost Basis & P&L Mechanics (`Position` Struct, Weighted Average Cost Basis, Unrealized P&L) — The Inventory Bucket & Weighted Average Price Scale

[EXERCISES.md]
+ ### Exercise 1.8-1 — Portfolio Holdings & Weighted Average Cost Basis (`Position`, `unrealized_pnl`)
```

</details>

<details>
<summary>2026-08-15 16:49 IST — [LEARNING.md / ROADMAP.md] — Marked Module 1.7 Completed</summary>

- **Files:** LEARNING.md, ROADMAP.md
- **Change type:** Modified
- **Reason:** Learner explicitly confirmed marking Module 1.7 (Wallet System: Money Management) complete `[x]`.

```diff
[LEARNING.md]
+ ### Module 1.7 — Wallet System: Money Management — 2026-08-15
+ **Status:** `[x]` completed & understood
+ **What I actually understood:**
+ - Atomic in-place map mutation via `HashMap::entry().or_insert()`.
+ - Overdraft protection logic using match guards and custom error variants.
+ - Rust iterator laziness and pipeline adapters (`.filter()`, `.cloned()`, `.collect()`).
+ - Difference between `.clone()` (iterator adapter clone) and `.cloned()` (item clone).
+ **Code I wrote / project progress:**
+ - Created `src/wallet.rs` with `Wallet`, `TransactionRecord`, and `TransactionType`.
+ - Linked `mod wallet;` in `src/main.rs`.

[ROADMAP.md]
- - [ ] **You build:** Multi-currency wallet supporting deposits, withdrawals...
- - [ ] **Concepts:** Iterators — `Iterator` trait, laziness, consuming methods...
- - [ ] **Architecture:** Wallet architecture in exchanges. Double-entry bookkeeping concept...
- - [ ] **Deliverable:** Deposit/withdraw/balance working. Transaction history with iterator-based filtering...
+ - [x] **You build:** Multi-currency wallet supporting deposits, withdrawals...
+ - [x] **Concepts:** Iterators — `Iterator` trait, laziness, consuming methods...
+ - [x] **Architecture:** Wallet architecture in exchanges. Double-entry bookkeeping concept...
+ - [x] **Deliverable:** Deposit/withdraw/balance working. Transaction history with iterator-based filtering...
```

</details>

<details>
<summary>2026-08-15 14:24 IST — [EXERCISES.md] — Incremented Hints Used (2/3) for Exercise 1.7-2</summary>

- **File:** EXERCISES.md
- **Change type:** Modified
- **Reason:** Learner asked what was wrong in `deposit` function (`currency` move issue). Incremented Hints used to 2/3.

```diff
[EXERCISES.md]
- **Hints used:** 1/3
+ **Hints used:** 2/3
```

</details>

<details>
<summary>2026-08-15 14:11 IST — [EXERCISES.md] — Incremented Hints Used (1/3) for Exercise 1.7-2</summary>

- **File:** EXERCISES.md
- **Change type:** Modified
- **Reason:** Learner asked for help on `get_history` in Exercise 1.7-2 (`cloned()` vs `clone()`). Incremented Hints used to 1/3.

```diff
[EXERCISES.md]
- **Hints used:** 0/3
+ **Hints used:** 1/3
```

</details>

<details>
<summary>2026-08-14 23:55 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 21 & Exercise 1.7-2</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Added Concept 21 (Rust Iterators & Closure Filtering - Bank Statement Conveyor Belt & Automated Quality Filter analogy) word-for-word to EXAMPLES.md per Rule 8, and added Exercise 1.7-2 skeleton to EXERCISES.md per STEP 3.5.

```diff
[EXAMPLES.md]
+ ### 21. Rust Iterators & Closure Filtering (`Iterator`, `.filter()`, Closures, `.collect()`) — The Bank Statement Conveyor Belt & Automated Quality Filter

[EXERCISES.md]
+ ### Exercise 1.7-2 — Transaction Audit History & Iterator Filtering (`TransactionRecord`, `.filter()`, `.collect()`)
```

</details>

<details>
<summary>2026-08-14 23:46 IST — [SOLUTIONS.md / EXERCISES.md] — Added Solution 1.7-1 & Solved Exercise 1.7-1</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Opened Solution 1.7-1 after learner submitted attempt and explicitly requested solution. Moved Exercise 1.7-1 to solved in EXERCISES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.7-1 — Multi-Currency `Wallet` Engine (`HashMap::entry`, Overdraft Protection)
+ **Reference Implementation:**
+ pub fn deposit(&mut self, currency: String, amount: u64) -> Result<()> { ... }
+ pub fn withdraw(&mut self, currency: &str, amount: u64) -> Result<()> { ... }
+ pub fn get_balance(&self, currency: &str) -> u64 { ... }

[EXERCISES.md]
- ### Exercise 1.7-1 — Multi-Currency `Wallet` Engine (`HashMap::entry`, Overdraft Protection) [open]
+ ### Exercise 1.7-1 — Multi-Currency `Wallet` Engine (`HashMap::entry`, Overdraft Protection) [solved]
```

</details>

<details>
<summary>2026-08-14 23:38 IST — [EXERCISES.md] — Incremented Hints Used (2/3) for Exercise 1.7-1</summary>

- **File:** EXERCISES.md
- **Change type:** Modified
- **Reason:** Learner requested Tier 2 structural hint for `TradingError::InsufficientFunds` in `Wallet::withdraw`. Incremented Hints used to 2/3.

```diff
[EXERCISES.md]
- **Hints used:** 1/3
+ **Hints used:** 2/3
```

</details>

<details>
<summary>2026-08-14 23:32 IST — [EXERCISES.md] — Incremented Hints Used (1/3) for Exercise 1.7-1</summary>

- **File:** EXERCISES.md
- **Change type:** Modified
- **Reason:** Learner requested code check & hints for Exercise 1.7-1 (`Wallet` deposit, withdraw, get_balance). Incremented Hints used to 1/3.

```diff
[EXERCISES.md]
- **Hints used:** 0/3
+ **Hints used:** 1/3
```

</details>

<details>
<summary>2026-08-12 22:47 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 20 & Exercise 1.7-1</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Added Concept 20 (HashMap Entry API & Atomic Wallet Operations - Bank Safety Deposit Lockers & Smart Locker Keymaster analogy) word-for-word to EXAMPLES.md per Rule 8, and added Exercise 1.7-1 skeleton to EXERCISES.md per STEP 3.5.

```diff
[EXAMPLES.md]
+ ### 20. HashMap Entry API & Atomic Wallet Operations (`HashMap::entry`, `.or_insert()`, `Wallet`) — The Bank Safety Deposit Lockers & Smart Locker Keymaster

[EXERCISES.md]
+ ### Exercise 1.7-1 — Multi-Currency `Wallet` Engine (`HashMap::entry`, Overdraft Protection)
```

</details>

<details>
<summary>2026-08-12 22:46 IST — [LEARNING.md / ROADMAP.md] — Marked Module 1.6 Completed</summary>

- **Files:** LEARNING.md, ROADMAP.md
- **Change type:** Modified
- **Reason:** Learner explicitly confirmed marking Module 1.6 (User & Authentication System) complete `[x]`.

```diff
[LEARNING.md]
+ ### Module 1.6 — User & Authentication System — 2026-08-12
+ **Status:** `[x]` completed & understood
+ **What I actually understood:**
+ - Globally unique identity generation using `uuid::Uuid::new_v4()`.
+ - One-way cryptographic password hashing using `sha2::Sha256`.
+ - Timezone-aware timestamping with `chrono::Utc`.
+ - Dual-index in-memory data structures (`HashMap<Uuid, User>` & `HashMap<String, Uuid>`).
+ **Code I wrote / project progress:**
+ - Added `uuid`, `sha2`, `chrono` dependencies to `Cargo.toml`.
+ - Created `src/users.rs` with `User` and `UserManager`.
+ - Linked `mod users;` in `src/main.rs`.

[ROADMAP.md]
- - [ ] **You build:** User registration with username/password...
- - [ ] **Concepts:** Traits — defining behavior contracts...
- - [ ] **Architecture:** Authentication flow in trading platforms...
- - [ ] **Deliverable:** Register/login flow working end-to-end...
+ - [x] **You build:** User registration with username/password...
+ - [x] **Concepts:** Traits — defining behavior contracts...
+ - [x] **Architecture:** Authentication flow in trading platforms...
+ - [x] **Deliverable:** Register/login flow working end-to-end...
```

</details>

<details>
<summary>2026-08-12 20:48 IST — [SOLUTIONS.md / EXERCISES.md] — Added Solution 1.6-2 & Solved Exercise 1.6-2</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Opened Solution 1.6-2 after learner submitted attempt and explicitly requested solution. Moved Exercise 1.6-2 to solved in EXERCISES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.6-2 — In-Memory `UserManager` & Authentication Service (`HashMap`, Registration, Authentication)
+ **Reference Implementation:**
+ pub fn register(&mut self, username: String, password: &str) -> Result<&User> { ... }
+ pub fn authenticate(&self, username: &str, password: &str) -> Result<&User> { ... }

[EXERCISES.md]
- ### Exercise 1.6-2 — In-Memory `UserManager` & Authentication Service (`HashMap`, Registration, Authentication) [open]
+ ### Exercise 1.6-2 — In-Memory `UserManager` & Authentication Service (`HashMap`, Registration, Authentication) [solved]
```

</details>

<details>
<summary>2026-08-12 18:08 IST — [EXERCISES.md] — Incremented Hints Used (1/3) for Exercise 1.6-2</summary>

- **File:** EXERCISES.md
- **Change type:** Modified
- **Reason:** Learner requested hints for Exercise 1.6-2 (`UserManager` registration and authentication). Incremented Hints used to 1/3.

```diff
[EXERCISES.md]
- **Hints used:** 0/3
+ **Hints used:** 1/3
```

</details>

<details>
<summary>2026-08-12 16:20 IST — [SOLUTIONS.md / EXERCISES.md / EXAMPLES.md] — Added Solution 1.6-1, Exercise 1.6-2 & Concept 19</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md, EXAMPLES.md
- **Change type:** Modified
- **Reason:** Gated Solution 1.6-1 opened after learner completed implementation of `User` in `src/users.rs`. Moved Exercise 1.6-1 to solved in EXERCISES.md, added Exercise 1.6-2 skeleton, and added Concept 19 (User Management & Authentication Service) word-for-word to EXAMPLES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.6-1 — `User` Domain Model & Password Hashing (`uuid`, `sha2`, `chrono`)
+ **Reference Implementation:**
+ pub struct User { pub id: Uuid, pub username: String, pub password_hash: String, pub created_at: DateTime<Utc> }

[EXERCISES.md]
- ### Exercise 1.6-1 — `User` Domain Model & Password Hashing (`uuid`, `sha2`, `chrono`) [open]
+ ### Exercise 1.6-2 — In-Memory `UserManager` & Authentication Service (`HashMap`, Registration, Authentication) [open]
+ ### Exercise 1.6-1 — `User` Domain Model & Password Hashing (`uuid`, `sha2`, `chrono`) [solved]

[EXAMPLES.md]
+ ### 19. User Management & Authentication Service (`UserManager`, `HashMap<Uuid, User>`, Dual-Index Lookup) — The Exchange Membership Registry & Security Checkpoint
```

</details>

<details>
<summary>2026-08-12 15:32 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 18 & Exercise 1.6-1</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Added Concept 18 (Domain Identity, Trait Contracts & Password Hashing - Secure Digital Identity Badge & One-Way Vault Key analogy) word-for-word to EXAMPLES.md per Rule 8, and added Exercise 1.6-1 skeleton to EXERCISES.md per STEP 3.5.

```diff
[EXAMPLES.md]
+ ### 18. Domain Identity, Trait Contracts & Password Hashing (`User` Struct, `sha2`, `uuid::Uuid`, `chrono::Utc`) — The Secure Digital Identity Badge & One-Way Vault Key

[EXERCISES.md]
+ ### Exercise 1.6-1 — `User` Domain Model & Password Hashing (`uuid`, `sha2`, `chrono`)
```

</details>

<details>
<summary>2026-08-12 15:31 IST — [LEARNING.md / ROADMAP.md] — Marked Module 1.5 Completed</summary>

- **Files:** LEARNING.md, ROADMAP.md
- **Change type:** Modified
- **Reason:** Learner explicitly confirmed marking Module 1.5 (Error Handling: When Trades Fail) complete `[x]`.

```diff
[LEARNING.md]
+ ### Module 1.5 — Error Handling: When Trades Fail — 2026-08-12
+ **Status:** `[x]` completed & understood
+ **What I actually understood:**
+ - `Result<T, E>` as explicit, zero-cost error handling (vs exceptions / `panic!`).
+ - Custom error enums with `thiserror::Error` and `#[error("...")]` format attributes.
+ - `?` operator desugaring and automatic `From` conversions via `#[from]`.
+ - Idiomatic crate-level `pub type Result<T>` type aliasing.
+ **Code I wrote / project progress:**
+ - Added `thiserror` to `Cargo.toml`.
+ - Created `src/errors.rs` with `TradingError` enum and `Result<T>` alias.
+ - Linked `mod errors;` in `src/main.rs`.

[ROADMAP.md]
- - [ ] **You build:** A custom `TradingError` enum...
- - [ ] **Concepts:** `Result<T, E>` as the alternative to exceptions...
- - [ ] **Architecture:** Error hierarchy design...
- - [ ] **Deliverable:** `TradingError` used across all modules...
+ - [x] **You build:** A custom `TradingError` enum...
+ - [x] **Concepts:** `Result<T, E>` as the alternative to exceptions...
+ - [x] **Architecture:** Error hierarchy design...
+ - [x] **Deliverable:** `TradingError` used across all modules...
```

</details>

<details>
<summary>2026-08-12 14:11 IST — [SOLUTIONS.md / EXERCISES.md] — Added Solution 1.5-2 & Solved Exercise 1.5-2</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Gated Solution 1.5-2 opened after learner completed implementation of `#[from]` conversions and `Result<T>` type alias in `src/errors.rs`. Moved Exercise 1.5-2 to solved in EXERCISES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.5-2 — Automatic Error Conversions (`#[from]`) & Custom `Result` Type Alias
+ **Reference Implementation:**
+ Io(#[from] std::io::Error), ConfigParse(#[from] toml::de::Error), pub type Result<T> = std::result::Result<T, TradingError>;

[EXERCISES.md]
- ### Exercise 1.5-2 — Automatic Error Conversions (`#[from]`) & Custom `Result` Type Alias [open]
+ ### Exercise 1.5-2 — Automatic Error Conversions (`#[from]`) & Custom `Result` Type Alias [solved]
```

</details>

<details>
<summary>2026-08-12 13:55 IST — [SOLUTIONS.md / EXERCISES.md / EXAMPLES.md] — Added Solution 1.5-1, Exercise 1.5-2 & Concept 17</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md, EXAMPLES.md
- **Change type:** Modified
- **Reason:** Gated Solution 1.5-1 opened after learner completed implementation of `TradingError` in `src/errors.rs`. Moved Exercise 1.5-1 to solved in EXERCISES.md, added Exercise 1.5-2 skeleton, and added Concept 17 (Error Propagation & Automatic Conversions) word-for-word to EXAMPLES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.5-1 — Custom `TradingError` Enum (`thiserror`, `#[derive(Error)]`)
+ **Reference Implementation:**
+ pub enum TradingError { InsufficientFunds { .. }, OrderNotFound { .. }, InvalidQuantity { .. } }

[EXERCISES.md]
- ### Exercise 1.5-1 — Custom `TradingError` Enum (`thiserror`, `#[derive(Error)]`) [open]
+ ### Exercise 1.5-2 — Automatic Error Conversions (`#[from]`) & Custom `Result` Type Alias [open]
+ ### Exercise 1.5-1 — Custom `TradingError` Enum (`thiserror`, `#[derive(Error)]`) [solved]

[EXAMPLES.md]
+ ### 17. Error Propagation & Automatic Conversions (`?` Operator, `#[from]`, `Result<T>` Type Alias) — The Automatic Passport Translator at Border Control
```

</details>

<details>
<summary>2026-08-12 13:48 IST — [EXERCISES.md] — Fixed Exercise 1.5-1 Skeleton (Blanked Variants per Rule 16)</summary>

- **File:** EXERCISES.md
- **Change type:** Modified
- **Reason:** Corrected Exercise 1.5-1 skeleton to remove pre-exposed solution variants, leaving `todo!()` comments for learner implementation per Rule 16.

```diff
[EXERCISES.md]
- InsufficientFunds { required: u64, available: u64 },
- OrderNotFound { order_id: u64 },
- InvalidQuantity { message: String },
+ // TODO(1): Define InsufficientFunds variant...
+ // TODO(2): Define OrderNotFound variant...
+ // TODO(3): Define InvalidQuantity variant...
```

</details>

<details>
<summary>2026-08-12 12:50 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 16 & Exercise 1.5-1</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Added Concept 16 (Custom Error Types with thiserror - Security Alarm vs Emergency Power Cut analogy) word-for-word to EXAMPLES.md per Rule 8, and added Exercise 1.5-1 skeleton to EXERCISES.md per STEP 3.5.

```diff
[EXAMPLES.md]
+ ### 16. Custom Error Types & Error Hierarchy (`Result<T, E>`, `thiserror`, `?` operator) — The Security Alarm System vs Emergency Power Cut

[EXERCISES.md]
+ ### Exercise 1.5-1 — Custom `TradingError` Enum (`thiserror`, `#[derive(Error)]`)
```

</details>

<details>
<summary>2026-08-12 12:49 IST — [LEARNING.md / ROADMAP.md] — Marked Module 1.4 Completed</summary>

- **Files:** LEARNING.md, ROADMAP.md
- **Change type:** Modified
- **Reason:** Learner explicitly confirmed marking Module 1.4 (CLI Interface: Interactive Trading Terminal) complete `[x]`.

```diff
[LEARNING.md]
+ ### Module 1.4 — CLI Interface: Interactive Trading Terminal — 2026-08-12
+ **Status:** `[x]` completed & understood
+ **What I actually understood:**
+ - `clap` derive API: `#[derive(Parser)]` and `#[derive(Subcommand)]`.
+ - Positional arguments vs named flags (`#[arg(long)]`).
+ - Rust Module System & Visibility Rules: `mod`, `pub`, `pub(crate)`.
+ - Command Pattern Dispatching: `Cli::parse()` -> `match cli.command` variant destructuring.
+ **Code I wrote / project progress:**
+ - Added `clap` to `Cargo.toml`.
+ - Created `src/cli.rs` with `Cli` and `Commands`.
+ - Wired `Cli::parse()` and subcommand `match` handler in `src/main.rs`.

[ROADMAP.md]
- - [ ] **You build:** A `clap`-based CLI...
- - [ ] **Concepts:** `clap` derive API...
- - [ ] **Architecture:** Command pattern...
- - [ ] **Deliverable:** Working CLI skeleton...
+ - [x] **You build:** A `clap`-based CLI...
+ - [x] **Concepts:** `clap` derive API...
+ - [x] **Architecture:** Command pattern...
+ - [x] **Deliverable:** Working CLI skeleton...
```

</details>

<details>
<summary>2026-08-12 12:46 IST — [SOLUTIONS.md / EXERCISES.md] — Added Solution 1.4-2 & Solved Exercise 1.4-2</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Gated Solution 1.4-2 opened after learner completed implementation of `Cli::parse()` and subcommand dispatching in `src/main.rs`. Moved Exercise 1.4-2 to solved in EXERCISES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.4-2 — Command Parsing & Dispatching (`Cli::parse()`, `match cli.command`)
+ **Reference Implementation:**
+ match &cli.command { Commands::Buy { .. } => ..., Commands::Sell { .. } => ..., Commands::Balance => ..., Commands::Orders => ... }

[EXERCISES.md]
- ### Exercise 1.4-2 — Command Parsing & Dispatching (`Cli::parse()`, `match cli.command`) [open]
+ ### Exercise 1.4-2 — Command Parsing & Dispatching (`Cli::parse()`, `match cli.command`) [solved]
```

</details>

<details>
<summary>2026-08-11 14:38 IST — [RULES.md / .agents/workflows/next.md] — Added Rule 16 (Strict Exercise Isolation & Zero-Solution Pre-exposure)</summary>

- **Files:** RULES.md, .agents/workflows/next.md
- **Change type:** Modified
- **Reason:** Learner requested strict enforcement preventing solution code pre-exposure in chat prompts or exercise descriptions before attempts are submitted. Added Rule 16 and updated workflow self-audit gates.

```diff
[RULES.md]
+ 22. **Strict Exercise Isolation & Zero-Solution Pre-exposure:** When presenting an exercise to the learner (STEP 3.5), the AI MUST provide ONLY a skeleton containing `todo!()` blocks for the parts to be implemented. The AI MUST NEVER include the full solution body inside the exercise prompt, chat message, or `EXERCISES.md` skeleton before the learner submits their attempt.

[.agents/workflows/next.md]
+ - [ ] If a hands-on portion was reached: a skeleton exercise exists in `EXERCISES.md` (STEP 3.5-A) — a finished solution was NOT given in chat or exercise files instead (Rule #16).
+ - Never paste a complete working solution in chat or exercise descriptions before the learner attempts the exercise (Rule #16) — skeleton with `todo!()` + teaching comes first.
```

</details>

<details>
<summary>2026-08-11 13:55 IST — [SOLUTIONS.md / EXERCISES.md / EXAMPLES.md] — Added Solution 1.4-1, Exercise 1.4-2 & Concept 15</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md, EXAMPLES.md
- **Change type:** Modified
- **Reason:** Gated Solution 1.4-1 opened after learner completed implementation of `Cli` and `Commands` in `src/cli.rs`. Moved Exercise 1.4-1 to solved in EXERCISES.md, added Exercise 1.4-2 skeleton, and added Concept 15 (CLI Parsing, Command Dispatching & Module System) word-for-word to EXAMPLES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.4-1 — CLI Commands & Subcommands (`clap`, `Parser`, `Subcommand`)
+ **Reference Implementation:**
+ pub struct Cli { pub command: Commands } pub enum Commands { Buy { ... }, Sell { ... }, Balance, Orders }

[EXERCISES.md]
- ### Exercise 1.4-1 — CLI Commands & Subcommands (`clap`, `Parser`, `Subcommand`) [open]
+ ### Exercise 1.4-2 — Command Parsing & Dispatching (`Cli::parse()`, `match cli.command`) [open]
+ ### Exercise 1.4-1 — CLI Commands & Subcommands (`clap`, `Parser`, `Subcommand`) [solved]

[EXAMPLES.md]
+ ### 15. CLI Parsing, Command Dispatching & Module System (`Cli::parse()`, `match cli.command`, `pub(crate)`) — Central Train Station Dispatcher & VIP Security Passes
```

</details>

<details>
<summary>2026-08-10 23:27 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 14 & Exercise 1.4-1</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Added Concept 14 (Derive-Based CLI Parsers with clap - Bank Teller Counter vs Service Windows analogy) word-for-word to EXAMPLES.md per Rule 8, and added Exercise 1.4-1 skeleton to EXERCISES.md per STEP 3.5.

```diff
[EXAMPLES.md]
+ ### 14. Derive-Based CLI Parsers with `clap` Subcommands (`#[derive(Parser, Subcommand)]`) — Bank Teller Counter vs Dedicated Service Windows

[EXERCISES.md]
+ ### Exercise 1.4-1 — CLI Commands & Subcommands (`clap`, `Parser`, `Subcommand`)
```

</details>

<details>
<summary>2026-08-10 23:26 IST — [LEARNING.md / ROADMAP.md] — Marked Module 1.3 Completed</summary>

- **Files:** LEARNING.md, ROADMAP.md
- **Change type:** Modified
- **Reason:** Learner explicitly confirmed marking Module 1.3 (Configuration System) complete `[x]`.

```diff
[LEARNING.md]
+ ### Module 1.3 — Configuration System — 2026-08-10
+ **Status:** `[x]` completed & understood
+ **What I actually understood:**
+ - Ownership & Borrowing: `String` heap moves vs `&str` stack slices.
+ - `Option<T>` combinators: `unwrap_or` (eager) vs `unwrap_or_else` (lazy closure evaluation).
+ - Layered configuration architecture: File (`config.toml`) → Environment (`std::env::var`) → Hardcoded Defaults.
+ - Serde deserialization: `#[derive(Deserialize)]` & `toml::from_str::<Config>(&contents)`.
+ **Code I wrote / project progress:**
+ - Added `serde` and `toml` dependencies to `Cargo.toml`.
+ - Created `src/config.rs` containing `Config` struct, `from_env_or_default()`, and `from_file_or_env()`.
+ - Linked `mod config;` in `src/main.rs`.

[ROADMAP.md]
- - [ ] **You build:** A configuration loader...
- - [ ] **Concepts:** Ownership deep dive...
- - [ ] **Architecture:** Configuration hierarchy...
- - [ ] **Deliverable:** Config system that loads from file...
+ - [x] **You build:** A configuration loader...
+ - [x] **Concepts:** Ownership deep dive...
+ - [x] **Architecture:** Configuration hierarchy...
+ - [x] **Deliverable:** Config system that loads from file...
```

</details>

<details>
<summary>2026-08-10 23:10 IST — [SOLUTIONS.md / EXERCISES.md] — Added Solution 1.3-3 & Solved Exercise 1.3-3</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Gated Solution 1.3-3 opened after learner completed implementation of Serde TOML deserialization in `Cargo.toml` and `src/config.rs`. Moved Exercise 1.3-3 to solved in EXERCISES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.3-3 — Serde TOML Deserialization (`serde`, `toml::from_str`)
+ **Reference Implementation:**
+ match toml::from_str::<Config>(&contents) { Ok(config) => config, Err(_) => Self::from_env_or_default() }

[EXERCISES.md]
- ### Exercise 1.3-3 — Serde TOML Deserialization (`serde`, `toml::from_str`) [open]
+ ### Exercise 1.3-3 — Serde TOML Deserialization (`serde`, `toml::from_str`) [solved]
```

</details>

<details>
<summary>2026-08-10 14:17 IST — [SOLUTIONS.md / EXERCISES.md / EXAMPLES.md] — Added Solution 1.3-2, Exercise 1.3-3 & Concept 13</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md, EXAMPLES.md
- **Change type:** Modified
- **Reason:** Gated Solution 1.3-2 opened after learner completed implementation of `from_file_or_env` in `src/config.rs`. Moved Exercise 1.3-2 to solved in EXERCISES.md, added Exercise 1.3-3 skeleton, and added Concept 13 (Serde TOML Deserialization) word-for-word to EXAMPLES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.3-2 — File Parsing & Layered Fallback (`config.toml`, `std::fs::read_to_string`)
+ **Reference Implementation:**
+ impl Config { pub fn from_file_or_env(path: &str) -> Self { ... } }

[EXERCISES.md]
- ### Exercise 1.3-2 — File Parsing & Layered Fallback (`config.toml`, `std::fs::read_to_string`) [open]
+ ### Exercise 1.3-3 — Serde TOML Deserialization (`serde`, `toml::from_str`) [open]
+ ### Exercise 1.3-2 — File Parsing & Layered Fallback (`config.toml`, `std::fs::read_to_string`) [solved]

[EXAMPLES.md]
+ ### 13. Serde TOML Deserialization (`#[derive(Deserialize)]` & `toml::from_str`) — The Automated Customs Scanner
```

</details>

<details>
<summary>2026-08-08 14:47 IST — [SOLUTIONS.md / EXERCISES.md / EXAMPLES.md] — Added Solution 1.3-1, Exercise 1.3-2 & Concept 12</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md, EXAMPLES.md
- **Change type:** Modified
- **Reason:** Gated Solution 1.3-1 opened after learner completed implementation of `from_env_or_default` in `src/config.rs`. Moved Exercise 1.3-1 to solved in EXERCISES.md, added Exercise 1.3-2 skeleton, and added Concept 12 (File Parsing & Fallback Algorithm) word-for-word to EXAMPLES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.3-1 — Config Struct & Env Fallback (`Option<T>` & `unwrap_or_else`)
+ **Reference Implementation:**
+ impl Config { pub fn from_env_or_default() -> Self { ... } }

[EXERCISES.md]
- ### Exercise 1.3-1 — Config Struct & Env Fallback (`Option<T>` & `unwrap_or_else`) [open]
+ ### Exercise 1.3-2 — File Parsing & Layered Fallback (`config.toml`, `std::fs::read_to_string`) [open]
+ ### Exercise 1.3-1 — Config Struct & Env Fallback (`Option<T>` & `unwrap_or_else`) [solved]

[EXAMPLES.md]
+ ### 12. File Parsing with TOML & Layered Fallback — The Restaurant Menu Book vs Verbal Daily Specials
```

</details>

<details>
<summary>2026-08-08 14:05 IST — [EXERCISES.md] — Bumped Hints used to 2/3 for Exercise 1.3-1</summary>

- **File:** EXERCISES.md
- **Change type:** Modified
- **Reason:** Learner requested syntax/structural hint for Exercise 1.3-1 (`Config::from_env_or_default`). Bumped `Hints used: 2/3` per STEP 3.5-B.

```diff
[EXERCISES.md]
- **Hints used:** 1/3
+ **Hints used:** 2/3
```

</details>

<details>
<summary>2026-08-08 14:03 IST — [EXERCISES.md] — Bumped Hints used to 1/3 for Exercise 1.3-1</summary>

- **File:** EXERCISES.md
- **Change type:** Modified
- **Reason:** Learner requested hint for Exercise 1.3-1 (`Config::from_env_or_default`). Bumped `Hints used: 1/3` per STEP 3.5-B.

```diff
[EXERCISES.md]
- **Hints used:** 0/3
+ **Hints used:** 1/3
```

</details>

<details>
<summary>2026-08-07 23:42 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 11 & Exercise 1.3-1</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Added Concept 11 (Ownership & Borrowing Deep Dive - Deed vs Photo analogy) word-for-word to EXAMPLES.md per Rule 8, and added Exercise 1.3-1 skeleton to EXERCISES.md per STEP 3.5.

```diff
[EXAMPLES.md]
+ ### 11. Ownership & Borrowing Deep Dive (`String` vs `&str` in Configs) — The Deed vs The Verified Photo

[EXERCISES.md]
+ ### Exercise 1.3-1 — Config Struct & Env Fallback (`Option<T>` & `unwrap_or_else`)
```

</details>

<details>
<summary>2026-08-07 23:41 IST — [LEARNING.md / ROADMAP.md] — Marked Module 1.2 Completed</summary>

- **Files:** LEARNING.md, ROADMAP.md
- **Change type:** Modified
- **Reason:** Learner confirmed `/next` advancing, approving completion of Module 1.2 (Domain Types) `[x]`.

```diff
[LEARNING.md]
+ ### Module 1.2 — Domain Types: The Language of Trading — 2026-08-07
+ **Status:** `[x]` completed & understood
+ **What I actually understood:**
+ - Enums as Algebraic Data Types (discriminated unions with variant payloads).
+ - Newtype pattern (`struct Price(pub i64)`) for zero-cost type safety.
+ - Struct memory layout & derives (`Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`).
+ - Method receiver semantics: `&self` (immutable read), `&mut self` (exclusive write), `self` (consuming move).
+ **Code I wrote / project progress:**
+ - Built `src/models.rs` containing `Side`, `OrderType`, `OrderStatus`, `Price`, `Quantity`, and `Order` with `impl Order` methods (`new`, `fill`, `is_pending`).

[ROADMAP.md]
- - [ ] **You build:** Core domain types...
- - [ ] **Concepts:** Structs...
- - [ ] **Architecture:** Domain-Driven Design...
- - [ ] **Deliverable:** All core trading types defined...
+ - [x] **You build:** Core domain types...
+ - [x] **Concepts:** Structs...
+ - [x] **Architecture:** Domain-Driven Design...
+ - [x] **Deliverable:** All core trading types defined...
```

</details>

<details>
<summary>2026-08-07 15:11 IST — [SOLUTIONS.md / EXERCISES.md] — Added Solution 1.2-3 & Solved Exercise 1.2-3</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Gated Solution 1.2-3 opened after learner completed implementation of `impl Order` methods in `src/models.rs`. Moved Exercise 1.2-3 to solved in EXERCISES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.2-3 — Impl Blocks, Constructors (`Self::new()`), and Method Mutability (`&mut self`)
+ **Reference Implementation:**
+ impl Order { pub fn new(...) -> Self { ... } pub fn fill(&mut self) { ... } pub fn is_pending(&self) -> bool { ... } }

[EXERCISES.md]
- ### Exercise 1.2-3 — Impl Blocks, Constructors (`Self::new()`), and Method Mutability (`&mut self`) [open]
+ ### Exercise 1.2-3 — Impl Blocks, Constructors (`Self::new()`), and Method Mutability (`&mut self`) [solved]
```

</details>

<details>
<summary>2026-08-07 14:00 IST — [SOLUTIONS.md / EXERCISES.md / EXAMPLES.md] — Added Solution 1.2-2, Exercise 1.2-3 & Concept 10</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md, EXAMPLES.md
- **Change type:** Modified
- **Reason:** Gated Solution 1.2-2 opened after learner completed attempt in `src/models.rs`. Moved Exercise 1.2-2 to solved in EXERCISES.md, added Exercise 1.2-3 skeleton, and added Concept 10 (Method Receivers & Constructors) word-for-word to EXAMPLES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.2-2 — Structs & Newtype Pattern (`Price`, `Quantity`, `Order`)
+ **Reference Implementation:**
+ pub struct Price(pub i64);
+ pub struct Quantity(pub u64);
+ pub struct Order { ... }

[EXERCISES.md]
- ### Exercise 1.2-2 — Structs & Newtype Pattern (`Price`, `Quantity`, `Order`) [open]
+ ### Exercise 1.2-3 — Impl Blocks, Constructors (`Self::new()`), and Method Mutability (`&mut self`) [open]
+ ### Exercise 1.2-2 — Structs & Newtype Pattern (`Price`, `Quantity`, `Order`) [solved]

[EXAMPLES.md]
+ ### 10. Method Receivers (`&self`, `&mut self`, `self`) & Constructors (`Self::new()`) — Car Dashboard, Mechanic's Wrench, and Crusher
```

</details>

<details>
<summary>2026-08-07 12:15 IST — [SOLUTIONS.md / EXERCISES.md / EXAMPLES.md] — Added Solution 1.2-1, Exercise 1.2-2 & Concept 9</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md, EXAMPLES.md
- **Change type:** Modified
- **Reason:** Gated Solution 1.2-1 opened after learner completed attempt in `src/models.rs`. Moved Exercise 1.2-1 to solved in EXERCISES.md, added Exercise 1.2-2 skeleton, and added Concept 9 (Newtype Pattern & Structs) word-for-word to EXAMPLES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.2-1 — Core Trading Enums (`Side` and `OrderType`)
+ **Reference Implementation:**
+ pub enum Side { Buy, Sell }
+ pub enum OrderType { Market, Limit { price: i64 }, StopLoss { trigger_price: i64 } }

[EXERCISES.md]
- ### Exercise 1.2-1 — Defining Core Trading Enums (`Side` and `OrderType`) [open]
+ ### Exercise 1.2-2 — Structs & Newtype Pattern (`Price`, `Quantity`, `Order`) [open]
+ ### Exercise 1.2-1 — Defining Core Trading Enums (`Side` and `OrderType`) [solved]

[EXAMPLES.md]
+ ### 9. Structs & The Newtype Pattern (`Price`, `Quantity`, `Order`) — Currency Wallets & Trading Tickets
```

</details>

<details>
<summary>2026-08-06 18:51 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 8 & Exercise 1.2-1</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Added Concept 8 (Enums as ADTs - Multi-Tool Switch analogy) word-for-word to EXAMPLES.md per Rule 8, and added Exercise 1.2-1 skeleton to EXERCISES.md per STEP 3.5.

```diff
[EXAMPLES.md]
+ ### 8. Enums as Algebraic Data Types (`Side`, `OrderType`) — The Multi-Tool Switch
+ 
+ **ELI5 Analogy: The Multi-Tool Switch**
+ * In traditional languages (like C or TypeScript enums), an Enum is just a list of numbered labels (`0 = Buy`, `1 = Sell`).
+ * In Rust, an Enum is an **Algebraic Data Type (Sum Type)**: a multi-tool switch where each position can hold completely different tools and data!

[EXERCISES.md]
+ ### Exercise 1.2-1 — Defining Core Trading Enums (`Side` and `OrderType`)
+ **Status:** open
+ **Goal:** Define the `Side` and `OrderType` enums to represent trading sides and order types as algebraic data types.
```

</details>

<details>
<summary>2026-08-06 18:25 IST — [LEARNING.md / ROADMAP.md] — Marked Module 1.1 Completed</summary>

- **Files:** LEARNING.md, ROADMAP.md
- **Change type:** Modified
- **Reason:** User explicitly approved marking Module 1.1 (Project Setup & Cargo Fundamentals) as completed `[x]` after verifying all deliverables and concepts.

```diff
[LEARNING.md]
- | Phase 1 | Rust Foundations Through Real Trading Features | `[ ]` Not started |
+ | Phase 1 | Rust Foundations Through Real Trading Features | `[~]` In progress |

+ ### Module 1.1 — Project Setup & Cargo Fundamentals — 2026-08-06
+ **Status:** `[x]` completed & understood
+ **What I actually understood:**
+ - `cargo run` compiles and executes binary crates via `src/main.rs`.
+ - `Cargo.toml` vs `Cargo.lock` (human manifest vs deterministic version/hash lockfile).
+ - `&str` string literals are fat pointers (16 bytes on 64-bit stack) pointing to read-only data (`.rodata`) in binary memory, causing zero heap allocations.
+ - `String` is a 3-word struct (24 bytes) holding pointer, length, capacity for growable heap memory.
+ - Single-binary monolith architecture rationale (ADR-001) for Phase 1.
+ - Production linters: `cargo fmt --check` and `cargo clippy -- -D warnings`.
+ **Code I wrote / project progress:**
+ - Created `Cargo.toml`, `rust-toolchain.toml`, `.gitignore`, `src/main.rs`.
+ - Compiled and ran `trading-platform` binary printing the ecosystem initialization banner.

[ROADMAP.md]
- - [ ] **You build:** The project skeleton...
- - [ ] **Concepts:** `cargo new` / `cargo check`...
- - [ ] **Architecture:** Single-binary architecture...
- - [ ] **Deliverable:** Project compiles and runs...
+ - [x] **You build:** The project skeleton...
+ - [x] **Concepts:** `cargo new` / `cargo check`...
+ - [x] **Architecture:** Single-binary architecture...
+ - [x] **Deliverable:** Project compiles and runs...
```

</details>

<details>
<summary>2026-08-06 17:00 IST — [EXAMPLES.md / RULES.md / next.md] — Enforced Exact Word-for-Word Storage</summary>

- **Files:** EXAMPLES.md, RULES.md, .agents/workflows/next.md
- **Change type:** Modified
- **Reason:** Updated Concept 7 in EXAMPLES.md to match the exact word-for-word response text provided in chat. Updated Governance Rule 8 in RULES.md and STEP 3 of next.md workflow to strictly require exact word-for-word storage without paraphrasing or omitting lines.

```diff
[EXAMPLES.md]
- * **`&str` (String Slice / Literal):** A permanent billboard painted on a building wall (`.rodata` section of the binary)...
+ * **`&str` (String Literal / Slice):** Like a billboard painted directly onto a building wall. You didn't buy the building, and you can't erase or add new words to the wall. But pointing to it (`&`) is instantaneous and costs zero money (zero allocations).

[RULES.md]
- 8. **ELI5 Analogy Storage & Detail:** ...
+ 8. **Exact Word-for-Word ELI5 Analogy & Technical Detail Storage:** Whenever the AI explains any concept, the AI MUST provide BOTH a simple ELI5 analogy AND a rigorous, deep technical explanation. The AI MUST store the EXACT, word-for-word analogy and deep technical breakdown in `EXAMPLES.md` without any paraphrasing, line omissions, or summary rewrites.

[.agents/workflows/next.md]
- 5. **ELI5 analogy + deep technical explanation** for every new concept (Rule #8)... Write the entry into `EXAMPLES.md`...
+ 5. **ELI5 analogy + deep technical explanation** for every new concept (Rule #8)... Write the **EXACT, word-for-word** ELI5 analogy and deep technical breakdown into `EXAMPLES.md`...
```

</details>

<details>
<summary>2026-08-06 16:50 IST — [EXAMPLES.md] — Added String Literals vs Heap Strings (Concept 7)</summary>

- **File:** EXAMPLES.md
- **Change type:** Modified
- **Reason:** Added ELI5 analogy (Billboard vs Notepad) and deep technical explanation for `&str` vs `String` memory layout in Module 1.1 per Rule 8.

```diff
+ ### 7. String Literals (`&str`) vs Heap Strings (`String`) — The Billboard vs The Notepad
+ 
+ **Core Concept:** The difference between an unallocated string slice pointing to read-only data (`&str`) and an owned, growable heap buffer (`String`).
+ 
+ **ELI5 Analogy: The Billboard vs The Notepad**
+ * **`&str` (String Slice / Literal):** A permanent billboard painted on a building wall...
+ * **`String` (Owned String):** A physical spiral notepad you bought and carry in your backpack...
```

</details>

<details>
<summary>2026-08-06 15:21 IST — [EXAMPLES.md] — Added Cargo & Toolchain Mental Model (Concept 6)</summary>

- **File:** EXAMPLES.md
- **Change type:** Modified
- **Reason:** User rightly called out that Concept 6 (The Restaurant Kitchen: rustup, rustc, cargo, Cargo.toml, Cargo.lock) taught during Module 1.1 kickoff was missing from EXAMPLES.md, violating Governance Rule 8. Added both the ELI5 analogy and deep technical explanation.

```diff
+ ### 6. Cargo & Toolchain — The Restaurant Kitchen
+ 
+ **Core Concept:** Understanding the responsibilities of `rustup`, `rustc`, `cargo`, `Cargo.toml`, and `Cargo.lock`.
+ 
+ **ELI5 Analogy: The Restaurant Kitchen**
+ * **`rustup` (The Franchise Manager):** Installs and updates the whole kitchen system...
+ * **`rustc` (The Executive Chef):** The raw compiler...
+ * **`cargo` (The Kitchen Manager):** Coordinates the whole operation...
+ * **`Cargo.toml` (The Recipe & Ingredient List):** Human-readable dependency manifest...
+ * **`Cargo.lock` (The Exact Batch Delivery Receipt):** Lockfile guaranteeing byte-for-byte build reproducibility...
+ 
+ **Deep Technical Explanation:**
+ Rust decouples version management (`rustup`), compilation (`rustc`), and package/build orchestration (`cargo`)...
```

</details>

<details>
<summary>2026-07-28 02:41 IST — [.gitignore] — Created standard Rust gitignore</summary>

- **File:** .gitignore
- **Change type:** Created
- **Reason:** Add standard Rust gitignore rules for Cargo build artifacts, IDE configs, OS files, and environment files.

```diff
+ # Generated by Cargo
+ /target/
+ .idea/
+ .vscode/
+ *.swp
+ *.swo
+ .DS_Store
+ Thumbs.db
+ .env
+ *.env
```

</details>


<details>
<summary>2026-07-27 23:41 IST — [Initial Project Setup] — Created all project files</summary>

- **Files:** ROADMAP.md, LEARNING.md, LOGS.md, PROMPTS.md, EXAMPLES.md, DECISIONS.md, QUESTIONS.md, RULES.md, KICKOFF_PROMPT.md, Cargo.toml, rust-toolchain.toml, .gitignore, src/main.rs
- **Change type:** Created (all files)
- **Reason:** Initial project setup for the Production-Grade Trading Ecosystem learning project. All governance files, configuration, and source entry point created from scratch following the same format established in `C:\Dev\Rust`.

```diff
+ Created 13 files in C:\Dev\Rust-Projects\trading-platform\
+
+ Governance files (7):
+   ROADMAP.md     — 3-phase curriculum with ~45 modules
+   LEARNING.md    — Progress journal (empty, ready for Module 1.1)
+   LOGS.md        — This file (file change audit log)
+   PROMPTS.md     — Prompt history tracker
+   EXAMPLES.md    — ELI5 analogies + deep technical explanations
+   DECISIONS.md   — Architecture Decision Records
+   QUESTIONS.md   — Interview questions + answers
+
+ Reference files (2):
+   RULES.md           — 15 governance rules
+   KICKOFF_PROMPT.md  — Master prompt for portability
+
+ Project files (4):
+   Cargo.toml         — Initial crate manifest (edition 2024)
+   rust-toolchain.toml — Pins stable toolchain
+   .gitignore         — Standard Rust gitignore
+   src/main.rs        — Minimal entry point
```

</details>
