# PRD — Fix exp: Exact Dual-Quaternion Exponential

## Problem
The `exp()` function at `src/lib.rs:248-258` implements only a first-order Taylor approximation:
```rust
Motor {
    dir: f32x4::from_array([1.0_f32 + d[0], d[1], d[2], d[3]]),
    mom: f32x4::from_array([mo[0], mo[1], mo[2], mo[3]]),
}
```
This is mathematically incorrect for dual-quaternion exponential. For a dual quaternion `M = q + ε * q'` (where `q = (r, u)` is quaternion part, `q' = (0, v) + p*ε` is dual part), the exact exponential is:
```
exp(M) = exp(q) + ε * (exp(q) * q')
```
Where `exp(q)` for quaternion `q = (r, u)` with `|u| = θ`:
- If `θ > 0`: `exp(q) = (cosh(r) * cos(θ) + sinh(r) * sin(θ)/θ * u, ...)` — but for pure rotation quaternions `r = cos(θ/2)`, the formula simplifies
- For pure rotation motor (no translation): `exp(q) = (cos(θ/2), sin(θ/2) * û)`

**Evidence:** Harness output `exp_z1=[1.0, 0.0, 0.0, 1.0]` for motor `dir=[0,0,0,1], mom=[0,0,0,0]`. This is wrong — identity motor should produce identity.

## Goal
Replace first-order approximation with exact dual-quaternion exponential formula while preserving all contracts.

## Scope

### In Scope
- `src/lib.rs:248-258` — `exp()` function
- Verify tests still pass

### Out of Scope
- `sandwich()`, `motor_chain()`, `rotor()` (issues #1-3, fixed)
- Emission backends (they don't emit `exp()` currently)

## Acceptance Criteria

1. **Mathematical correctness:**
   - `exp(identity_motor)` returns identity motor
   - `exp(pure_rotation_motor)` returns correct rotation motor
   - `exp(pure_translation_motor)` returns correct translation motor
   - General motor: `exp(q + ε*q') = exp(q) + ε*exp(q)*q'`

2. **Contract preservation:**
   - `cargo +nightly check` passes
   - `cargo +nightly test` passes
   - Zero allocations in arithmetic path
   - Branchless arithmetic (no `if`/`else` in main computation)

3. **No regressions**

## Constraints

- Must use `core::simd::f32x4` native operations
- Must preserve `Motor { dir: f32x4, mom: f32x4 }` return type
- No `if`/`else`/ternary in arithmetic loop (boundary checks OK)