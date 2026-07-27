# ❓ QUESTIONS.md — Questions, Answers & Interview Prep

> Questions asked during the learning journey, answers provided, topics to revisit, and interview-style questions for self-testing.

---

## Format
```
### Q[N]: <question>
**Asked:** <date> · **Module:** <module reference>
**Answer:** <detailed answer>
**Revisit?:** Yes/No · **Confidence:** Low/Medium/High
```

---

## Trading Domain Questions (Pre-Module)

These questions help build intuition about the trading domain BEFORE writing code.

### Q1: What is an order book, and why does it exist?
**Asked:** 2026-07-27 · **Module:** Pre-project
**Answer:** An order book is a real-time, organized list of all outstanding buy and sell orders for a specific asset on an exchange. It exists because buyers and sellers need a transparent, fair mechanism to discover prices and find counterparties. Without an order book, you'd have to individually negotiate with every potential buyer/seller (like a flea market). The order book aggregates all interest at each price level, creating a "market" that's efficient and fair.
**Revisit?:** Yes (deep dive in Module 3.1) · **Confidence:** Medium

### Q2: What's the difference between a market order and a limit order?
**Asked:** 2026-07-27 · **Module:** Pre-project
**Answer:**
- **Market order:** "Buy 100 shares of AAPL at whatever the current best price is." Guaranteed execution, not guaranteed price. You might pay more than expected if the order book is thin (slippage).
- **Limit order:** "Buy 100 shares of AAPL, but only at $150 or less." Guaranteed price (or better), but NOT guaranteed execution. If nobody is selling at $150, your order sits in the book waiting.
**Revisit?:** Yes (Module 1.2 & 3.2) · **Confidence:** Medium

### Q3: Why do trading platforms need to be so fast?
**Asked:** 2026-07-27 · **Module:** Pre-project
**Answer:** Speed matters because of **price competition**. If two traders both want to buy AAPL at $150, the one whose order arrives at the exchange first gets filled first (price-time priority). In HFT, firms compete on microseconds. Even for retail platforms like Zerodha/Robinhood, latency affects user experience (nobody wants to click "Buy" and wait 5 seconds to know if it worked). This is why we use Rust instead of Python — Rust gives us predictable, low-latency performance without garbage collection pauses.
**Revisit?:** Yes (Phase 3) · **Confidence:** Medium

---

## Rust Technical Questions (Interview Style)

These are questions you should be able to answer confidently by the end of each phase.

### Phase 1 — After Completion, You Should Answer:

- [ ] What is ownership and why does Rust have it? What does it replace?
- [ ] Explain the difference between `String` and `&str`. When do you use each?
- [ ] What are the borrowing rules? Why can't you have `&T` and `&mut T` at the same time?
- [ ] What's the difference between `Clone` and `Copy`? When is a type `Copy`?
- [ ] Explain `Result<T, E>` vs `Option<T>`. When do you use each?
- [ ] What does the `?` operator do? How does it desugar?
- [ ] What's the difference between `thiserror` and `anyhow`? When do you use each?
- [ ] Explain the difference between a struct and an enum in Rust. How are they different from other languages?
- [ ] What does `match` guarantee that `if/else` doesn't?
- [ ] What are iterators in Rust? Why are they "zero-cost abstractions"?
- [ ] Explain `Box<T>`, `Rc<T>`, and `RefCell<T>`. When do you use each?
- [ ] What's the newtype pattern? Why is it useful?
- [ ] How does Rust's module system work? What's `pub(crate)`?

### Phase 2 — After Completion, You Should Answer:

- [ ] What's the difference between `async`/`.await` in Rust vs JavaScript?
- [ ] Why does Rust need an external async runtime (Tokio)?
- [ ] What does `Send` mean? What does `Sync` mean?
- [ ] When do you use `Arc<Mutex<T>>` vs `Arc<RwLock<T>>`?
- [ ] Explain `tokio::sync::Mutex` vs `std::sync::Mutex` for async code.
- [ ] What is `tower` middleware and why does Axum use it?
- [ ] What's compile-time checked SQL in `sqlx`? Why does it matter?
- [ ] How do Cargo workspaces work? When should you split into multiple crates?

### Phase 3 — After Completion, You Should Answer:

- [ ] How does an order book work internally? What data structure would you use?
- [ ] What's a matching engine? Why is it typically single-threaded?
- [ ] When is `unsafe` justified? What invariants must you uphold?
- [ ] What's zero-copy parsing? Why does it matter for performance?
- [ ] Explain atomic ordering: Relaxed, Acquire, Release, SeqCst.
- [ ] What's the difference between lock-free and wait-free?
- [ ] When would you use a custom allocator vs the global allocator?

---

*(New questions and answers will be added throughout the learning journey.)*
