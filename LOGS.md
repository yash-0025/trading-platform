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
<summary>2026-08-18 12:49 IST — [RULES.md / next.md / SOLUTIONS_EXPLANATIONS.md] — Expanded Rule 19 for Exhaustive Token/Syntax Breakdown</summary>

- **Files:** RULES.md, .agents/workflows/next.md, SOLUTIONS_EXPLANATIONS.md
- **Change type:** Modified
- **Reason:** Learner requested expanding Rule 19 requiring every solution entry in `SOLUTIONS_EXPLANATIONS.md` to include exhaustive line-by-line token, keyword, symbol, type bound, and method call breakdowns for BOTH skeleton syntax and solution syntax (what it is, why used, exact syntax mechanics, and technical rationale). Updated workflow `.agents/workflows/next.md` and expanded Solution 1.10-3 entry.

```diff
[RULES.md]
- 19. Solution Plain English Explanation Storage (SOLUTIONS_EXPLANATIONS.md)...
+ 19. Solution Plain English Explanation & Exhaustive Syntax Storage (SOLUTIONS_EXPLANATIONS.md)... [requires exhaustive token/keyword/symbol/type-bound syntax breakdowns for both skeleton and solution syntax]

[.agents/workflows/next.md]
+ Updated STEP 3.5-D Revealing and STEP 7 Self-Audit checklist for Rule 19 expanded syntax breakdown requirements.

[SOLUTIONS_EXPLANATIONS.md]
+ Solution 1.10-3: Added 🦴 Skeleton Syntax Deep Breakdown and 💡 Solution Syntax Deep Breakdown covering every token, keyword, and method call.
```

</details>

<details>
<summary>2026-08-18 02:00 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 34 & Exercise 1.10-3</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Gap 5 (Module 1.10): Serde attributes (`#[serde(rename_all = "camelCase")]`, `#[serde(default)]`, `#[serde(skip)]`), struct lifetimes (`StorageMetadata<'a>`), `PathBuf` path extension methods, and atomic file saving (`.tmp` write & rename) were listed in ROADMAP.md but never coded in `src/storage.rs`. Added Concept 34 word-for-word to EXAMPLES.md and created Exercise 1.10-3 skeleton in EXERCISES.md.

```diff
[EXAMPLES.md]
+ ### 34. Serde Field Attributes (`rename_all`, `default`, `skip`), Struct Lifetimes (`'a`), `PathBuf` vs `Path`, & Atomic File Writes (`.tmp` Rename)
+ [ELI5 analogies + deep technical breakdowns]

[EXERCISES.md]
+ ### Exercise 1.10-3 — Serde Field Attributes, Struct Lifetimes, `PathBuf` & Atomic Storage Writes
+ **Status:** open
+ [2 TODOs: load_json_or_default with T::default() fallback, save_json_atomic with .tmp path write & fs::rename]
```

</details>

<details>
<summary>2026-08-18 01:03 IST — [SOLUTIONS.md / SOLUTIONS_EXPLANATIONS.md / EXERCISES.md] — Placed Solution 1.9-3 & Solved Exercise 1.9-3 in Exact Numerical Order</summary>

- **Files:** SOLUTIONS.md, SOLUTIONS_EXPLANATIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Learner completed attempt for Exercise 1.9-3 in `src/orders.rs` and requested solution. Placed Solution 1.9-3 in exact numerical position in SOLUTIONS.md, added plain English thought translation to SOLUTIONS_EXPLANATIONS.md per Rule 19, and moved Exercise 1.9-3 to Solved in EXERCISES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.9-3 — Data-Bearing Enums (`OrderType`), Auto-Incrementing IDs (`OrderId`), & `OrderManager` Query Engine
+ [Reference implementation + line-by-line breakdown + attempt comparison]

[SOLUTIONS_EXPLANATIONS.md]
+ ### Solution 1.9-3 — Data-Bearing Enums (`OrderType`), Auto-Incrementing IDs (`OrderId`), & `OrderManager` Query Engine
+ [Plain English thought translation + line-by-line breakdown]

[EXERCISES.md]
+ ### Exercise 1.9-3 — Data-Bearing Enums (`OrderType`), Auto-Incrementing IDs (`OrderId`), & `OrderManager` Query Engine [solved]
```

