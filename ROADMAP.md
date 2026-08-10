# 🦀 TRADING PLATFORM ROADMAP — Production-Grade Trading Ecosystem

> **Created:** 2026-07-27 · **Project:** trading-platform · **Location:** `C:\Dev\Rust-Projects\trading-platform`
> **Learner profile:** Full-stack (MERN) engineer with Web3/Solidity security background. Completed Weeks 1-2 of Rust CLI curriculum (ownership, borrowing, lifetimes, generics, traits, smart pointers, testing). Currently in Week 3 (async/concurrency).
> **Philosophy:** ONE project, continuously evolving. Every Rust concept exists because the project demands it. Never teach in isolation.
> **Rust edition target:** 2024 Edition · **Toolchain:** stable

---

## 🔒 GOVERNANCE RULES (read before touching any file)

1. **No silent edits.** Neither `ROADMAP.md` nor `LEARNING.md` is ever modified by the AI without explicitly asking the learner first and getting a yes.
2. **Every change is logged.** Any edit to any file gets a corresponding collapsible entry in `LOGS.md` with full before/after diffs.
3. **`LEARNING.md` is the source of truth for progress.** `ROADMAP.md` is the curriculum. `LEARNING.md` is what's actually been learned.
4. **The roadmap cross-checks the learning log.** Before starting a new module, check `LEARNING.md` for what's done, what's shaky.
5. **Status markers:**
   - `[ ]` Not started
   - `[~]` In progress
   - `[x]` Completed & understood
   - `[!]` Completed but shaky / needs revisit
6. **One concept at a time, in project context.** Teach one concept fully before moving on.
7. **Never encourage copy-paste.** Provide code as reference and explanation; learner types manually.
8. **Always provide ELI5 + deep technical explanations.** Store in `EXAMPLES.md`.
9. **Explain every line of code.** Break down what, how, why.
10. **Review governance rules before every response.**
11. **Explain every syntax element.** `&`, `*`, `mut`, `Option`, `map`, `self` — over-explain.
12. **Explain the goal before writing code.** Clear picture of final outcome before Step 1.
13. **Explain the whole project before every major subsystem.** High-level overview + ASCII architecture.
14. **Verify completion before advancing.** Cross-reference roadmap, ensure nothing was missed.
15. **Always explain system architecture before implementation.** System design, data flow, thread model, memory flow.

---

## 📐 HOW THIS ROADMAP IS STRUCTURED

This roadmap is organized into **3 Phases**, each containing **~15 Modules**. Unlike the 30-day CLI curriculum, this project is **milestone-based** rather than time-boxed — each module represents a meaningful feature addition to the trading platform.

Every module has:
- **You build** — the feature/subsystem being added
- **Concepts you learn** — Rust features the project demands (just-in-time)
- **Architecture** — system design, data flow, ASCII diagrams
- **Deliverable** — what should be working after this module
- **Status** — `[ ]` / `[~]` / `[x]` / `[!]`

---

## PHASE 1 — Rust Foundations Through Real Trading Features

> **Goal:** Build a working CLI-based trading application. Learn Rust's core type system, ownership model, error handling, testing, and module architecture — all through building real trading features.
>
> **End state:** A polished, tested, modular CLI trading platform with user management, wallet, portfolio, order management, and file persistence. Portfolio piece #1.

---

### Module 1.1 — Project Setup & Cargo Fundamentals
- [x] **You build:** The project skeleton. `cargo new`, `Cargo.toml` configuration, `rust-toolchain.toml`, `.gitignore`, initial `main.rs` that prints a trading platform banner and version.
- [x] **Concepts:** `cargo new` / `cargo check` / `cargo run` / `cargo build --release` · `Cargo.toml` vs `Cargo.lock` · Editions (2024) · `rustfmt` + `clippy` from minute one · `rust-toolchain.toml` · Basic `println!` macro · String literals vs `String` type · Program entry point
- [x] **Architecture:** Single-binary architecture. Why we start here and evolve into a workspace later.
- [x] **Deliverable:** Project compiles and runs. `cargo fmt --check` and `cargo clippy -- -D warnings` pass.

---

### Module 1.2 — Domain Types: The Language of Trading
- [x] **You build:** Core domain types that represent the trading world: `Asset`, `Side` (Buy/Sell), `OrderType` (Market/Limit/StopLoss), `OrderStatus`, `Order`, `Price`, `Quantity`.
- [x] **Concepts:** Structs (named-field, tuple structs, unit structs) · Enums as algebraic data types (not like TS/Java enums) · `match` exhaustiveness · Deriving `Debug`, `Clone`, `PartialEq` · `impl` blocks · `Self::new()` constructor pattern · `&self` vs `&mut self` vs `self` methods · Type aliases · Documentation comments `///`
- [x] **Architecture:** Domain-Driven Design — why types should model the business domain. How Zerodha/Binance represent orders internally.
- [x] **Deliverable:** All core trading types defined in `src/models/` with constructors, display formatting, and basic unit tests.

---

