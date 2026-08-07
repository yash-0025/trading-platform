# ✍️ EXERCISES.md — Hands-On Exercises (Skeleton + Hints)

> Every hands-on part of a module lands here as an exercise BEFORE any full solution exists.
> You write the code. The AI gives you a skeleton with the taught concept blanked out, plus tiered hints on request.
> Full solutions live in `SOLUTIONS.md` and are gated — see `.agents/workflow/next.md` STEP 3.5 for the exact flow.

**Status legend:** `open` (not attempted yet) · `attempted` (you've had a go, not yet checked) · `solved` (compared against SOLUTIONS.md and understood)

**Rules for this file**
1. Skeleton code only has the CURRENT concept blanked out (`todo!()` / `// TODO(n)`). Everything else — imports, unrelated function bodies, struct defs — is pre-filled so you're never blocked by unrelated syntax.
2. Hints are tiered (conceptual → structural → near-solution) and only given one at a time, on request. "Hints used" gets bumped each time.
3. Nobody (including the AI) opens `SOLUTIONS.md` for an exercise until you've made an actual attempt AND asked to see it.

---

## Entry Format

```
### Exercise <module#>.<n> — <short title>
**Status:** open
**Goal:** one sentence — what this proves you can do.

**Skeleton:**
​```rust
// pre-filled context
fn example() -> Result<(), TradingError> {
    // TODO(1): ...
    todo!()
}
​```

**Constraints:** what NOT to change (signatures, imports, deps).
**Hints used:** 0/3
**My attempt:** *(paste here when ready, even if broken/partial)*
```

---

## Open / In-Progress

### Exercise 1.2-2 — Structs & Newtype Pattern (`Price`, `Quantity`, `Order`)
**Status:** open
**Goal:** Implement the newtype wrapper types `Price` and `Quantity`, and construct the `Order` struct.

**Skeleton:**
```rust
// In src/models.rs:

// TODO(1): Define Price as a newtype tuple struct wrapping i64 (cents), derive Debug, Clone, Copy, PartialEq, Eq
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price(pub i64);

// TODO(2): Define Quantity as a newtype tuple struct wrapping u64 (shares/units), derive Debug, Clone, Copy, PartialEq, Eq
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity(pub u64);

// TODO(3): Define OrderStatus enum with Pending, Filled, Cancelled variants, derive Debug, Clone, PartialEq, Eq
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
}

// TODO(4): Define Order struct with fields: id (u64), asset (String), side (Side), order_type (OrderType), qty (Quantity), status (OrderStatus)
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

**Constraints:** Ensure `Price` and `Quantity` are tuple structs (`pub struct Price(pub i64)`).
**Hints used:** 0/3
**My attempt:** *(paste here when ready, even if broken/partial)*

---

## Solved

### Exercise 1.2-1 — Defining Core Trading Enums (`Side` and `OrderType`)
**Status:** solved
**Goal:** Define the `Side` and `OrderType` enums to represent trading sides and order types as algebraic data types.
**Note:** Solved in `src/models.rs`. Checked against `SOLUTIONS.md` — learned CamelCase convention for variants (`StopLoss`).