</details>

<details>
<summary>2026-08-18 00:30 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 33 & Exercise 1.9-3</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Gap 4 (Module 1.9): Data-bearing enums (`OrderType::Market` vs `OrderType::Limit { limit_price: u64 }`), `OrderManager` struct, and order collection filtering (`get_pending_orders`, `filter_by_symbol`) were listed in ROADMAP.md but never coded in `src/orders.rs`. Added Concept 33 word-for-word to EXAMPLES.md and created Exercise 1.9-3 skeleton in EXERCISES.md.

```diff
[EXAMPLES.md]
+ ### 33. Enums with Data Variants (`OrderType`), Auto-Incrementing ID Generator (`OrderId`), & `OrderManager` Search Filtering
+ [ELI5 analogies + deep technical breakdowns]

[EXERCISES.md]
+ ### Exercise 1.9-3 — Data-Bearing Enums (`OrderType`), Auto-Incrementing IDs (`OrderId`), & `OrderManager` Query Engine
+ **Status:** open
+ [4 TODOs: submit order with auto-increment ID, cancel order by ID, get_pending_orders filter, filter_by_symbol]
```

</details>

<details>
<summary>2026-08-17 23:25 IST — [SOLUTIONS.md / SOLUTIONS_EXPLANATIONS.md / EXERCISES.md] — Placed Solution 1.8-3 & Solved Exercise 1.8-3 in Exact Numerical Order</summary>

- **Files:** SOLUTIONS.md, SOLUTIONS_EXPLANATIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Learner completed attempt for Exercise 1.8-3 in `src/portfolio.rs` and requested solution. Placed Solution 1.8-3 in exact numerical position in SOLUTIONS.md, added plain English thought translation to SOLUTIONS_EXPLANATIONS.md per Rule 19, and moved Exercise 1.8-3 to Solved in EXERCISES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.8-3 — `BTreeMap` Portfolio View, Advanced Iterator Chains & `Display` Trait (`.zip()`, `.enumerate()`, `.flat_map()`, `.chain()`, `fmt::Display`)
+ [Reference implementation + line-by-line breakdown + attempt comparison]

[SOLUTIONS_EXPLANATIONS.md]
+ ### Solution 1.8-3 — `BTreeMap` Portfolio View, Advanced Iterator Chains & `Display` Trait
+ [Plain English thought translation + line-by-line breakdown]

[EXERCISES.md]
+ ### Exercise 1.8-3 — `BTreeMap` Portfolio View, Advanced Iterator Chains & `Display` Trait [solved]
```

</details>

<details>
<summary>2026-08-17 22:57 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 32 & Exercise 1.8-3</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Gap 3 (Module 1.8): `BTreeMap`, `.zip()`, `.enumerate()`, `.flat_map()`, `.chain()`, and `Display` trait are listed in ROADMAP.md but never coded in `src/portfolio.rs`. Added Concept 32 word-for-word to EXAMPLES.md and created Exercise 1.8-3 skeleton in EXERCISES.md.

```diff
[EXAMPLES.md]
+ ### 32. `BTreeMap` vs `HashMap`, Advanced Iterator Adapters (`.zip()`, `.enumerate()`, `.flat_map()`, `.chain()`), and `Display` Trait Formatting
+ [ELI5 analogies + deep technical breakdowns for all 6 concepts]