### Module 1.3 — Configuration System
- [x] **You build:** A configuration loader that reads settings from a TOML file (`config.toml`) and falls back to environment variables. Settings: exchange name, default currency, max order size, log level.
- [x] **Concepts:** Ownership deep dive — `String` vs `&str`, moves, clones · Borrowing: `&T` (shared) vs `&mut T` (exclusive) · The borrow checker as compile-time data-race prevention · `std::fs::read_to_string` · `toml` crate for parsing · `std::env::var` · `Option<T>` for optional config values · `unwrap()` vs `unwrap_or` vs `unwrap_or_else`
- [x] **Architecture:** Configuration hierarchy (file → env → defaults). How production systems handle config.
- [x] **Deliverable:** Config system that loads from file with env var overrides. Unit tests for each fallback path.

---

### Module 1.4 — CLI Interface: Interactive Trading Terminal
- [ ] **You build:** A `clap`-based CLI with subcommands: `register`, `login`, `deposit`, `withdraw`, `balance`, `buy`, `sell`, `portfolio`, `orders`, `cancel`. Interactive menu mode.
- [ ] **Concepts:** `clap` derive API · Module system: `mod`, `pub`, `pub(crate)`, file-based modules · `use` imports, re-exports · Visibility rules · `std::io` for reading user input · Shadowing vs mutation · `loop` + `break` for REPL
- [ ] **Architecture:** Command pattern — how CLIs map to domain operations. How trading terminals work (Bloomberg Terminal, TOS).
- [ ] **Deliverable:** Working CLI skeleton that accepts all subcommands and prints placeholder responses.

---

### Module 1.5 — Error Handling: When Trades Fail
- [ ] **You build:** A custom `TradingError` enum covering: `InsufficientFunds`, `OrderNotFound`, `InvalidQuantity`, `AuthenticationFailed`, `ConfigError`, `StorageError`. Propagation through the entire call stack.
- [ ] **Concepts:** `Result<T, E>` as the alternative to exceptions · `?` operator and `From::from` desugaring · `panic!` vs `Result` — when each is appropriate · Custom error enums with `thiserror` · `anyhow` for application-level errors · `From`/`Into` for error conversion · `unwrap()`, `expect()` in production = code smell · `Display` trait for user-facing errors
- [ ] **Architecture:** Error hierarchy design. How trading platforms handle cascading failures. Why typed errors matter for order rejection reasons.
- [ ] **Deliverable:** `TradingError` used across all modules. No `unwrap()` in non-test code. Descriptive error messages for every failure path.

---

### Module 1.6 — User & Authentication System
- [ ] **You build:** User registration with username/password, login verification, session tracking. Passwords stored as hashes (SHA-256 or argon2). User profiles with creation date, last login.
- [ ] **Concepts:** Traits — defining behavior contracts · Implementing traits for types · Default trait implementations · Trait bounds · Generic functions · `From`/`Into`/`TryFrom`/`TryInto` · `Display` and `Debug` traits · `Hash` trait · `chrono` crate for timestamps · `uuid` crate for user IDs
- [ ] **Architecture:** Authentication flow in trading platforms. Why user identity matters for order attribution, audit trails, compliance (KYC).
- [ ] **Deliverable:** Register/login flow working end-to-end through CLI. Passwords hashed. User data persisted (in-memory for now, file in Module 1.10).

---

### Module 1.7 — Wallet System: Money Management
- [ ] **You build:** Multi-currency wallet supporting deposits, withdrawals, balance queries, and transaction history. Overdraft protection. Transaction logging with timestamps.
- [ ] **Concepts:** Iterators — `Iterator` trait, laziness, consuming methods · Closures — `Fn`, `FnMut`, `FnOnce`, capture semantics · Iterator chains: `.map()`, `.filter()`, `.fold()`, `.sum()`, `.collect()` · `HashMap<K, V>` for currency balances · `Entry` API (`.entry().or_insert()`) · Turbofish syntax `::<>` · Zero-cost abstractions — iterator chains vs hand-rolled loops
- [ ] **Architecture:** Wallet architecture in exchanges. Double-entry bookkeeping concept. Why atomicity matters (what happens if deposit succeeds but balance update fails?).
- [ ] **Deliverable:** Deposit/withdraw/balance working. Transaction history with iterator-based filtering (by date, by currency, by type).

---

### Module 1.8 — Portfolio Management: Your Holdings
- [ ] **You build:** Portfolio tracker showing: current holdings, average buy price, unrealized P&L per asset, total portfolio value. Sorting by value, by P&L, alphabetically.
- [ ] **Concepts:** `BTreeMap` vs `HashMap` — sorted vs unsorted, when each fits · Advanced iterator patterns: `.zip()`, `.enumerate()`, `.flat_map()`, `.chain()`, `.take()`, `.skip()` · `Ord`, `PartialOrd` for custom sorting · `Display` trait for formatted output · Floating-point precision issues in financial calculations · `f64` vs integer-based price representation
- [ ] **Architecture:** Portfolio management at scale. How brokers calculate P&L. FIFO vs LIFO vs average cost basis.
- [ ] **Deliverable:** Portfolio view with real-time P&L calculation. Multiple sort options. Formatted table output.

---

