# 🦀 Production-Grade Trading Platform in Rust

> A hands-on, zero-to-hero journey building a high-frequency, production-grade cryptocurrency and equity trading platform engine from scratch in Rust — guided by an interactive AI pair-programming curriculum system.

---

## 📌 Project Overview & End Goal

The **Apex Trading Platform Engine** is designed to model real-world institutional financial infrastructure (*Binance, Zerodha, NASDAQ, Interactive Brokers*). 

The goal of this project is twofold:
1. **System Capability**: Build a single-binary, memory-safe, ultra-fast trading platform engine supporting user registration, dual-indexed password hashing, multi-currency wallet ledgers with overdraft protection, portfolio position tracking with weighted average cost basis, order builder state machines, file persistence, limit orderbooks, and price feeds.
2. **Mastery of Modern Systems Programming**: Deeply understand Rust's core ownership model, zero-cost abstractions, trait contracts, error propagation, atomic map mutations, generic data structures, generic serialization (`serde`), iterator pipelines, and safe floating-point arithmetic without resorting to lazy fallbacks or unsafe code.

---

## 🗺️ Project Architecture & Curriculum Roadmap

The project is structured into progressive learning phases:

```
+-----------------------------------------------------------------------+
|                    TRADING PLATFORM ENGINE ARCHITECTURE               |
|                                                                       |
|  +-----------------------------------------------------------------+  |
|  | CLI Parsing & Command Dispatch (clap v4)                        |  |
|  +-----------------------------------------------------------------+  |
|          |                                                            |
|          v                                                            |
|  +-----------------------------------------------------------------+  |
|  | User & Authentication Manager (uuid v4 + SHA-256 Hashing)       |  |
|  +-----------------------------------------------------------------+  |
|          |                                                            |
|          v                                                            |
|  +-----------------------------------------------------------------+  |
|  | Multi-Currency Wallet Engine (HashMap Entry API + Audit Log)    |  |
|  +-----------------------------------------------------------------+  |
|          |                                                            |
|          v                                                            |
|  +-----------------------------------------------------------------+  |
|  | Portfolio Manager (Position Averaging + Real-Time P&L Sorting)  |  |
|  +-----------------------------------------------------------------+  |
|          |                                                            |
|          v                                                            |
|  +-----------------------------------------------------------------+  |
|  | Order Management System (Newtype OrderId + Builder State Machine)| |
|  +-----------------------------------------------------------------+  |
|          |                                                            |
|          v                                                            |
|  +-----------------------------------------------------------------+  |
|  | File Persistence Storage Engine (Serde JSON + Disk I/O)         |  |
|  +-----------------------------------------------------------------+  |
+-----------------------------------------------------------------------+
```

### Phase 1: Core Engine & Single-Binary Architecture
- [x] **Module 1.1 — Workspace & Environment Setup**: Rust edition 2024, `Cargo.toml`, project layout.
- [x] **Module 1.2 — System Architecture & Component Flow**: Monolith module structure and data flow.
- [x] **Module 1.3 — Domain Types & Data Models**: Memory layouts, enums, structs, constructors.
- [x] **Module 1.4 — Configuration Management**: TOML file parsing, environment overrides, fallback defaults.
- [x] **Module 1.5 — Production Error Handling**: Custom error enums with `thiserror`, `Result<T>` type aliasing, `?` operator desugaring.
- [x] **Module 1.6 — User & Authentication System**: `Uuid` identity generation, SHA-256 password hashing, dual-index `HashMap` lookup.
- [x] **Module 1.7 — Wallet System (Money Management)**: Atomic map mutation (`HashMap::entry`), overdraft protection, `TransactionRecord` audit history, iterator filter chains.
- [x] **Module 1.8 — Portfolio Management (Your Holdings)**: Position tracking, weighted average cost basis updates, real-time unrealized P&L calculations, custom sorting (`sort_by`, `PartialOrd::partial_cmp`).
- [x] **Module 1.9 — Order Management (Basic OMS)**: Type-safe `OrderId(pub u64)` newtype, `OrderStatus` state machine transitions, `OrderBuilder` with fluent method chaining and atomic validation.
- [x] **Module 1.10 — File Persistence (Saving State)**: Generic `StorageEngine` serialization and deserialization (`serde` + `serde_json`), disk I/O, `PathBuf` vs `&Path`.
- [x] **Module 1.11 — Position Tracking**: Realized vs unrealized P&L breakdown, position sizing, smart pointer memory model.
- [x] **Module 1.12 — Testing Suite**: Integration tests in `tests/`, doc tests in `///`, `#[should_panic]` test assertions.
- [x] **Module 1.13 — Multi-Module Architecture Refactoring**: Clean 6-domain subtree refactoring (`models/`, `services/`, `storage/`, `errors/`, `cli/`, `config/`).
- [x] **Module 1.14 — Documentation & Code Quality**: Inner module docs (`//!`), intra-doc links, `rustfmt.toml`, `clippy.toml`, `#![warn(missing_docs)]`.
- [x] **Module 1.15 — 🏁 Phase 1 Capstone**: Final polish pass, execution latency benchmark telemetry (`std::time::Instant`), comprehensive README documentation.


