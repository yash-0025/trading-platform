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