### Module 1.9 — Order Management (Basic)
- [ ] **You build:** Submit market/limit orders, cancel pending orders, view order history with status tracking. Order lifecycle: `Pending → Filled/Cancelled/Rejected`. Order ID generation.
- [ ] **Concepts:** Builder pattern — `OrderBuilder` with method chaining · Newtype pattern — `struct OrderId(u64)` for type safety · `enum` with data variants · State machine pattern via enums · `Vec<Order>` with iterator-based filtering · Sorting and searching · `Eq`, `PartialEq`, `Hash` for ID types
- [ ] **Architecture:** Order Management System (OMS) design. Order lifecycle in real exchanges. Why order IDs must be globally unique. How Zerodha/Binance track order state.
- [ ] **Deliverable:** Full order CRUD. State transitions enforced by types. Order history with filtering by status, asset, date.

---

### Module 1.10 — File Persistence: Saving State
- [ ] **You build:** JSON-based persistence for all data: users, wallets, orders, portfolio. Load on startup, save on every state change. File locking to prevent corruption.
- [ ] **Concepts:** `serde` + `serde_json` for serialization · `#[derive(Serialize, Deserialize)]` · Serde attributes: `#[serde(rename_all = "camelCase")]`, `#[serde(default)]`, `#[serde(skip)]` · Lifetimes in structs — `struct Config<'a> { ... }` · `'static` lifetime · File I/O with proper error propagation · `PathBuf` vs `Path` (owned vs borrowed, like `String` vs `&str`)
- [ ] **Architecture:** Persistence strategies. Why JSON for dev, binary/DB for production. Write-ahead logging concept. Crash recovery.
- [ ] **Deliverable:** All state survives process restarts. Graceful handling of corrupted/missing files. Unit tests for round-trip serialization.

---

### Module 1.11 — Positions Tracking
- [ ] **You build:** Open/close position tracking. When an order fills, a position opens. Track entry price, current value, realized vs unrealized P&L. Position sizing.
- [ ] **Concepts:** Smart pointers — `Box<T>` for heap allocation · `Rc<T>` for shared ownership (positions referenced by both portfolio and order) · `RefCell<T>` for interior mutability · `Rc<RefCell<T>>` pattern · `Weak<T>` for breaking cycles · When to use each smart pointer
- [ ] **Architecture:** Position management in brokerages. Mark-to-market. Margin concepts (preview for Phase 3).
- [ ] **Deliverable:** Position tracking integrated with order fills. P&L breakdown: realized (closed positions) vs unrealized (open positions).

---

### Module 1.12 — Testing Suite
- [ ] **You build:** Comprehensive tests across ALL Phase 1 modules. Unit tests, integration tests, doc tests. Test helpers and fixtures for trading scenarios.
- [ ] **Concepts:** `#[cfg(test)] mod tests` · `#[test]`, `assert!`, `assert_eq!`, `assert_ne!` · `#[should_panic]` · `Result`-returning tests · Integration tests in `tests/` directory · Doc tests in `///` comments · Test organization and `pub(crate)` · Function pointers vs closures · Test fixtures and builders
- [ ] **Architecture:** Testing strategies in financial software. Why trading systems need exhaustive edge-case testing. Property-based testing concepts (preview).
- [ ] **Deliverable:** 90%+ test coverage. Every public API has doc tests. Integration tests for end-to-end trading flows.

---

### Module 1.13 — Multi-Module Architecture Refactoring
- [ ] **You build:** Refactor the entire codebase into clean module architecture: `src/models/`, `src/services/`, `src/storage/`, `src/errors/`, `src/cli/`, `src/config/`.
- [ ] **Concepts:** Module system deep dive — `mod.rs` vs `foo.rs` + `foo/` (modern style) · Re-exports with `pub use` · Visibility levels · `pub(crate)` vs `pub(super)` · Dependency direction — services depend on models, not vice versa · Circular dependency prevention
- [ ] **Architecture:** Clean architecture principles. Separation of concerns. Dependency inversion. How production Rust projects organize code.
- [ ] **Deliverable:** Clean module tree. No circular dependencies. Each module has a clear single responsibility.

---

### Module 1.14 — Documentation & Code Quality
- [ ] **You build:** `rustdoc` documentation for all public APIs. README.md with usage examples. Clippy configuration. Formatting rules.
- [ ] **Concepts:** `///` and `//!` doc comments · `cargo doc --open` · Intra-doc links · Code examples in docs (that compile and run as tests) · `#![deny(missing_docs)]` · Clippy lints configuration · `rustfmt.toml` customization · `#[allow(...)]` and `#[warn(...)]`
- [ ] **Architecture:** Documentation culture in production Rust. Why Rust's doc system is considered best-in-class.
- [ ] **Deliverable:** Complete API documentation. README with quickstart guide. All clippy warnings resolved.

---

### Module 1.15 — 🏁 Phase 1 Capstone: Portfolio-Ready CLI Trading Platform
- [ ] **You build:** Final polish pass. Refactor, optimize, add missing tests, improve error messages, benchmark key operations, write comprehensive README.
- [ ] **Concepts:** Review and solidify ALL Phase 1 concepts · Performance measurement with `std::time::Instant` · Code review checklist · Idiomatic Rust patterns review
- [ ] **Architecture:** Architecture review. What works, what doesn't, what Phase 2 will improve.
- [ ] **Deliverable:** A polished, tested, documented CLI trading platform. **Portfolio piece #1.**

---

