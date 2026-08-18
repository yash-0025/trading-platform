# 📘 LEARNING.md — Living Progress Journal

> This file is the **source of truth for actual progress**. The AI tutor reads this before each session to calibrate pacing, but will **never edit this file without asking first.**
> Update this yourself as you go, or ask the AI to "log today's progress" at the end of a session — it will draft the entry and show you before writing anything.

**Status legend:** `[ ]` not started · `[~]` in progress · `[x]` done & understood · `[!]` done but shaky, needs revisit

### ⚙️ Environment
- **OS:** Windows 11
- **Toolchain:** *(to be confirmed at Module 1.1 — run `rustc --version` and update)*
- **IDE:** VS Code / Antigravity IDE
- **Project:** `C:\Dev\Rust-Projects\trading-platform`

### 🗣️ Teaching Style
- **Use simple, everyday English.** No fancy words or dense academic writing. Talk like a friend explaining things.
- **Show through code, not words.** Short code examples > long paragraphs.
- **Use real-world analogies** from JS/TS/Solidity and trading/finance.
- **One small step at a time.** Don't explain 5 things in one go.
- **Always teach before assigning.** Explain the concept fully before asking to write code.
- **ELI5 + Deep Technical** for every concept. Store in `EXAMPLES.md`.
- **Explain every syntax element.** Over-explain `&`, `*`, `mut`, `Option`, `self`, etc.
- **Explain the goal before code.** Clear picture of final outcome before Step 1.

---

## 📊 Quick Progress Snapshot
*(Update this table as phases progress.)*

| Phase | Focus Area | Status |
|---|---|---|
| Phase 1 | Rust Foundations Through Real Trading Features | `[~]` In progress |
| Phase 2 | Production Backend | `[ ]` Not started |
| Phase 3 | Trading Infrastructure & HFT Concepts | `[ ]` Not started |

---

## Module-by-Module Log

### Module 1.1 — Project Setup & Cargo Fundamentals — 2026-08-06
**Status:** `[x]` completed & understood
**What I actually understood:**
- `cargo run` compiles and executes binary crates via `src/main.rs`.
- `Cargo.toml` vs `Cargo.lock` (human manifest vs deterministic version/hash lockfile).
- `&str` string literals are fat pointers (16 bytes on 64-bit stack) pointing to read-only data (`.rodata`) in binary memory, causing zero heap allocations.
- `String` is a 3-word struct (24 bytes) holding pointer, length, capacity for growable heap memory.
- Single-binary monolith architecture rationale (ADR-001) for Phase 1.
- Production linters: `cargo fmt --check` and `cargo clippy -- -D warnings`.
**Code I wrote / project progress:**
- Created `Cargo.toml`, `rust-toolchain.toml`, `.gitignore`, `src/main.rs`.
- Compiled and ran `trading-platform` binary printing the ecosystem initialization banner.

### Module 1.2 — Domain Types: The Language of Trading — 2026-08-07
**Status:** `[x]` completed & understood
**What I actually understood:**
- Enums as Algebraic Data Types (discriminated unions with variant payloads).
- Newtype pattern (`struct Price(pub i64)`) for zero-cost type safety.
- Struct memory layout & derives (`Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`).
- Method receiver semantics: `&self` (immutable read), `&mut self` (exclusive write), `self` (consuming move).
**Code I wrote / project progress:**
- Built `src/models.rs` containing `Side`, `OrderType`, `OrderStatus`, `Price`, `Quantity`, and `Order` with `impl Order` methods (`new`, `fill`, `is_pending`).

### Module 1.3 — Configuration System — 2026-08-10
**Status:** `[x]` completed & understood
**What I actually understood:**
- Ownership & Borrowing: `String` heap moves vs `&str` stack slices.
- `Option<T>` combinators: `unwrap_or` (eager) vs `unwrap_or_else` (lazy closure evaluation).
- Layered configuration architecture: File (`config.toml`) → Environment (`std::env::var`) → Hardcoded Defaults.
- Serde deserialization: `#[derive(Deserialize)]` & `toml::from_str::<Config>(&contents)`.
**Code I wrote / project progress:**
- Added `serde` and `toml` dependencies to `Cargo.toml`.
- Created `src/config.rs` containing `Config` struct, `from_env_or_default()`, and `from_file_or_env()`.
- Linked `mod config;` in `src/main.rs`.

