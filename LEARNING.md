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