**🏁 Phase 1 Deliverables Summary:**
- Complete domain model for trading (orders, assets, positions, wallets)
- User authentication with password hashing
- Multi-currency wallet with transaction history
- Portfolio management with P&L tracking
- Order management with state machine lifecycle
- File-based persistence with serde
- Comprehensive test suite
- Clean multi-module architecture
- Full documentation
- **Portfolio-ready CLI trading application**

---

## PHASE 2 — Production Backend

> **Goal:** Transform the CLI trading app into a production-grade async web service with database, caching, authentication, observability, and containerization. Learn async Rust, Tokio, Axum, and production infrastructure.
>
> **End state:** A fully functional REST API with PostgreSQL, Redis, JWT auth, Docker deployment, structured logging, and benchmarks. Portfolio piece #2.

---

### Module 2.1 — Async Foundations: Why Trading Needs Async
- [ ] **You build:** Convert the trading engine's core operations to async. Understand why an exchange handling thousands of concurrent connections can't use thread-per-request.
- [ ] **Concepts:** Why async exists — I/O-bound concurrency vs CPU-bound parallelism · `async fn`, `.await`, `Future` trait · Futures are lazy state machines (not JS promises!) · Why Rust needs an external runtime · Tokio: `#[tokio::main]`, `tokio::spawn`, tasks vs OS threads · Cooperative scheduling · `.await` as preemption points · `Send` + `Sync` bounds on futures
- [ ] **Architecture:** Thread-per-connection vs event-loop vs async. How exchanges handle 100k+ concurrent WebSocket connections. C10K problem.
- [ ] **Deliverable:** Core trading operations are async. Understanding of when async helps vs hurts.

---

### Module 2.2 — REST API: HTTP Interface for the Trading Platform
- [ ] **You build:** Axum-based REST API exposing all trading operations: `POST /orders`, `GET /portfolio`, `POST /auth/login`, `GET /wallet/balance`, etc.
- [ ] **Concepts:** Axum design: `tokio` + `hyper` + `tower` stack · Routing and handler functions · Extractors: `Path`, `Query`, `Json`, `State` · `Arc<AppState>` for shared state · Request/response JSON with serde · Custom error type implementing `IntoResponse` · HTTP status codes mapping to trading errors
- [ ] **Architecture:** REST API design for trading platforms. Endpoint naming conventions. Request/response schemas. How Binance/Coinbase structure their public APIs.
- [ ] **Deliverable:** All Phase 1 CLI operations accessible via REST API. Proper HTTP status codes for all error types.

---

### Module 2.3 — Database: PostgreSQL with Compile-Time Safety
- [ ] **You build:** Migrate from JSON file storage to PostgreSQL. Schema design for users, orders, wallets, positions. Migrations.
- [ ] **Concepts:** `sqlx` — compile-time checked SQL · `query!` and `query_as!` macros · Connection pools · Migrations · Transactions · `FromRow` derive · Async database operations · `DATABASE_URL` configuration · Schema design for financial data
- [ ] **Architecture:** Database design for trading platforms. ACID properties for financial transactions. Why you can't lose a trade record. Sharding/partitioning strategies.
- [ ] **Deliverable:** Full database integration. Migrations for all tables. All CRUD operations use sqlx. JSON storage removed.

---

### Module 2.4 — Authentication API: JWT & Middleware
- [ ] **You build:** JWT-based authentication. Login returns a token, protected routes require valid token. Token refresh. Password hashing with argon2.
- [ ] **Concepts:** `tower` middleware/layers · Custom extractors · JWT creation/validation · `tower-http` layers: CORS, compression · Middleware ordering · Request guards · `async` trait methods
- [ ] **Architecture:** Authentication in trading APIs. API key vs JWT vs OAuth. Rate limiting per user. How exchanges protect against unauthorized trading.
- [ ] **Deliverable:** Login → JWT → authenticated requests flow. Protected routes reject unauthenticated requests with 401.

---

### Module 2.5 — Redis: Caching & Session Management
- [ ] **You build:** Redis integration for session caching, rate limiting, and hot data caching (recent prices, active orders).
- [ ] **Concepts:** `redis` crate · Async Redis operations · Connection pooling (`deadpool-redis` or `bb8`) · TTL-based expiration · Pub/Sub (preview for Phase 3) · Cache-aside pattern · Cache invalidation strategies
- [ ] **Architecture:** Caching layers in trading platforms. Why market data needs sub-millisecond reads. Redis vs in-process cache vs both. Cache stampede prevention.
- [ ] **Deliverable:** Session management via Redis. Rate limiter. Cached portfolio/balance queries with TTL.

---

### Module 2.6 — Concurrency: Processing Orders in Parallel
- [ ] **You build:** Concurrent order processing pipeline. Multiple orders submitted simultaneously must not corrupt state. Read-heavy portfolio queries must not block writes.
- [ ] **Concepts:** `Arc<T>` — atomic reference counting · `Mutex<T>` vs `RwLock<T>` — exclusive vs read-heavy access · `tokio::sync::Mutex` vs `std::sync::Mutex` — when to use which · Channels: `mpsc`, `broadcast`, `watch`, `oneshot` · Deadlock prevention · Poisoning · `Send`/`Sync` marker traits
- [ ] **Architecture:** Concurrency in exchanges. Order queuing. Fair ordering. Why lock contention kills latency. Lock-free preview (Phase 3).
- [ ] **Deliverable:** Concurrent order submission without data races. Read-write separated for portfolio queries. Channel-based order processing pipeline.