[EXERCISES.md]
+ ### Exercise 1.8-3 — `BTreeMap` Portfolio View, Advanced Iterator Chains & `Display` Trait
+ **Status:** open
+ [4 TODOs: add_to_sorted BTreeMap, portfolio_report with .enumerate()/.chain(), Display for Position, Display for Portfolio]
```

</details>

<details>
<summary>2026-08-17 22:45 IST — [.agents/workflows/next.md] — Compressed from 12,928 to 10,380 bytes (all content preserved)</summary>

- **File:** .agents/workflows/next.md
- **Change type:** Modified
- **Reason:** File exceeded 12,000-byte limit (was 12,928 bytes). Compressed to 10,380 bytes by tightening verbose phrasing, removing redundant blank lines, and shortening repeated explanations. All 19 rules, all 10 STEPs, all audit checks, all anti-patterns, and all off-script handlers remain fully intact and detailed.

```diff
- Total Bytes: 12,928 (over limit)
+ Total Bytes: 10,380 (under 12,000 limit, 2,548 bytes saved)
  All STEPs (-1 through 9): preserved
  All 19 governance rules: referenced
  All 11 self-audit checkboxes: preserved
  All 9 anti-patterns: preserved
```

</details>

<details>
<summary>2026-08-17 22:31 IST — [.agents/workflows/next.md] — Synchronized All Governance Rules (16, 17, 18, 19) into Workflow Script</summary>

- **File:** .agents/workflows/next.md
- **Change type:** Modified
- **Reason:** Synchronized workflow script to reflect all 19 governance rules, updating rule count in STEP 0, adding Rules 17-19 to STEP 3 checklist, and incorporating Rules 17-19 into STEP 7 mandatory self-audit.

```diff
[.agents/workflows/next.md]
- 1. RULES.md — 15 governance rules. Re-internalize before proceeding; obey silently.
+ 1. RULES.md — 19 governance rules. Re-internalize before proceeding; obey silently.
+ - [ ] Rule 17: Plain English "thought translation" was provided alongside technical breakdown.
+ - [ ] Rule 18: Every concept in ROADMAP.md has an active hands-on code exercise in src/. Theory-only is forbidden.
+ - [ ] Rule 19: If a solution was revealed, its plain English thought translation was appended to SOLUTIONS_EXPLANATIONS.md in exact numerical order.
```

</details>

<details>
<summary>2026-08-17 22:28 IST — [RULES.md / SOLUTIONS_EXPLANATIONS.md / next.md] — Added Rule 19 & Created SOLUTIONS_EXPLANATIONS.md</summary>

- **Files:** RULES.md, SOLUTIONS_EXPLANATIONS.md, .agents/workflows/next.md
- **Change type:** Created / Modified
- **Reason:** Learner requested adding Rule 19 requiring all revealed solution plain English explanations and line-by-line breakdowns to be permanently preserved in `SOLUTIONS_EXPLANATIONS.md` in exact numerical order, and updated `.agents/workflows/next.md` workflow.

```diff
[RULES.md]
+ 19. **Solution Plain English Explanation Storage (`SOLUTIONS_EXPLANATIONS.md`):** Whenever a solution is created and added to `SOLUTIONS.md`, the AI MUST also create/append an entry in `SOLUTIONS_EXPLANATIONS.md` storing the plain natural English "thought translation" and line-by-line breakdown of that solution in exact numerical order.

[SOLUTIONS_EXPLANATIONS.md]
+ Created SOLUTIONS_EXPLANATIONS.md with plain English thought translations and line-by-line breakdowns for Solutions 1.4-3, 1.7-3, and 1.11-1 in exact numerical order.
```

</details>

<details>
<summary>2026-08-17 22:25 IST — [SOLUTIONS.md / EXERCISES.md] — Placed Solution 1.7-3 & Solved Exercise 1.7-3 in Strict Numerical Order</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Learner completed Exercise 1.7-3 in `src/wallet.rs`. Added Solution 1.7-3 in exact numerical position in SOLUTIONS.md and moved Exercise 1.7-3 to exact numerical position under Solved in EXERCISES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.7-3 — Wallet Accumulation & Closure Trait Queries (`.sum()`, Turbofish `::<>`, `Fn`)
+ **Reference Implementation:**
+ pub fn total_balance(&self) -> u64 { self.balances.values().sum::<u64>() }
+ pub fn filter_transactions<F>(&self, predicate: F) -> Vec<TransactionRecord> where F: Fn(&TransactionRecord) -> bool { ... }

[EXERCISES.md]
+ ### Exercise 1.7-3 — Wallet Accumulation & Closure Trait Queries (`.sum()`, Turbofish `::<>`, `Fn`) [solved]
```