### Module 1.4 — CLI Interface: Interactive Trading Terminal — 2026-08-12
**Status:** `[x]` completed & understood
**What I actually understood:**
- `clap` derive API: `#[derive(Parser)]` and `#[derive(Subcommand)]`.
- Positional arguments vs named flags (`#[arg(long)]`).
- Rust Module System & Visibility Rules: `mod`, `pub`, `pub(crate)`.
- Command Pattern Dispatching: `Cli::parse()` -> `match cli.command` variant destructuring.
**Code I wrote / project progress:**
- Added `clap` to `Cargo.toml`.
- Created `src/cli.rs` with `Cli` and `Commands`.
- Wired `Cli::parse()` and subcommand `match` handler in `src/main.rs`.

### Module 1.5 — Error Handling: When Trades Fail — 2026-08-12
**Status:** `[x]` completed & understood
**What I actually understood:**
- `Result<T, E>` as explicit, zero-cost error handling (vs exceptions / `panic!`).
- Custom error enums with `thiserror::Error` and `#[error("...")]` format attributes.
- `?` operator desugaring and automatic `From` conversions via `#[from]`.
- Idiomatic crate-level `pub type Result<T>` type aliasing.
**Code I wrote / project progress:**
- Added `thiserror` to `Cargo.toml`.
- Created `src/errors.rs` with `TradingError` enum and `Result<T>` alias.
- Linked `mod errors;` in `src/main.rs`.

### Module 1.6 — User & Authentication System — 2026-08-12
**Status:** `[x]` completed & understood
**What I actually understood:**
- Globally unique identity generation using `uuid::Uuid::new_v4()`.
- One-way cryptographic password hashing using `sha2::Sha256`.
- Timezone-aware timestamping with `chrono::Utc`.
- Dual-index in-memory data structures (`HashMap<Uuid, User>` & `HashMap<String, Uuid>`).
**Code I wrote / project progress:**
- Added `uuid`, `sha2`, `chrono` dependencies to `Cargo.toml`.
- Created `src/users.rs` with `User` and `UserManager`.
- Linked `mod users;` in `src/main.rs`.

### Module 1.7 — Wallet System: Money Management — 2026-08-15
**Status:** `[x]` completed & understood
**What I actually understood:**
- Atomic in-place map mutation via `HashMap::entry().or_insert()`.
- Overdraft protection logic using match guards and custom error variants.
- Rust iterator laziness and pipeline adapters (`.filter()`, `.cloned()`, `.collect()`).
- Difference between `.clone()` (iterator adapter clone) and `.cloned()` (item clone).
**Code I wrote / project progress:**
- Created `src/wallet.rs` with `Wallet`, `TransactionRecord`, and `TransactionType`.
- Linked `mod wallet;` in `src/main.rs`.

### Module 1.8 — Portfolio Management: Your Holdings — 2026-08-15
**Status:** `[x]` completed & understood
**What I actually understood:**
- Weighted average cost basis state tracking across multiple position fills.
- `HashMap` vs `BTreeMap` ordering trade-offs.
- Safe floating-point (`f64`) sorting using `PartialOrd::partial_cmp` and `unwrap_or(Ordering::Equal)`.
- Entry upserting via `.and_modify()` and `.or_insert_with()`.
**Code I wrote / project progress:**
- Created `src/portfolio.rs` with `Position` and `Portfolio`.
- Linked `mod portfolio;` in `src/main.rs`.

### Module 1.9 — Order Management (Basic) — 2026-08-16
**Status:** `[x]` completed & understood
**What I actually understood:**
- Zero-cost type safety using the Newtype pattern (`struct OrderId(pub u64)`).
- Enforcing domain invariants via enum state machines (`OrderStatus::Pending -> Cancelled`).
- The Builder Pattern with method chaining (`mut self` returns) and atomic validation on `.build()`.
- Struct variant error construction (`TradingError::InvalidOrder { message: String }`).
**Code I wrote / project progress:**
- Created `src/orders.rs` with `OrderId`, `OrderSide`, `OrderStatus`, `Order`, and `OrderBuilder`.
- Linked `mod orders;` in `src/main.rs`.