---

## 📦 Project Dependencies & Crates

| Dependency | Purpose | Key Features Used |
| :--- | :--- | :--- |
| **`serde`** | High-performance serialization framework | `#[derive(Serialize, Deserialize)]`, `rename_all`, attributes |
| **`serde_json`** | JSON data formatting & file I/O | `to_string_pretty`, `from_str` |
| **`toml`** | Configuration file parser | TOML string deserialization into `Config` struct |
| **`clap`** | CLI argument parsing & command routing | Derive macros `#[derive(Parser, Subcommand)]` |
| **`thiserror`** | Production error enum management | `#[derive(Error)]`, `#[error("...")]`, `#[from]` automatic conversions |
| **`uuid`** | Globally unique identifier generation | `Uuid::new_v4()` for user IDs |
| **`sha2`** | Cryptographic hashing | `Sha256` digest hashing for secure password storage |
| **`chrono`** | Timezone-aware timestamping | `DateTime<Utc>`, `Utc::now()` for audit logs & order timestamps |

---

## 🤖 AI Learning System Architecture & File Setup

This repository contains a **self-contained, autonomous AI-assisted learning framework**. If you open this workspace in an AI coding assistant (such as **Antigravity** / **Gemini AI**), the AI reads the governance files to guide you step-by-step through exercises without revealing solutions upfront!

### Governance & Context File Landscape:
1. **`RULES.md`**: 15 non-negotiable governance rules enforcing zero-solution pre-exposure, exact word-for-word analogy persistence, and log self-auditing.
2. **`ROADMAP.md`**: The master curriculum index containing every module's deliverables, concepts, and architectural patterns.
3. **`LEARNING.md`**: The learner's source-of-truth progress log tracking completed modules (`[x]`), what was understood, and files written.
4. **`EXAMPLES.md`**: A permanent repository of ELI5 analogies and deep technical breakdowns for every concept taught in the project.
5. **`EXERCISES.md`**: Open hands-on exercise skeletons with `todo!()` blocks and tiered hint counters (0/3).
6. **`SOLUTIONS.md`**: Gated reference implementations and line-by-line breakdowns (unlocked ONLY after a valid attempt is submitted).
7. **`LOGS.md`**: Complete `git log --patch` equivalent recording full diff blocks for every file change in the workspace.
8. **`PROMPTS.md`**: Chronological transcript log of prompts, questions, compiler error resolutions, and step transitions.
9. **`DECISIONS.md` & `QUESTIONS.md`**: Architectural decision records (ADRs) and revisit tracking for tricky topics.
10. **`.agents/skills/next/SKILL.md`**: The `/next` workflow skill instructing the AI on the exact 10-step algorithm to read, teach, evaluate, and advance curriculum steps.

---

## 🚀 How to Use & Learn From This Project

### Option A: Interactive Learning with AI (/next Workflow)
1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/trading-platform.git
   cd trading-platform
   ```
2. **Open in AI-Enabled IDE** (e.g., Antigravity).
3. **Run the `/next` command** in chat:
   - The AI will inspect `LEARNING.md`, `ROADMAP.md`, and `EXERCISES.md`.
   - It will teach **one concept at a time** using ELI5 analogies and technical breakdowns.
   - It will generate an exercise skeleton in `EXERCISES.md`.
4. **Write your code** in `src/`.
5. **Verify with compiler**:
   ```bash
   cargo check
   ```
6. **Submit your attempt**: Type `Done /next` in chat. The AI will check your implementation, compare it against `SOLUTIONS.md`, update tracking files, and advance you to the next step!

---

### Option B: Building & Running Manually

```bash
# Check compilation across all modules
cargo check

# Build production binary
cargo build --release

# Run CLI commands (once CLI dispatch is linked)
cargo run -- --help
```

---

## 📜 License & Acknowledgments

Built for mastering high-performance systems engineering, memory safety, and financial engine design in **Rust 2024**.
