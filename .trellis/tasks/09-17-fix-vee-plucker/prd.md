# PRD — Fix vee: Unify with Standard Plücker Meet Formula

## Problem
The `vee()` function at `src/lib.rs:47-57` implements a formula that doesn't match the standard Plücker meet (vee) of two planes, nor does it match the emission formulas in `emit.rs` and `emit_cuda.rs`. There are three different formulas in the codebase:

1. **lib.rs (vee)**: Lines 47-57
2. **emit.rs (vee WGSL)**: Lines 9-14
3. **emit_cuda.rs (vee CUDA)**: Lines 17-22

All three differ, indicating confusion about the correct Plücker coordinates for the meet of two planes (Grade 1) → Line (Grade 2).

## Goal
Unify all three implementations to the standard Plücker meet formula for two planes in PGA R_3_0_1.

## Scope

### In Scope
- `src/lib.rs:47-57` — `vee()` function
- `src/parser/emit.rs:9-14` — WGSL `vee` emission
- `src/parser/emit_cuda.rs:17-22` — CUDA `vee_cuda` emission

### Out of Scope
- `wedge()` (correct, matches standard exterior product)
- Other geometric primitives

## Acceptance Criteria

1. **Mathematical correctness:**
   - All three implementations produce identical Plücker coordinates
   - Formula matches standard meet: for planes `P=(p,w)` and `Q=(q,v)` in homogeneous coordinates, the Plücker line is `L = P ∧ Q` (in dual space) = `[p×q, p·w - q·v]` or similar
   - `vee(vee(P,Q), R)` consistency checks pass

2. **Contract preservation:**
   - `cargo +nightly check` passes
   - `cargo +nightly test` passes
   - Branchless arithmetic
   - `f32x4` SIMD usage unchanged

3. **Emission parity:** All three backends identical

## Constraints

- Branchless arithmetic (no `if`/`else` in computation)
- `f32x4` SIMD native operations
- Zero allocations in arithmetic path
- Must preserve `Line { dir: f32x4, mom: f32x4 }` return structure (dir = 4 components, mom = 4 components, but Plücker has 6 components)