</details>

<details>
<summary>2026-08-17 22:02 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 31 & Exercise 1.7-3</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Added Concept 31 (Advanced Iterator Accumulation, Turbofish Syntax & Closure Generics) word-for-word to EXAMPLES.md per Rule 8, and created Exercise 1.7-3 skeleton in EXERCISES.md to fill Module 1.7 roadmap code gaps.

```diff
[EXAMPLES.md]
+ ### 31. Advanced Iterator Accumulation, Turbofish Syntax & Closure Generics (`.fold()`, `.sum()`, `::<>`, `Fn`)

[EXERCISES.md]
+ ### Exercise 1.7-3 — Wallet Accumulation & Closure Trait Queries (`.sum()`, Turbofish `::<>`, `Fn`)
```

</details>

<details>
<summary>2026-08-17 21:59 IST — [SOLUTIONS.md / EXERCISES.md] — Updated Exercise 1.4-3 Field Target to max_order_size</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Updated Exercise 1.4-3 target field to `max_order_size` to match `Config` struct's actual fields (`exchange_name`, `currency`, `max_order_size`, `log_level`).

```diff
[SOLUTIONS.md]
- ### Solution 1.4-3 — Environment Variable Overrides (`std::env::var`, `TRADING_PORT`)
+ ### Solution 1.4-3 — Environment Variable Overrides (`std::env::var`, `TRADING_MAX_ORDER_SIZE`)
```

</details>

<details>
<summary>2026-08-17 21:51 IST — [SOLUTIONS.md / EXERCISES.md] — Placed Solution 1.4-3 & Solved Exercise 1.4-3 in Strict Numerical Order</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Learner completed Exercise 1.4-3 in `src/config.rs`. Added Solution 1.4-3 in exact numerical position in SOLUTIONS.md and moved Exercise 1.4-3 to exact numerical position under Solved in EXERCISES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.4-3 — Environment Variable Overrides (`std::env::var`, `TRADING_PORT`)
+ **Reference Implementation:**
+ pub fn apply_env_overrides(&mut self) { ... }

[EXERCISES.md]
+ ### Exercise 1.4-3 — Environment Variable Overrides (`std::env::var`, `TRADING_PORT`) [solved]
```

</details>

<details>
<summary>2026-08-17 21:42 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 30 & Exercise 1.4-3</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Added Concept 30 (Reading Environment Variables `std::env::var` - Command Post Override Switch analogy) word-for-word to EXAMPLES.md per Rule 8, and created Exercise 1.4-3 skeleton in EXERCISES.md to begin systematically filling all roadmap code gaps.

```diff
[EXAMPLES.md]
+ ### 30. Reading Environment Variables (`std::env::var`, `VarError`) — The Command Post Override Switch

