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

### Solution 1.2-3 — Impl Blocks, Constructors (`Self::new()`), and Method Mutability (`&mut self`)

**Reference Implementation:**
```rust
impl Order {
    pub fn new(id: u64, asset: String, side: Side, order_type: OrderType, qty: Quantity) -> Self {
        Order {
            id,
            asset,
            side,
            order_type,
            qty,
            status: OrderStatus::Pending,
        }
    }

    pub fn fill(&mut self) {
        self.status = OrderStatus::Filled;
    }

    pub fn is_pending(&self) -> bool {
        self.status == OrderStatus::Pending
    }
}
```

**Line-by-Line Breakdown:**
- `impl Order { ... }` — Implementation block attaching functions and methods to the `Order` struct.
- `pub fn new(...) -> Self` — Associated function constructor. `Self` is a type alias for `Order`. Field init shorthand (`id`, `asset`, etc.) maps parameter names directly to struct fields. Sets default `status: OrderStatus::Pending`.
- `pub fn fill(&mut self)` — Method taking an exclusive mutable borrow (`&mut self`) to mutate `self.status = OrderStatus::Filled`.
- `pub fn is_pending(&self) -> bool` — Method taking an immutable shared borrow (`&self`) to inspect status without mutating memory.

**Compared to your attempt:**
- **Exact Match!**: Your implementation used field initialization shorthand, correct `&mut self` mutation, and `&self` equality comparison. Flawless!

---

*(Additional solutions will be added as exercises get gated open.)*

