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
- **Matches**: Perfect enum structures, visibility `pub`, correct payload types (`i64`), and derive attributes (`Debug`, `Clone`, `PartialEq`, `Eq`).
- **Difference**: You named the variant `Stoploss` (lowercase `l`), while idiomatic Rust uses CamelCase `StopLoss`. Both compile and are functionally equivalent.

---

### Solution 1.2-2 — Structs & Newtype Pattern (`Price`, `Quantity`, `Order`)

**Reference Implementation:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub id: u64,
    pub asset: String,
    pub side: Side,
    pub order_type: OrderType,
    pub qty: Quantity,
    pub status: OrderStatus,
}
```

**Line-by-Line Breakdown:**
- `pub struct Price(pub i64);` — Newtype pattern tuple struct wrapping `i64`. Derives `Copy` for zero-cost stack copying.
- `pub struct Quantity(pub u64);` — Newtype pattern tuple struct wrapping `u64`.
- `pub struct Order { ... }` — Named-field struct aggregating fields into a cohesive order entity.

**Compared to your attempt:**
- **Matches**: Perfect field layout, newtype definitions, visibility `pub`, and enum variants.
- **Compiler Fix Needed**: You typed `ParitalEq` (typo: `i` before `t`) twice on `Quantity` and `Order`. Changing `ParitalEq` → `PartialEq` will fix compiler errors.

---

*(Additional solutions will be added as exercises get gated open.)*