---

### Module 2.7 — Tracing & Observability
- [ ] **You build:** Structured logging and distributed tracing across the trading platform. Request IDs, span contexts, performance timing for every operation.
- [ ] **Concepts:** `tracing` crate — spans, events, subscribers · `tracing-subscriber` configuration · Structured fields vs string messages · Log levels and filtering · `#[instrument]` attribute · Request tracing through middleware · Performance timing spans · JSON log output for log aggregation
- [ ] **Architecture:** Observability in production trading systems. Why structured logs matter. How firms detect latency spikes, failed trades, system anomalies.
- [ ] **Deliverable:** Every API request traced with timing. Structured JSON logs. Span context through the entire request lifecycle.

---

### Module 2.8 — Docker: Containerization & Deployment
- [ ] **You build:** Dockerfile for the trading platform. `docker-compose.yml` with PostgreSQL + Redis + the trading API. Multi-stage builds.
- [ ] **Concepts:** Docker multi-stage builds for Rust (compile stage + runtime stage) · `docker-compose` for multi-service orchestration · Environment variable configuration in containers · Health checks · Volume mounts for data persistence · Network configuration
- [ ] **Architecture:** Container orchestration for trading platforms. Why multi-stage builds reduce image size from 2GB to 50MB. Dev vs staging vs production configs.
- [ ] **Deliverable:** `docker-compose up` starts the entire stack. API accessible on configured port. Database persists between restarts.

---

### Module 2.9 — Configuration Management
- [ ] **You build:** Environment-aware configuration system. Dev/staging/production profiles. Secrets management. Configuration validation at startup.
- [ ] **Concepts:** `config` crate — layered configuration · `serde` for config deserialization · Builder pattern for config · `once_cell` / `LazyLock` for global config · Environment-based profiles · Secrets vs config (never commit secrets) · Feature flags (`[features]` in Cargo.toml)
- [ ] **Architecture:** Configuration management in production. 12-factor app principles. How trading firms manage thousands of configuration parameters.
- [ ] **Deliverable:** Profile-based configuration. Startup validation that fails fast on missing/invalid config. Feature flags for optional capabilities.

---

### Module 2.10 — Background Jobs & Scheduling
- [ ] **You build:** Background job system for: portfolio rebalancing checks, expired order cleanup, session cleanup, daily P&L snapshots.
- [ ] **Concepts:** `tokio::spawn` for background tasks · `tokio::time::interval` for periodic jobs · `tokio::select!` for cancellation · Graceful shutdown · `tokio::signal` for SIGTERM handling · `JoinSet` for managing multiple tasks · Structured concurrency patterns
- [ ] **Architecture:** Job scheduling in trading platforms. Why overnight batch processes exist. Real-time vs batch processing tradeoffs.
- [ ] **Deliverable:** Background job runner with configurable schedules. Graceful shutdown on SIGTERM. Job status monitoring.

---

### Module 2.11 — Multi-Layer Caching Strategy
- [ ] **You build:** In-process cache (LRU) + Redis cache + database. Cache warming on startup. Invalidation on writes. Cache metrics.
- [ ] **Concepts:** LRU cache implementation · `dashmap` for concurrent HashMap · Cache-aside vs write-through vs write-behind · TTL strategies · Cache coherence across instances · Metrics collection (`prometheus` or custom)
- [ ] **Architecture:** Multi-layer cache architecture. L1 (process) → L2 (Redis) → L3 (database). How HFT firms keep hot data in L1 cache for nanosecond access.
- [ ] **Deliverable:** Three-tier caching with automatic fallthrough. Cache hit/miss metrics. Invalidation on writes.

---

### Module 2.12 — Authorization & Permissions (RBAC)
- [ ] **You build:** Role-Based Access Control. Roles: `Admin`, `Trader`, `Viewer`. Permissions: `can_trade`, `can_withdraw`, `can_manage_users`. Middleware guards.
- [ ] **Concepts:** Enum-based permission modeling · Bitflag patterns for efficient permission checks · Middleware composition · Type-state pattern for authorization · `tower::Layer` for composable middleware
- [ ] **Architecture:** Authorization in financial systems. Regulatory requirements (who can approve large trades). Principle of least privilege.
- [ ] **Deliverable:** Role-based route protection. Admin-only endpoints. Permission checks on sensitive operations.

---

### Module 2.13 — Workspace Architecture: Multi-Crate Project
- [ ] **You build:** Split the monolith into a Cargo workspace: `trading-core` (domain logic), `trading-api` (REST server), `trading-cli` (CLI interface), `trading-common` (shared types).
- [ ] **Concepts:** Cargo workspaces — `[workspace]` in root `Cargo.toml` · `[workspace.dependencies]` for shared versions · Path dependencies · Crate boundaries and public APIs · `pub(crate)` vs `pub` at crate level · Inter-crate testing · Workspace-level commands
- [ ] **Architecture:** Multi-crate architecture in production Rust. Why `trading-core` has zero framework dependencies. Dependency inversion at crate level.
- [ ] **Deliverable:** Clean workspace with 4+ crates. Each crate has focused responsibility. Shared types defined once.

---