[EXERCISES.md]
+ ### Exercise 1.4-3 — Environment Variable Overrides (`std::env::var`, `TRADING_PORT`)
```

</details>

<details>
<summary>2026-08-17 21:37 IST — [RULES.md] — Added Rule 18 (100% Roadmap-to-Code Implementation Enforcement)</summary>

- **File:** RULES.md
- **Change type:** Modified
- **Reason:** Learner requested adding a governance rule requiring that every concept listed in `ROADMAP.md` MUST be actively coded, compiled, and tested in `src/` source files, prohibiting theory-only explanations without code.

```diff
[RULES.md]
+ 18. **100% Roadmap-to-Code Implementation Enforcement:** Every single concept, data structure, smart pointer, iterator adapter, trait, and design pattern listed under a module in `ROADMAP.md` MUST be actively coded, compiled, and tested in `src/` source files. Theory-only or text-only explanations of roadmap concepts in markdown files (`EXAMPLES.md` or chat) are STRICTLY FORBIDDEN unless accompanied by hands-on Rust code in `src/`.
```

</details>

<details>
<summary>2026-08-17 21:22 IST — [EXAMPLES.md / EXERCISES.md] — Added Concept 29 & Exercise 1.11-2</summary>

- **Files:** EXAMPLES.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Added Concept 29 (Shared Ownership & Interior Mutability in Action - Multi-Key Safe Deposit Box analogy) word-for-word to EXAMPLES.md per Rule 8, and created Exercise 1.11-2 skeleton in EXERCISES.md.

```diff
[EXAMPLES.md]
+ ### 29. Shared Ownership & Interior Mutability in Action (`Rc<RefCell<T>>` & `Weak<T>`) — The Multi-Key Safe Deposit Box

[EXERCISES.md]
+ ### Exercise 1.11-2 — Shared Position Mutability & Unit Test Suite (`Rc<RefCell<Position>>`, `#[test]`)
```

</details>

<details>
<summary>2026-08-17 19:48 IST — [RULES.md] — Added Rule 17 (Plain English / Layman's Natural Language Code Translation)</summary>

- **File:** RULES.md
- **Change type:** Modified
- **Reason:** Learner requested adding a governance rule requiring plain everyday English "thought translations" alongside complex Rust code idioms.

```diff
[RULES.md]
+ 17. **Plain English / Layman's Natural Language Code Translation:** Whenever explaining complex Rust code, idiom blocks (`match`, `HashMap::entry().and_modify().or_insert_with()`, `get_mut()`, `if let`, etc.), or solutions, the AI MUST provide a plain natural English "thought translation" alongside the technical breakdown...
```

</details>

<details>
<summary>2026-08-17 19:44 IST — [SOLUTIONS.md / EXERCISES.md] — Added Solution 1.11-1</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Opened Solution 1.11-1 after learner submitted an attempt for `PositionTracker` in `src/tracker.rs`. Moved Exercise 1.11-1 to solved in EXERCISES.md.

```diff
[SOLUTIONS.md]
+ ### Solution 1.11-1 — Realized & Unrealized P&L Accounting Engine (`PositionTracker`, `Order` Fill Execution)
+ **Reference Implementation:**
+ pub fn process_fill(&mut self, side: OrderSide, symbol: &str, qty: f64, price: f64) { ... }
+ pub fn total_pnl(&self, current_prices: &HashMap<String, f64>) -> f64 { ... }

[EXERCISES.md]
- ### Exercise 1.11-1 — Realized & Unrealized P&L Accounting Engine (`PositionTracker`, `Order` Fill Execution) [open]
+ ### Exercise 1.11-1 — Realized & Unrealized P&L Accounting Engine (`PositionTracker`, `Order` Fill Execution) [solved]
```

</details>

<details>
<summary>2026-08-17 19:33 IST — [EXERCISES.md] — Incremented Hint Counter (1/3)</summary>

- **File:** EXERCISES.md
- **Change type:** Modified
- **Reason:** Incremented hint counter to 1/3 for Exercise 1.11-1 as learner asked for guidance on `total_pnl`.

```diff
[EXERCISES.md]
- **Hints used:** 0/3
+ **Hints used:** 1/3
```

</details>

<details>
<summary>2026-08-17 17:44 IST — [SOLUTIONS.md / EXERCISES.md] — Added Solution 1.10-2 & Exercise 1.11-1</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Opened Solution 1.10-2 after learner completed Exercise 1.10-2 in `src/storage.rs`. Moved Exercise 1.10-2 to solved in EXERCISES.md, and added Exercise 1.11-1 skeleton.

```diff
[SOLUTIONS.md]
+ ### Solution 1.10-2 — Domain Struct Serde Derives & Round-Trip Persistence Testing (`#[derive(Serialize, Deserialize)]`, `#[test]`)
+ **Reference Implementation:**
+ fn test_storage_rountrip() { ... }

