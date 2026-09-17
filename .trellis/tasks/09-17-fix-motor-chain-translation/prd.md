# PRD — Fix motor_chain Translation Coupling (Exact Dual-Quaternion Multiplication)

## Problem
The current `motor_chain()` function at `src/lib.rs:102-132` implements quaternion rotation composition correctly but **omits dual-quaternion cross-terms** in the translation coupling. Specifically, the translation part uses:
```
v = v1 * r2 + r1 * v2 + u1 × v2
```
Missing terms: `v1 × u2` and dual-scalar contributions required by full dual-quaternion multiplication `M = M1 * M2`.

**Evidence:** The current code (lines 119-124) lacks the `v1 × u2` cross-term and dual-scalar (`p`) coupling terms.

## Goal
Replace the translation coupling with exact dual-quaternion multiplication formula while preserving all existing contracts:
- Branchless arithmetic (0 executable branches in geometric core)
- `f32x4` SIMD dense arrays (Odd/Even split preserved)
- Zero allocations in arithmetic path
- Exact geometric arithmetic (no approximations)
- Multi-backend independence (WGSL + CUDA emission)

## Scope

### In Scope
- `src/lib.rs:102-132` — `motor_chain()` function translation coupling (lines 119-126)
- `src/parser/emit.rs:26-32` — WGSL `motor_chain_unrolled` emission
- `src/parser/emit.rs:55-60` — WGSL `motor_chain` emission
- `src/parser/emit_cuda.rs:26-33` — CUDA `motor_chain_unrolled_cuda` emission
- `src/parser/emit_cuda.rs:48-54` — CUDA `motor_chain_unrolled_cuda` emission
- Verify existing tests pass: `motor_chain_no_panic`, `agile_eye_spherical_ik_demo`

### Out of Scope
- Rotation composition (already correct: `r_new = r1*r2 - u1·u2`, etc.)
- `sandwich()` function (separate issue #1, already fixed)
- `rotor` function (separate issue #3)
- Database/parser layers (independent)

## Acceptance Criteria

1. **Mathematical correctness:**
   - Sequential motor composition matches dual-quaternion algebra: `M = M1 * M2`
   - Translation coupling includes all cross-terms: `v = v1*r2 + r1*v2 + u1×v2 + v1×u2`
   - Dual-scalar part: `p = p1 + p2 + r1*p2 - r2*p1` (already present in current code)

2. **Contract preservation:**
   - `cargo +nightly test` passes (all 11 tests)
   - `cargo +nightly check --all-targets` exits 0
   - Zero executable branches in `motor_chain()` and emission functions
   - `f32x4` SIMD usage unchanged
   - No heap allocations in arithmetic path

3. **Emission parity:**
   - WGSL emission (`emit.rs`) and CUDA emission (`emit_cuda.rs`) produce mathematically equivalent shader code
   - Both backends implement the exact same dual-quaternion multiplication

4. **No regressions:**
   - `motor_chain_no_panic` still passes
   - `agile_eye_spherical_ik_demo` still passes
   - Rotation composition unchanged (already correct)

## Constraints

- Must use `core::simd::f32x4` native operations (branchless FMA-friendly)
- Must preserve `Odd` (1 register) / `Even` (2 register split) layout
- Rotation composition lines 110-117 remain unchanged
- No `if`/`else`/ternary in arithmetic loop