### Module 1.10 — File Persistence: Saving State — 2026-08-17
**Status:** `[x]` completed & understood
**What I actually understood:**
- Generic serialization and deserialization bounds (`T: Serialize`, `T: DeserializeOwned`).
- Flexible file path borrowing with `&Path` vs heap-allocated `PathBuf`.
- Crate feature flags (`uuid = { features = ["serde"] }`, `chrono = { features = ["serde"] }`) for external Serde support.
- Writing unit tests (`#[cfg(test)]`) to verify round-trip JSON persistence and file cleanup.
**Code I wrote / project progress:**
- Created `src/storage.rs` with `StorageEngine` and `test_storage_rountrip`.
- Derived `Serialize` and `Deserialize` across all domain models.
- Linked `mod storage;` in `src/main.rs`.

### Module 1.11 — Positions Tracking — 2026-08-18
**Status:** `[x]` completed & understood
**What I actually understood:**
- Realized vs unrealized P&L accounting engines (locking cash profit on sell fills vs mark-to-market open position valuation).
- Smart pointer mechanics: `Box<T>` (heap allocation), `Rc<T>` (shared reference counting), `RefCell<T>` (interior mutability runtime borrow checks), and `Rc<RefCell<T>>`.
- Unit testing with `#[cfg(test)] mod tests` and `#[test]` assertion macros (`assert_eq!`).
**Code I wrote / project progress:**
- Created `src/tracker.rs` with `PositionTracker`, `process_fill`, `total_pnl`, and `test_position_tracker_buy_sell_pnl`.
- Linked `mod tracker;` in `src/main.rs` and re-exported in `src/lib.rs`.

### Module 1.12 — Testing Suite — 2026-08-18
**Status:** `[x]` completed & understood
**What I actually understood:**
- Integration testing in root `tests/` directory (`tests/*.rs`) treating crate as an external library consumer (`use trading_platform::*`).
- `Result<(), E>` returning test signatures for clean `?` error propagation without panicking.
- Executable documentation tests (`///`) compiled and executed via `cargo test --doc`.
- Edge-case panic testing with `#[should_panic(expected = "InsufficientFunds")]`.
**Code I wrote / project progress:**
- Created `src/lib.rs` re-exporting all modules.
- Created `tests/integration_test.rs` with `test_end_to_end_trading_flow`.
- Added executable doc comments and panic tests to `src/wallet.rs`.

### Module 1.13 — Multi-Module Architecture Refactoring — 2026-08-19
**Status:** `[x]` completed & understood
**What I actually understood:**
- Modern Rust module subtrees (`foo.rs` + `foo/`) across 100% of codebase domains.
- Public re-export facades (`pub use path::Item`) decoupling internal file structures from public crate APIs.
- Visibility level controls (`pub(crate)`) restricting internal helpers to crate scope.
- One-way dependency direction (Services depend on Models, Models never depend on Services).
**Code I wrote / project progress:**

### Module 1.14 — Documentation & Code Quality — 2026-08-19
**Status:** `[x]` completed & understood
**What I actually understood:**
- Inner module doc comments (`//!`) for crate architecture overviews vs outer (`///`) for struct/function items.
- Intra-doc link syntax (`[`Wallet`]`) automatically hyperlinking code symbols in generated HTML docs.
- Compiler inner attributes (`#![warn(missing_docs)]`) enforcing documentation standards across public APIs.
- Project-wide code quality configuration via `rustfmt.toml` and `clippy.toml`.
- Item-level lint attributes (`#[allow(dead_code)]`, `#[warn(missing_docs)]`, `#[deny(unused_variables)]`) controlling lint severity per item.
**Code I wrote / project progress:**
- Created `rustfmt.toml` and `clippy.toml` in project root.
- Added `//!` inner doc block and `#![warn(missing_docs)]` to `src/lib.rs`.
- Added `#[allow(dead_code)]` to `clear_positions` in `src/services/tracker.rs`.
- Added `#[warn(missing_docs)]` in `src/config/settings.rs` and `#[deny(unused_variables)]` in `src/storage/engine.rs`.
- Updated `README.md` with complete Phase 1 module overview.