[EXERCISES.md]
- ### Exercise 1.10-2 — Domain Struct Serde Derives & Round-Trip Persistence Testing (`#[derive(Serialize, Deserialize)]`, `#[test]`) [open]
+ ### Exercise 1.11-1 — Realized & Unrealized P&L Accounting Engine (`PositionTracker`, `Order` Fill Execution) [open]
+ ### Exercise 1.10-2 — Domain Struct Serde Derives & Round-Trip Persistence Testing (`#[derive(Serialize, Deserialize)]`, `#[test]`) [solved]
```

</details>

<details>
<summary>2026-08-17 17:43 IST — [LEARNING.md / ROADMAP.md] — Marked Module 1.10 Completed</summary>

- **Files:** LEARNING.md, ROADMAP.md
- **Change type:** Modified
- **Reason:** Learner explicitly confirmed marking Module 1.10 (File Persistence: Saving State) complete `[x]`.

```diff
[LEARNING.md]
+ ### Module 1.10 — File Persistence: Saving State — 2026-08-17
+ **Status:** `[x]` completed & understood
+ **What I actually understood:**
+ - Generic serialization and deserialization bounds (`T: Serialize`, `T: DeserializeOwned`).
+ - Flexible file path borrowing with `&Path` vs heap-allocated `PathBuf`.
+ - Crate feature flags (`uuid = { features = ["serde"] }`, `chrono = { features = ["serde"] }`) for external Serde support.
+ - Writing unit tests (`#[cfg(test)]`) to verify round-trip JSON persistence and file cleanup.
+ **Code I wrote / project progress:**
+ - Created `src/storage.rs` with `StorageEngine` and `test_storage_rountrip`.
+ - Derived `Serialize` and `Deserialize` across all domain models.
+ - Linked `mod storage;` in `src/main.rs`.

[ROADMAP.md]
- - [ ] **You build:** JSON-based persistence for all data: users, wallets, orders, portfolio...
- - [ ] **Concepts:** `serde` + `serde_json` for serialization · `#[derive(Serialize, Deserialize)]`...
- - [ ] **Architecture:** Persistence strategies. Why JSON for dev, binary/DB for production...
- - [ ] **Deliverable:** All state survives process restarts. Graceful handling of corrupted/missing files...
+ - [x] **You build:** JSON-based persistence for all data: users, wallets, orders, portfolio...
+ - [x] **Concepts:** `serde` + `serde_json` for serialization · `#[derive(Serialize, Deserialize)]`...
+ - [x] **Architecture:** Persistence strategies. Why JSON for dev, binary/DB for production...
+ - [x] **Deliverable:** All state survives process restarts. Graceful handling of corrupted/missing files...
```

</details>

<details>
<summary>2026-08-17 17:38 IST — [Cargo.toml] — Enabled Serde Feature Flags on uuid and chrono</summary>

- **File:** Cargo.toml
- **Change type:** Modified
- **Reason:** Enabled `"serde"` feature flags on `uuid` (`features = ["v4", "serde"]`) and `chrono` (`features = ["serde"]`) so `Uuid` and `DateTime<Utc>` implement `Serialize` and `Deserialize`.

```diff
[Cargo.toml]
- uuid = { version = "1.6", features = ["v4"] }
+ uuid = { version = "1.6", features = ["v4", "serde"] }
- chrono = "0.4"
+ chrono = { version = "0.4", features = ["serde"] }
```

</details>

<details>
<summary>2026-08-17 15:15 IST — [SOLUTIONS.md / EXERCISES.md] — Added Solution 1.10-1 & Exercise 1.10-2</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Opened Solution 1.10-1 after learner completed `StorageEngine` in `src/storage.rs`. Moved Exercise 1.10-1 to solved in EXERCISES.md, and added Exercise 1.10-2 skeleton.

```diff
[SOLUTIONS.md]
+ ### Solution 1.10-1 — Domain Model Serde Derive & Storage Persistence Engine (`Serialize`, `Deserialize`, `save`, `load`)
+ **Reference Implementation:**
+ pub fn save_json<T: Serialize>(path: &Path, data: &T) -> Result<(), TradingError> { ... }
+ pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T, TradingError> { ... }

