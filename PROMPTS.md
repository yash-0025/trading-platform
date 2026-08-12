# 📜 PROMPTS.md — Complete Session Context Log

> This file records every prompt and its output summary so you can port context to any AI tool at any time.
> **Updated after every prompt/response exchange.**

---

## Entry Format
```
### [Timestamp] — Prompt #N
**Prompt:** <the user's prompt, summarized or quoted>
**Context:** <what files were referenced, what state we were in>
**Output Summary:** <what the AI responded with, key decisions, code shown, exercises given>
**Outcome / Next Step:** <what the learner should do next>
```

---

### 2026-07-27 23:04 IST — Prompt #1
**Prompt:** Master setup prompt — "Just like this i want you to create a new folder in C:\Dev\Rust-Projects and help me setup this giant project to learn like we learn in here." Full specification of the trading ecosystem project: role definition, project scope, 3-phase roadmap structure, teaching style, governance rules, required files, and learning philosophy.
**Context:** User has an existing Rust learning project at `C:\Dev\Rust` with 2+ weeks of progress (CLI tools, generics, type system, smart pointers, beginning async/concurrency). This new project is a separate, parallel learning track focused on building ONE evolving trading platform instead of many small projects.
**Output Summary:** AI studied the existing `C:\Dev\Rust` project structure (ROADMAP.md, LEARNING.md, LOGS.md, PROMPT_HISTORY.md, EXAMPLES.md, RULES.md, KICKOFF_PROMPT.md) to understand the governance format. Created an implementation plan with 13 files across governance, reference, and source categories. User approved the plan. AI created all files.
**Outcome / Next Step:** Confirm all files exist. Run `cargo check` to verify project compiles. Then begin Module 1.1 — Project Setup & Cargo Fundamentals.

---

