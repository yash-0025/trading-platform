# 🏛️ DECISIONS.md — Architecture Decision Records (ADRs)

> Every significant architectural decision made in this project is recorded here with: the decision, why it was chosen, alternatives considered, trade-offs, and future improvements.

---

## ADR-001: Single Binary → Cargo Workspace Evolution Strategy

**Date:** 2026-07-27
**Status:** Accepted
**Module:** Project-wide (Phase 1 → Phase 2 transition)

### Context
We need to decide how to structure the Rust project from Day 1. The trading platform will eventually have multiple components: core trading logic, REST API server, CLI interface, and shared types.

### Decision
Start as a **single binary crate** in Phase 1, then refactor into a **Cargo workspace** with multiple crates in Phase 2, Module 2.13.

### Why This Approach
1. **Learning-first:** A single crate removes workspace complexity while learning Rust fundamentals. You focus on ownership, traits, and error handling — not `[workspace.dependencies]` configuration.
2. **Natural refactoring point:** By Module 2.13, you'll have enough code that the monolith *feels* painful. The refactoring will be motivated by real pain, not artificial structure.
3. **Production pattern:** Many successful Rust projects start as a single crate and split when boundaries become clear. Premature modularization creates wrong boundaries.

### Alternatives Considered
| Alternative | Pros | Cons |
|---|---|---|
| Workspace from Day 1 | Clean separation early | Adds config overhead, premature boundary decisions, distracts from Rust learning |
| Library + binary crate | Separates logic from entry point | Still premature; adds `lib.rs` vs `main.rs` complexity before it's needed |
| Monorepo with separate crates | Each feature is independent | Too much overhead for a learning project; defeats "one evolving project" goal |

### Trade-offs
- **Pro:** Simpler learning curve. One `Cargo.toml`, one `src/` tree, one compilation unit.
- **Con:** Phase 1 code may have tighter coupling than ideal. Module 2.13 refactoring will require moving files and fixing imports.
- **Acceptable because:** The refactoring itself is a valuable learning exercise (real-world Rust projects do this regularly).

### Future Improvements
- Phase 2 Module 2.13: Split into `trading-core`, `trading-api`, `trading-cli`, `trading-common`
- Phase 3: Potentially add `trading-engine`, `trading-risk`, `trading-feed` crates

---

## ADR-002: Integer-Based Price Representation

**Date:** 2026-07-27
**Status:** Proposed (to be implemented in Module 1.2)

### Context
Financial calculations require precise arithmetic. Floating-point numbers (`f64`) have representation errors (e.g., `0.1 + 0.2 != 0.3`), which is unacceptable for a trading platform where every cent matters.

### Decision
Use **integer-based price representation** internally. Prices stored as `i64` in the smallest unit (e.g., cents, or 1/10000th of a dollar for 4-decimal precision). Display conversion handled by a `Price` newtype wrapper.

### Why This Approach
1. **Correctness:** Integer arithmetic is exact. No floating-point surprises.
2. **Performance:** Integer operations are faster than decimal library operations.
3. **Industry standard:** Most exchanges internally represent prices as integers. FIX protocol uses fixed-point.

### Alternatives Considered
| Alternative | Pros | Cons |
|---|---|---|
| `f64` everywhere | Simple, familiar | Precision errors, not suitable for financial calculations |
| `rust_decimal` crate | Arbitrary precision, easy to use | Runtime overhead, external dependency for core type |
| Custom fixed-point type | Full control | More implementation work |

### Trade-offs
- **Pro:** Zero precision errors. Fast. Production-realistic.
- **Con:** Need conversion logic for display. Multiplication requires care (multiply two prices → need to divide by scale factor).
- **Acceptable because:** The `Price` newtype will encapsulate all conversion logic, making it transparent to users.

---

*(New ADRs will be added as architectural decisions are made throughout the project.)*
