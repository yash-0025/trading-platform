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

### Exercise 1.3-2 — File Parsing & Layered Fallback (`config.toml`, `std::fs::read_to_string`)
**Status:** open
**Goal:** Implement `Config::from_file_or_env(path: &str)` to attempt reading `config.toml` before falling back to env vars / defaults.

**Skeleton:**
```rust
// In src/config.rs:

impl Config {
    // TODO(1): Attempt reading config file at `path` using std::fs::read_to_string(path).
    // TODO(2): If file read fails (Err), return Config::from_env_or_default().
    // TODO(3): For now, if file read succeeds, return Config::from_env_or_default() (we will add toml parsing next).
    pub fn from_file_or_env(path: &str) -> Self {
        todo!()
    }
}
```

**Constraints:** Do not panic on file missing; fall back safely to `Config::from_env_or_default()`.
**Hints used:** 0/3
**My attempt:** *(paste here when ready, even if broken/partial)*

---

## Solved

### Exercise 1.3-1 — Config Struct & Env Fallback (`Option<T>` & `unwrap_or_else`)
**Status:** solved
**Goal:** Build a `Config` struct representing exchange settings and implement `from_env_or_default()` using `Option` combinators.
**Note:** Solved in `src/config.rs`. Checked against `SOLUTIONS.md` — learned `unwrap_or_else` vs `match` syntax and placing method bodies inside functions.


### Exercise 1.2-3 — Impl Blocks, Constructors (`Self::new()`), and Method Mutability (`&mut self`)
**Status:** solved
**Goal:** Implement the `new()` constructor and `fill()` method on `Order`.
**Note:** Solved in `src/models.rs`. Checked against `SOLUTIONS.md` — exact match on struct field init shorthand, `&mut self` mutation, and `&self` status checking.

### Exercise 1.2-2 — Structs & Newtype Pattern (`Price`, `Quantity`, `Order`)
**Status:** solved
**Goal:** Implement the newtype wrapper types `Price` and `Quantity`, and construct the `Order` struct.
**Note:** Solved in `src/models.rs`. Checked against `SOLUTIONS.md` — fixed `ParitalEq` typo to `PartialEq`.

### Exercise 1.2-1 — Defining Core Trading Enums (`Side` and `OrderType`)
**Status:** solved
**Goal:** Define the `Side` and `OrderType` enums to represent trading sides and order types as algebraic data types.
**Note:** Solved in `src/models.rs`. Checked against `SOLUTIONS.md` — learned CamelCase convention for variants (`StopLoss`).