### 2026-07-28 02:41 IST — Prompt #2
**Prompt:** "Let;s do one thing will delete the cargo.toml and rust-toolchain.toml and will setup the project from scratch just add .gitignore"
**Context:** User prefers to manually initialize Cargo manifests from scratch during Module 1.1 instead of using pre-created templates.
**Output Summary:** AI created [.gitignore](file:///C:/Dev/Rust-Projects/trading-platform/.gitignore) with standard Rust/Cargo/IDE ignore rules and logged it in LOGS.md. Advised user on manually deleting `Cargo.toml` and `rust-toolchain.toml` to begin Module 1.1 setup from a completely clean slate.
**Outcome / Next Step:** User deletes `Cargo.toml` and `rust-toolchain.toml` manually, then we start Module 1.1 System Design & Setup!

---

### 2026-08-06 15:01 IST — Prompt #3
**Prompt:** "/next check everything is setup and let's start"
**Context:** Kickoff of Module 1.1 — Project Setup & Cargo Fundamentals.
**Output Summary:** AI verified workspace files, presented the Grand Overview, single-binary architecture diagram, memory flow, and introduced Concept 6 (The Cargo Mental Model / Restaurant Kitchen analogy). Instructed user to run `rustc --version` and manually construct `Cargo.toml`, `rust-toolchain.toml`, and `src/main.rs`.
**Outcome / Next Step:** User creates project files and runs `cargo run`.

---

### 2026-08-06 15:21 IST — Prompt #4
**Prompt:** "Why i don't see rules being followed properly the analogies is not bein added in Examples.md"
**Context:** User called out Rule 8 violation (analogy presented in response was not automatically committed to EXAMPLES.md).
**Output Summary:** AI acknowledged the mistake, immediately updated EXAMPLES.md with Concept 6 (The Restaurant Kitchen: rustup, rustc, cargo, Cargo.toml, Cargo.lock), and recorded the edit in LOGS.md.
**Outcome / Next Step:** User continues with Module 1.1 manual file setup (`Cargo.toml`, `rust-toolchain.toml`, `src/main.rs`).

---

### 2026-08-06 16:50 IST — Prompt #5
**Prompt:** "/next let's move to next thing here's the output of cargo run - ..."
**Context:** User completed manual creation of `Cargo.toml`, `rust-toolchain.toml`, and `src/main.rs` and ran `cargo run` successfully.
**Output Summary:** AI validated `cargo run` output, taught Concept 7 (`&str` vs `String` fat pointer vs smart pointer, Billboard vs Notepad analogy), committed Concept 7 to EXAMPLES.md, logged in LOGS.md, and proposed completion of Module 1.1 with updates to LEARNING.md and ROADMAP.md.
**Outcome / Next Step:** User runs `cargo clippy` / `cargo fmt`, approves roadmap/learning log updates, and moves to Module 1.2.

---

### 2026-08-06 17:00 IST — Prompt #6
**Prompt:** "Why the analogy provided has some lines missing in Examples.md ? I want it to be same word to word in the Examples.md update the rules in rules.md..."
**Context:** User called out that Concept 7 in `EXAMPLES.md` was summarized rather than saved exact word-for-word as presented in chat response.
**Output Summary:** AI updated Concept 7 in EXAMPLES.md to match exact word-for-word response text. Updated Governance Rule 8 in RULES.md and STEP 3 line 5 in `.agents/workflows/next.md` to strictly mandate exact word-for-word storage without paraphrasing or line omissions. Recorded diffs in LOGS.md.
**Outcome / Next Step:** User approves LEARNING.md and ROADMAP.md updates for Module 1.1 completion.

---

### 2026-08-06 18:25 IST — Prompt #7
**Prompt:** "Yes update first then i will ask to tell next"
**Context:** Learner explicitly approved marking Module 1.1 complete in LEARNING.md and ROADMAP.md.
**Output Summary:** AI updated LEARNING.md (marked Module 1.1 `[x]` with summary of concepts understood and progress made) and ROADMAP.md (checked off all Module 1.1 items). Logged both edits in LOGS.md.
**Outcome / Next Step:** Await user prompt to kick off Module 1.2 — Domain Types.

---

### 2026-08-06 18:51 IST — Prompt #8
**Prompt:** "/next let's move to next step"
**Context:** Resuming curriculum via `/next` workflow algorithm. Module 1.1 complete `[x]`, moving to Module 1.2 — Domain Types.
**Output Summary:** AI presented Module 1.2 overview, Domain Types architecture diagram, and Concept 8 (Enums as ADTs - Multi-Tool Switch analogy). Saved Concept 8 word-for-word into EXAMPLES.md, created Exercise 1.2-1 skeleton in EXERCISES.md, and recorded diffs in LOGS.md.
**Outcome / Next Step:** Learner attempts Exercise 1.2-1 in EXERCISES.md.

---

### 2026-08-07 12:15 IST — Prompt #9
**Prompt:** "Done check models.rs and then we can move to /next"
**Context:** Learner implemented `Side` and `OrderType` enums in `src/models.rs`.
**Output Summary:** AI checked `src/models.rs`, opened gated Solution 1.2-1 in SOLUTIONS.md, moved Exercise 1.2-1 to solved in EXERCISES.md, introduced Concept 9 (Newtype Pattern & Structs - Currency Wallets & Trading Tickets analogy), committed Concept 9 to EXAMPLES.md word-for-word, created Exercise 1.2-2 skeleton in EXERCISES.md, and recorded diffs in LOGS.md.
**Outcome / Next Step:** Learner attempts Exercise 1.2-2 (`Price`, `Quantity`, `Order`) in `src/models.rs`.

---

### 2026-08-07 14:00 IST — Prompt #10
**Prompt:** "Done /next"
**Context:** Learner implemented `Price`, `Quantity`, `OrderStatus`, and `Order` in `src/models.rs`.
**Output Summary:** AI checked `src/models.rs` (noted `ParitalEq` typo), opened gated Solution 1.2-2 in SOLUTIONS.md, moved Exercise 1.2-2 to solved in EXERCISES.md, introduced Concept 10 (Method Receivers `&self`/`&mut self`/`self` and Constructors `Self::new()` - Car Dashboard, Mechanic's Wrench, and Crusher analogy), committed Concept 10 to EXAMPLES.md word-for-word, created Exercise 1.2-3 skeleton in EXERCISES.md, and recorded diffs in LOGS.md.
**Outcome / Next Step:** Learner fixes `ParitalEq` typo and implements `impl Order` constructor and methods in `src/models.rs`.

---

### 2026-08-07 15:11 IST — Prompt #11
**Prompt:** "Done check and let's move /next"
**Context:** Learner completed `impl Order` methods (`new`, `fill`, `is_pending`) in `src/models.rs`.
**Output Summary:** AI validated `src/models.rs` implementation (exact match on struct field init, `&mut self` mutation, and `&self` equality comparison), opened gated Solution 1.2-3 in SOLUTIONS.md, moved Exercise 1.2-3 to solved in EXERCISES.md, recorded diffs in LOGS.md, verified Module 1.2 completion per Rule 14, and proposed marking Module 1.2 complete in LEARNING.md and ROADMAP.md.
**Outcome / Next Step:** Learner approves LEARNING.md and ROADMAP.md updates for Module 1.2 completion, then moves to Module 1.3 (Configuration System).

---

### 2026-08-07 23:41 IST — Prompt #12
**Prompt:** "/next"
**Context:** Resuming via `/next` workflow algorithm. Learner approved Module 1.2 completion.
**Output Summary:** AI updated LEARNING.md (marked Module 1.2 `[x]`) and ROADMAP.md (checked off Module 1.2 items), logged edits in LOGS.md, kicked off Module 1.3 (Configuration System) with Overview, ASCII Hierarchy diagram, Concept 11 (Ownership Deep Dive - Deed vs Photo analogy), committed Concept 11 word-for-word to EXAMPLES.md, created Exercise 1.3-1 skeleton in EXERCISES.md, and recorded diffs in LOGS.md.
**Outcome / Next Step:** Learner attempts Exercise 1.3-1 (`Config` struct & `from_env_or_default`).

---

### 2026-08-08 14:03 IST — Prompt #13
**Prompt:** "I need hints for writing impl Config and from_env_or_dafault function"
**Context:** Learner requested Hint 1 for Exercise 1.3-1 (`Config::from_env_or_default`).
**Output Summary:** AI bumped `Hints used: 1/3` in EXERCISES.md, recorded diff in LOGS.md, and provided Hint Tier 1 (conceptual nudge on environment variable inspection and fallback strategy without naming functions or code).
**Outcome / Next Step:** Learner attempts `Config::from_env_or_default()` implementation or asks for Hint Tier 2.

---

### 2026-08-08 14:05 IST — Prompt #14
**Prompt:** "I don't know how to write this with syntax"
**Context:** Learner requested Hint 2 (structural/syntax hint) for Exercise 1.3-1 (`Config::from_env_or_default`).
**Output Summary:** AI bumped `Hints used: 2/3` in EXERCISES.md, recorded diff in LOGS.md, and provided Hint Tier 2 naming standard functions (`std::env::var`), method combinators (`unwrap_or_else`), and string conversions without providing full solution code.
**Outcome / Next Step:** Learner attempts `Config::from_env_or_default()` using `std::env::var` and `unwrap_or_else` or asks for Hint Tier 3 / solution.

---

### 2026-08-08 14:47 IST — Prompt #15
**Prompt:** "Done /next"
**Context:** Learner attempted `from_env_or_default` logic in `src/config.rs`.
**Output Summary:** AI checked `src/config.rs` (noted placing code inside function body, `=>` syntax for `match`, and `std::env::var`), opened gated Solution 1.3-1 in SOLUTIONS.md, moved Exercise 1.3-1 to solved in EXERCISES.md, introduced Concept 12 (File Parsing with TOML & Layered Fallback - Restaurant Menu Book vs Verbal Daily Specials analogy), committed Concept 12 to EXAMPLES.md word-for-word, created Exercise 1.3-2 skeleton in EXERCISES.md, and recorded diffs in LOGS.md.
**Outcome / Next Step:** Learner adjusts `src/config.rs` syntax and implements Exercise 1.3-2 (`Config::from_file_or_env`).

---

### 2026-08-10 14:17 IST — Prompt #16
**Prompt:** "Done /next"
**Context:** Learner completed `Config::from_file_or_env` in `src/config.rs`.
**Output Summary:** AI validated `src/config.rs` implementation, opened gated Solution 1.3-2 in SOLUTIONS.md, moved Exercise 1.3-2 to solved in EXERCISES.md, introduced Concept 13 (Serde TOML Deserialization `#[derive(Deserialize)]` & `toml::from_str` - Automated Customs Scanner analogy), committed Concept 13 to EXAMPLES.md word-for-word, created Exercise 1.3-3 skeleton in EXERCISES.md, and recorded diffs in LOGS.md.
**Outcome / Next Step:** Learner adds `serde` & `toml` to `Cargo.toml`, derives `Deserialize`, and implements TOML parsing in `from_file_or_env`.

---

### 2026-08-10 23:10 IST — Prompt #17
**Prompt:** "done /next"
**Context:** Learner completed `serde` dependency setup and `toml::from_str` deserialization in `Cargo.toml` and `src/config.rs`.
**Output Summary:** AI validated `src/config.rs` and `Cargo.toml`, opened gated Solution 1.3-3 in SOLUTIONS.md, moved Exercise 1.3-3 to solved in EXERCISES.md, recorded diffs in LOGS.md, verified Module 1.3 completion per Rule 14, and proposed marking Module 1.3 complete in LEARNING.md and ROADMAP.md.
**Outcome / Next Step:** Learner approves LEARNING.md and ROADMAP.md updates for Module 1.3 completion, then moves to Module 1.4 (Order Management Engine - In-Memory State & Collections).

---

### 2026-08-10 23:26 IST — Prompt #18
**Prompt:** "Yes mark it as complete"
**Context:** Learner approved Module 1.3 completion and committed changes to Git.
**Output Summary:** AI updated LEARNING.md (marked Module 1.3 `[x]`) and ROADMAP.md (checked off Module 1.3 items), logged edits in LOGS.md, kicked off Module 1.4 (CLI Interface: Interactive Trading Terminal) with Overview, ASCII Terminal Architecture diagram, Concept 14 (Derive-Based CLI Parsers with clap - Bank Teller Counter vs Service Windows analogy), committed Concept 14 word-for-word to EXAMPLES.md, created Exercise 1.4-1 skeleton in EXERCISES.md, and recorded diffs in LOGS.md.
**Outcome / Next Step:** Learner adds `clap` to `Cargo.toml`, creates `src/cli.rs`, and implements `Cli` struct & `Commands` enum.

---

### 2026-08-11 13:55 IST — Prompt #19
**Prompt:** "Done /next"
**Context:** Learner completed `Cli` struct and `Commands` enum in `src/cli.rs` and added `clap` to `Cargo.toml`.
**Output Summary:** AI validated `src/cli.rs`, `Cargo.toml`, and `src/main.rs`, opened gated Solution 1.4-1 in SOLUTIONS.md, moved Exercise 1.4-1 to solved in EXERCISES.md, introduced Concept 15 (CLI Parsing, Command Dispatching & Module System `Cli::parse()`, `match cli.command`, `pub(crate)` - Central Train Station Dispatcher & VIP Security Passes analogy), committed Concept 15 to EXAMPLES.md word-for-word, created Exercise 1.4-2 skeleton in EXERCISES.md, and recorded diffs in LOGS.md.
**Outcome / Next Step:** Learner wires `Cli::parse()` in `src/main.rs` and implements subcommand dispatching via `match`.

---

### 2026-08-11 14:38 IST — Prompt #20
**Prompt:** "The issue i am facing with you is that you hallucinate alot and now instead of giving exercise you are providing exercise and solution at the same time..."
**Context:** Learner pointed out solution code pre-exposure in chat prompts and reported a `clap` parsing error (`unexpected argument '--symbol'`).
**Output Summary:** AI added **Rule 16** (Strict Exercise Isolation & Zero-Solution Pre-exposure) to `RULES.md` and `.agents/workflows/next.md`, logged edits in `LOGS.md`, and explained `clap` positional vs named flag attributes (`#[arg(long)]`).
**Outcome / Next Step:** Learner tests positional arguments (`cargo run -- buy BTC 2 50000`) or adds `#[arg(long)]` to `src/cli.rs`, then completes Exercise 1.4-2.

---

### 2026-08-12 12:46 IST — Prompt #21
**Prompt:** "Done lets move to /next"
**Context:** Learner completed `Cli::parse()` and subcommand `match` dispatching in `src/main.rs`.
**Output Summary:** AI validated `src/main.rs` implementation, opened gated Solution 1.4-2 in SOLUTIONS.md, moved Exercise 1.4-2 to solved in EXERCISES.md, recorded diffs in LOGS.md, verified Module 1.4 completion per Rule 14, and proposed marking Module 1.4 complete in LEARNING.md and ROADMAP.md.
**Outcome / Next Step:** Learner approves LEARNING.md and ROADMAP.md updates for Module 1.4 completion, then moves to Module 1.5 (Order Management Engine - In-Memory State & Collections).

---

### 2026-08-12 12:49 IST — Prompt #22
**Prompt:** "Approved now update the files then we will move forward once you complete"
**Context:** Learner approved Module 1.4 completion and committed changes to Git.
**Output Summary:** AI updated LEARNING.md (marked Module 1.4 `[x]`) and ROADMAP.md (checked off Module 1.4 items), logged edits in LOGS.md, kicked off Module 1.5 (Error Handling: When Trades Fail) with Overview, ASCII Error Propagation diagram, Concept 16 (Custom Error Types & Error Hierarchy with `thiserror` - Security Alarm System vs Emergency Power Cut analogy), committed Concept 16 word-for-word to EXAMPLES.md, created Exercise 1.5-1 skeleton in EXERCISES.md, and recorded diffs in LOGS.md.
**Outcome / Next Step:** Learner adds `thiserror` to `Cargo.toml`, creates `src/errors.rs`, and defines `TradingError` enum.

---

### 2026-08-12 13:48 IST — Prompt #23
**Prompt:** "You are still not following rule 16 like you are giving the exercise and the solution in the same go..."
**Context:** Learner caught pre-exposed solution variants in Exercise 1.5-1 skeleton.
**Output Summary:** AI acknowledged error, updated `EXERCISES.md` to remove pre-exposed variants, logged fix in `LOGS.md`, and provided a strict blanked skeleton per Rule 16.
**Outcome / Next Step:** Learner defines `TradingError` variants in `src/errors.rs`.

---