### Module 2.14 — Benchmarking & Profiling
- [ ] **You build:** `criterion` benchmarks for hot paths: order creation, portfolio P&L calculation, serialization, database queries. Establish baselines.
- [ ] **Concepts:** `criterion` crate — statistical benchmarking · `cargo bench` · Benchmark groups and comparisons · `std::hint::black_box` · Profiling with `perf` or `cargo flamegraph` · Identifying hot paths · Optimization: algorithmic vs micro
- [ ] **Architecture:** Performance culture in trading. Why latency percentiles matter (p50, p99, p99.9). How firms benchmark matching engines.
- [ ] **Deliverable:** Benchmark suite with baselines. Flamegraph analysis. At least one optimization with before/after comparison.

---

### Module 2.15 — 🏁 Phase 2 Capstone: Production-Ready Trading API
- [ ] **You build:** Final production hardening. Load testing, security review, API documentation (OpenAPI/Swagger), deployment guide.
- [ ] **Concepts:** Review all Phase 2 concepts · API versioning · Rate limiting · Security headers · Input validation · SQL injection prevention (sqlx handles this) · Load testing with `hey` or `wrk`
- [ ] **Architecture:** Production readiness checklist for financial software. What would a code review at Zerodha look for?
- [ ] **Deliverable:** A production-grade REST API with database, caching, auth, observability, Docker deployment. **Portfolio piece #2.**

---

**🏁 Phase 2 Deliverables Summary:**
- Async trading engine on Tokio
- REST API with Axum (all trading operations)
- PostgreSQL with compile-time checked SQL
- JWT authentication & RBAC authorization
- Redis caching & session management
- Concurrent order processing with channels
- Structured tracing & observability
- Docker deployment with compose
- Multi-crate workspace architecture
- Benchmark suite with profiling
- **Production-ready trading API**

---

## PHASE 3 — Trading Infrastructure & HFT Concepts

> **Goal:** Build the core trading infrastructure that makes a platform feel real: order book, matching engine, risk engine, market data feeds, WebSocket streaming, and performance optimizations inspired by HFT firms.
>
> **End state:** A complete trading ecosystem with low-latency matching, real-time streaming, risk management, and production-grade performance. Portfolio piece #3 — the final showpiece.

---

### Module 3.1 — Order Book: The Heart of an Exchange
- [ ] **You build:** A proper order book with price-time priority. Bid and ask sides. Level 2 market data (price levels with aggregated quantities). Best bid/ask (BBO). Spread calculation.
- [ ] **Concepts:** Data structure design for performance · `BTreeMap<Price, VecDeque<Order>>` for price levels · Custom `Ord` implementations · `VecDeque` for FIFO order within price level · Memory layout considerations · Cache-friendly data structures
- [ ] **Architecture:** How order books work at NYSE, NASDAQ, Binance. Price-time priority vs pro-rata. L1 vs L2 vs L3 market data. How market makers use order book data.
- [ ] **Deliverable:** Working order book with add/cancel/modify operations. BBO calculation. L2 snapshot generation.

---

### Module 3.2 — Matching Engine: Where Orders Become Trades
- [ ] **You build:** FIFO matching engine. When a new order arrives, match it against the opposite side of the book. Partial fills. Trade generation. Order lifecycle management.
- [ ] **Concepts:** Matching algorithms (FIFO, pro-rata) · State machine formalization · `enum` with complex data · Pattern matching for state transitions · `Result` chains for multi-step operations · Performance-critical data structure choices
- [ ] **Architecture:** How matching engines work at major exchanges. Throughput requirements (millions of orders/second). Determinism requirements. Why matching engines are single-threaded by design.
- [ ] **Deliverable:** Working matching engine. Market and limit order matching. Partial fills. Trade event generation. Benchmark: orders matched per second.

---

### Module 3.3 — Risk Engine: Preventing Catastrophe
- [ ] **You build:** Pre-trade risk checks: position limits, order size limits, buying power validation, fat-finger prevention, circuit breakers. Post-trade risk: exposure calculation, margin requirements.
- [ ] **Concepts:** Trait objects for pluggable risk rules (`Vec<Box<dyn RiskCheck>>`) · Dynamic dispatch for extensibility · Builder pattern for composable risk rules · Error accumulation (collect all failures, not just first) · `Result` with custom error types containing risk details
- [ ] **Architecture:** Risk management at trading firms. Pre-trade vs post-trade vs real-time risk. How Knight Capital lost $440M in 45 minutes. Circuit breakers. Regulatory requirements.
- [ ] **Deliverable:** Pluggable risk engine. Multiple risk checks composed together. Every order passes through risk before matching.

---

### Module 3.4 — Market Data Feed: Simulated Prices
- [ ] **You build:** Simulated market data generator producing realistic tick data, OHLCV candles, and price movements. Random walk with drift. Orderbook-derived prices.
- [ ] **Concepts:** `rand` crate · Floating-point math in Rust · `Duration`, `Instant`, `SystemTime` · Iterator-based data generation · `impl Iterator for MarketDataFeed` · Lazy evaluation for infinite streams · `Channel`-based data distribution
- [ ] **Architecture:** How market data feeds work (Reuters, Bloomberg). FIX protocol overview. Tick-by-tick vs snapshot. How exchanges disseminate data.
- [ ] **Deliverable:** Market data generator producing realistic price streams. OHLCV candle aggregation. Historical data storage.

