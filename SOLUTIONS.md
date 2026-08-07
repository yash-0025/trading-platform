# 🔐 SOLUTIONS.md — Gated Reference Solutions

> **Read this only after you've attempted the matching exercise in `EXERCISES.md` and explicitly asked to see the solution.**
> Reading ahead defeats the point — the exercise is where the actual learning happens, not the solution.
>
> The AI will not open or paste from this file until both gate conditions in `.agents/workflow/next.md` STEP 3.5-C are met:
> 1. You've pasted/described a real attempt (even a broken or partial one).
> 2. You've explicitly asked to see the solution.

**Entry numbering matches `EXERCISES.md` exactly** — Exercise 1.1-1 → Solution 1.1-1, etc.

---

## Entry Format

```
### Solution <module#>.<n> — <short title>

**Reference implementation:**
​```rust
fn example() -> Result<(), TradingError> {
    if quantity == 0 {
        return Err(TradingError::InvalidQuantity);
    }
    // ...
    Ok(())
}
​```

**Line-by-line:**
- `if quantity == 0 { ... }` — why this check, why here, what it prevents.
- `return Err(TradingError::InvalidQuantity)` — why this error variant, how `?` would propagate it upstream.
- ...

**Compared to your attempt:**
- What matched.
- What differed, and why the reference approach was chosen (performance, idiom, correctness).
- Any misconception the diff reveals, so it doesn't repeat next exercise.
```

---

### Solution 1.2-1 — Core Trading Enums (`Side` and `OrderType`)

**Reference Implementation:**
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Market,
    Limit { price: i64 },
    StopLoss { trigger_price: i64 },
}
```

**Line-by-Line Breakdown:**
- `#[derive(Debug, Clone, PartialEq, Eq)]` — Auto-implements formatting (`Debug`), duplication (`Clone`), and value equality (`PartialEq`, `Eq`) for `Side`.
- `pub enum Side { Buy, Sell }` — Public enum with two unit variants representing trading sides.
- `#[derive(Debug, Clone, PartialEq)]` — `OrderType` contains `i64` payloads, so it derives `PartialEq`.
- `Limit { price: i64 }` — Struct variant holding fixed-point price in cents.
- `StopLoss { trigger_price: i64 }` — Struct variant holding trigger price in cents.

**Compared to your attempt:**
- **Matches**: Perfect enum structures, visibility `pub`, correct payload types (`i64`), and derive attributes (`Debug`, `Clone`, `PartialEq`, `Eq`).
- **Difference**: You named the variant `Stoploss` (lowercase `l`), while idiomatic Rust uses CamelCase `StopLoss`. Both compile and are functionally equivalent.

---

*(Additional solutions will be added as exercises get gated open.)*