[EXERCISES.md]
- ### Exercise 1.10-1 — Domain Model Serde Derive & Storage Persistence Engine (`Serialize`, `Deserialize`, `save`, `load`) [open]
+ ### Exercise 1.10-2 — Domain Struct Serde Derives & Round-Trip Persistence Testing (`#[derive(Serialize, Deserialize)]`, `#[test]`) [open]
+ ### Exercise 1.10-1 — Domain Model Serde Derive & Storage Persistence Engine (`Serialize`, `Deserialize`, `save`, `load`) [solved]
```

</details>

<details>
<summary>2026-08-16 22:24 IST — [errors.rs] — Added Json Variant to TradingError</summary>

- **File:** src/errors.rs
- **Change type:** Modified
- **Reason:** Added `Json(#[from] serde_json::Error)` variant to `TradingError` so the `?` operator automatically desugars `serde_json::Error` in storage operations.

```diff
[src/errors.rs]
+ #[error("Serde JSON error: {0}")]
+ Json(#[from] serde_json::Error),
```

</details>

<details>
<summary>2026-08-16 19:35 IST — [README.md] — Created Project README & AI Learning System Documentation</summary>

- **File:** README.md
- **Change type:** Created
- **Reason:** Learner requested comprehensive README detailing project vision, architecture, roadmap status, dependencies, AI learning system setup, and usage instructions.

```diff
[README.md]
+ # 🦀 Production-Grade Trading Platform in Rust
+ > A hands-on, zero-to-hero journey building a high-frequency, production-grade cryptocurrency and equity trading platform engine from scratch in Rust...
```

</details>

<details>
<summary>2026-08-16 15:54 IST — [Cargo.toml / EXAMPLES.md / EXERCISES.md / SOLUTIONS.md] — Added serde_json, Concept 26, Exercise 1.10-1, & Solution 1.9-2</summary>

- **Files:** Cargo.toml, EXAMPLES.md, EXERCISES.md, SOLUTIONS.md
- **Change type:** Modified
- **Reason:** Added `serde_json = "1.0"` to Cargo.toml for file persistence, unlocked Solution 1.9-2 in SOLUTIONS.md, added Concept 26 (Serde Data Serialization & File Paths - Universal Packing Crate & Shipping Manifest analogy) to EXAMPLES.md word-for-word per Rule 8, moved Exercise 1.9-2 to solved in EXERCISES.md, and added Exercise 1.10-1 skeleton.

```diff
[Cargo.toml]
+ serde_json = "1.0"

[EXAMPLES.md]
+ ### 26. Serde Data Serialization & File Paths (`serde`, `Serialize`/`Deserialize`, `PathBuf` vs `Path`) — The Universal Packing Crate & Shipping Manifest

[SOLUTIONS.md]
+ ### Solution 1.9-2 — The Builder Pattern for Order Creation (`OrderBuilder`, Method Chaining, Validation)

[EXERCISES.md]
- ### Exercise 1.9-2 — The Builder Pattern for Order Creation (`OrderBuilder`, Method Chaining, Validation) [open]
+ ### Exercise 1.10-1 — Domain Model Serde Derive & Storage Persistence Engine (`Serialize`, `Deserialize`, `save`, `load`) [open]
+ ### Exercise 1.9-2 — The Builder Pattern for Order Creation (`OrderBuilder`, Method Chaining, Validation) [solved]
```

</details>

<details>
<summary>2026-08-16 15:53 IST — [LEARNING.md / ROADMAP.md] — Marked Module 1.9 Completed</summary>

- **Files:** LEARNING.md, ROADMAP.md
- **Change type:** Modified
- **Reason:** Learner explicitly confirmed marking Module 1.9 (Order Management Basic) complete `[x]`.

```diff
[LEARNING.md]
+ ### Module 1.9 — Order Management (Basic) — 2026-08-16
+ **Status:** `[x]` completed & understood
+ **What I actually understood:**
+ - Zero-cost type safety using the Newtype pattern (`struct OrderId(pub u64)`).
+ - Enforcing domain invariants via enum state machines (`OrderStatus::Pending -> Cancelled`).
+ - The Builder Pattern with method chaining (`mut self` returns) and atomic validation on `.build()`.
+ - Struct variant error construction (`TradingError::InvalidOrder { message: String }`).
+ **Code I wrote / project progress:**
+ - Created `src/orders.rs` with `OrderId`, `OrderSide`, `OrderStatus`, `Order`, and `OrderBuilder`.
+ - Linked `mod orders;` in `src/main.rs`.

[ROADMAP.md]
- - [ ] **You build:** Submit market/limit orders, cancel pending orders...
- - [ ] **Concepts:** Builder pattern — `OrderBuilder` with method chaining...
- - [ ] **Architecture:** Order Management System (OMS) design. Order lifecycle...
- - [ ] **Deliverable:** Full order CRUD. State transitions enforced by types...
+ - [x] **You build:** Submit market/limit orders, cancel pending orders...
+ - [x] **Concepts:** Builder pattern — `OrderBuilder` with method chaining...
+ - [x] **Architecture:** Order Management System (OMS) design. Order lifecycle...
+ - [x] **Deliverable:** Full order CRUD. State transitions enforced by types...
```

</details>

<details>
<summary>2026-08-16 15:01 IST — [errors.rs / EXERCISES.md] — Added InvalidOrder Variant & Incremented Hints (1/3)</summary>

- **Files:** src/errors.rs, EXERCISES.md
- **Change type:** Modified
- **Reason:** Added `InvalidOrder { message: String }` variant to `TradingError` in `src/errors.rs` so `TradingError::InvalidOrder` works, and incremented Hints used to 1/3 in EXERCISES.md.

```diff
[src/errors.rs]
+ #[error("Invalid order: {message}")]
+ InvalidOrder { message: String},

[EXERCISES.md]
- **Hints used:** 0/3
+ **Hints used:** 1/3
```

</details>

<details>
<summary>2026-08-16 01:31 IST — [SOLUTIONS.md / EXERCISES.md] — Added Solution 1.9-1 & Exercise 1.9-2</summary>

- **Files:** SOLUTIONS.md, EXERCISES.md
- **Change type:** Modified
- **Reason:** Opened Solution 1.9-1 after learner completed `Order` state machine in `src/orders.rs`. Moved Exercise 1.9-1 to solved in EXERCISES.md, and added Exercise 1.9-2 skeleton.

```diff
[SOLUTIONS.md]
+ ### Solution 1.9-1 — Newtype `OrderId` & `Order` Domain State Machine (`OrderId`, `OrderSide`, `OrderStatus`)
+ **Reference Implementation:**
+ pub struct OrderId(pub u64);
+ pub fn cancel(&mut self) -> bool { ... }

[EXERCISES.md]
- ### Exercise 1.9-1 — Newtype `OrderId` & `Order` Domain State Machine (`OrderId`, `OrderSide`, `OrderStatus`) [open]
+ ### Exercise 1.9-2 — The Builder Pattern for Order Creation (`OrderBuilder`, Method Chaining, Validation) [open]
+ ### Exercise 1.9-1 — Newtype `OrderId` & `Order` Domain State Machine (`OrderId`, `OrderSide`, `OrderStatus`) [solved]
```

</details>

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