---

### Module 3.5 — WebSocket Streaming: Real-Time Data
- [ ] **You build:** WebSocket server streaming: live order book updates, trade feeds, portfolio updates, price tickers. Client can subscribe to specific channels.
- [ ] **Concepts:** `tokio-tungstenite` for WebSocket · `tokio::sync::broadcast` for pub/sub · Subscription management · JSON and binary message formats · Connection lifecycle · Heartbeat/ping-pong · Reconnection handling · Backpressure
- [ ] **Architecture:** WebSocket architecture at Binance/Coinbase. Channel-based subscriptions. How exchanges handle 100k+ concurrent WebSocket connections. Message ordering guarantees.
- [ ] **Deliverable:** WebSocket server with channel subscriptions. Real-time order book, trades, and ticker streams. Connection management with heartbeat.

---

### Module 3.6 — TCP Networking: Custom Binary Protocol
- [ ] **You build:** Custom TCP protocol for low-latency order submission. Binary message format. Length-prefixed framing. Connection multiplexing.
- [ ] **Concepts:** `tokio::net::TcpListener` / `TcpStream` · `tokio::io::AsyncRead` / `AsyncWrite` · `bytes` crate — `BytesMut`, `Buf`, `BufMut` · `tokio_util::codec` — `Encoder`/`Decoder` pattern · Framing: length-prefixed vs delimiter · Binary serialization (`bincode` or custom)
- [ ] **Architecture:** Why HFT firms use TCP not HTTP. FIX protocol vs proprietary protocols. Kernel bypass (DPDK, io_uring). Colocation.
- [ ] **Deliverable:** Custom TCP server accepting binary-encoded orders. Lower latency than REST API. Benchmark comparison: TCP vs HTTP for order submission.

---

### Module 3.7 — Memory Optimizations
- [ ] **You build:** Arena allocator for order objects. Object pool for reusable buffers. Cache-line aligned structures. Memory-mapped I/O for historical data.
- [ ] **Concepts:** `#[repr(C)]` for memory layout control · `#[repr(align(N))]` for cache-line alignment · `std::alloc` — custom allocators · Arena allocation pattern · Object pool pattern · `memmap2` for memory-mapped files · Stack vs heap allocation strategies · `SmallVec` for small-buffer optimization
- [ ] **Architecture:** Memory management in HFT. Why allocation in the hot path is forbidden. How Jane Street/Jump Trading think about memory. NUMA-awareness.
- [ ] **Deliverable:** Arena allocator for orders. Object pool for network buffers. Benchmark: allocation-free hot path.

---

### Module 3.8 — Unsafe Rust: When Safety Isn't Enough
- [ ] **You build:** Unsafe blocks for: raw pointer manipulation in the arena allocator, SIMD-accelerated price calculations (if available), FFI wrapper for a C library.
- [ ] **Concepts:** `unsafe` blocks and `unsafe fn` — what invariants you're promising · Raw pointers `*const T`, `*mut T` · Dereferencing raw pointers · `unsafe impl Send/Sync` · FFI: `extern "C"`, `#[no_mangle]`, `libc` · SIMD: `std::arch` intrinsics · Miri for undefined behavior detection
- [ ] **Architecture:** Where unsafe is justified in production. How Tokio, crossbeam, and std use unsafe internally. Code review process for unsafe blocks.
- [ ] **Deliverable:** Unsafe arena allocator with safety invariants documented. Miri passes. Performance comparison with safe version.

---

### Module 3.9 — Macros: Code Generation for Protocols
- [ ] **You build:** Declarative macros for: message type definitions, serialization boilerplate, test case generation. Procedural macro for deriving a custom `Protocol` trait.
- [ ] **Concepts:** Declarative macros (`macro_rules!`) — patterns, repetitions, hygiene · `#[derive(...)]` procedural macros · `syn` and `quote` crates · `proc-macro2` · When macros vs generics vs code generation · Macro debugging with `cargo expand`
- [ ] **Architecture:** Macro usage in production Rust. How `serde_derive`, `tokio::select!`, and `clap` use proc macros internally. When macros are the right tool.
- [ ] **Deliverable:** Declarative macros reducing protocol boilerplate. Custom derive macro for `Protocol` trait. `cargo expand` output showing generated code.

---

### Module 3.10 — Zero-Copy Parsing: Maximum Throughput
- [ ] **You build:** Zero-copy binary message parser. Parse incoming network messages without allocating new buffers. Borrow directly from the network buffer.
- [ ] **Concepts:** `nom` or custom parser combinators · Lifetime annotations for borrowed parsed data · `&[u8]` slice parsing · `zerocopy` crate · Endianness handling · Alignment requirements · `std::mem::transmute` (unsafe) vs safe casting
- [ ] **Architecture:** Zero-copy parsing in HFT. Why every allocation in the hot path costs microseconds. How network cards DMA data directly into user-space buffers.
- [ ] **Deliverable:** Zero-copy message parser. Benchmark: parsed messages per second with zero allocations.

---

### Module 3.11 — Profiling & Performance Analysis
- [ ] **You build:** Comprehensive performance analysis of the entire system. Flamegraphs, cache miss analysis, branch prediction analysis. Identify and fix top bottlenecks.
- [ ] **Concepts:** `cargo flamegraph` · `perf stat` / `perf record` · CPU cache hierarchy (L1/L2/L3) · Cache misses and how data layout affects them · Branch prediction · Instruction-level parallelism · `criterion` for micro-benchmarks · Amdahl's law
- [ ] **Architecture:** Performance engineering methodology. Profile → identify → hypothesize → fix → measure. Why premature optimization is bad but performance awareness is essential.
- [ ] **Deliverable:** Flamegraph analysis report. At least 3 optimizations with measured before/after improvements.

---

### Module 3.12 — Thread Pools & Lock-Free Concepts
- [ ] **You build:** Custom thread pool for CPU-bound work. Lock-free SPSC queue for order passing between threads. Atomic operations for shared counters.
- [ ] **Concepts:** `crossbeam` crate — scoped threads, channels, atomic data structures · `std::sync::atomic` — `AtomicU64`, `Ordering` (Relaxed, Acquire, Release, SeqCst) · Compare-and-swap (CAS) · Lock-free vs wait-free · Memory ordering and the CPU memory model · `crossbeam::queue::SegQueue` · Thread pinning / CPU affinity
- [ ] **Architecture:** Lock-free programming in HFT. Why mutexes are too slow for matching engines. LMAX Disruptor pattern. Mechanical sympathy.
- [ ] **Deliverable:** Lock-free SPSC queue. Atomic order ID generator. Benchmark: lock-free vs mutex-based queue throughput.

---

### Module 3.13 — Event-Driven Architecture
- [ ] **You build:** Event bus for the entire system. Events: `OrderSubmitted`, `OrderMatched`, `TradeExecuted`, `PositionUpdated`, `RiskBreached`. CQRS pattern for read/write separation.
- [ ] **Concepts:** Event sourcing concepts · `enum` for event types with data · `tokio::sync::broadcast` for event distribution · Event handlers as trait implementations · CQRS: separate read and write models · Event replay for state reconstruction · `async` event processing
- [ ] **Architecture:** Event-driven architecture in exchanges. How trade events flow through post-trade systems. Audit trails. Compliance recording.
- [ ] **Deliverable:** Event bus with typed events. Multiple consumers (logging, risk, notification). Event replay capability.

---

### Module 3.14 — Plugin System: Extensible Architecture
- [ ] **You build:** Plugin system allowing: custom risk checks, custom matching algorithms, custom market data sources, custom notification handlers. Hot-reloadable configuration.
- [ ] **Concepts:** Trait objects for plugin interfaces · `Box<dyn Plugin>` · Dynamic loading with `libloading` (optional) · Config-driven plugin selection · `inventory` crate for plugin registration · Strategy pattern · Composition over inheritance
- [ ] **Architecture:** Plugin architectures in production systems. How exchanges allow member firms to customize behavior. Strategy pattern in trading algorithms.
- [ ] **Deliverable:** Plugin system with example plugins for risk, matching, and notifications. New plugins addable without modifying core code.

---

### Module 3.15 — 🏁 Phase 3 Capstone: Complete Trading Ecosystem
- [ ] **You build:** Final integration, optimization, and polish. End-to-end flow: market data → order submission → risk check → matching → trade → position update → WebSocket notification. Load testing. Documentation.
- [ ] **Concepts:** System integration · End-to-end testing · Load testing · Performance regression testing · Architecture documentation · API documentation · Deployment automation
- [ ] **Architecture:** Complete system architecture review. What would need to change for actual production deployment. Regulatory considerations. Scaling strategies.
- [ ] **Deliverable:** Complete trading ecosystem. End-to-end demo. Architecture documentation. Performance report. **Portfolio piece #3 — the final showpiece.**

---

**🏁 Phase 3 Deliverables Summary:**
- Order book with price-time priority
- FIFO matching engine with partial fills
- Pre-trade & post-trade risk engine
- Simulated market data feed
- WebSocket streaming (real-time order book, trades, tickers)
- Custom TCP binary protocol for low-latency orders
- Arena allocator & object pool (memory optimization)
- Unsafe Rust for performance-critical paths
- Declarative & procedural macros
- Zero-copy binary message parsing
- Flamegraph profiling & optimization
- Lock-free data structures & atomic operations
- Event-driven architecture with CQRS
- Plugin system for extensibility
- **Complete, production-grade trading ecosystem**

---

## 📊 Rust Concepts by Phase (Cross-Reference)

| Phase | Rust Concepts Covered |
|---|---|
| **Phase 1** | Variables, types, structs, enums, match, ownership, borrowing, lifetimes, traits, generics, error handling, Result/Option, iterators, closures, collections, HashMap/BTreeMap, serde, smart pointers (Box/Rc/RefCell), modules, testing, documentation, clippy/fmt |
| **Phase 2** | async/await, Future trait, Tokio runtime, Axum, sqlx, Arc, Mutex, RwLock, channels (mpsc/broadcast/watch/oneshot), Send/Sync, tracing, Docker, feature flags, workspaces, criterion benchmarking, tower middleware |
| **Phase 3** | Unsafe, raw pointers, FFI, SIMD, macros (declarative + procedural), zero-copy parsing, arena allocators, atomics, lock-free structures, memory ordering, custom allocators, repr(C), binary protocols, thread pinning |
