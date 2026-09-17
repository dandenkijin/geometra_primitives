# PRD — Fix Sandwich Rotation Formula (Exact Quaternion Sandwich)

## Problem
The current `sandwich()` function at `src/lib.rs:69-84` implements left-multiplication `q * v` instead of the correct quaternion sandwich `q * v * q⁻¹`. This produces mathematically incorrect rotation results.

**Evidence:** Harness output for 180° Z-rotor applied to point `[1,0,0,1]` returns `[1,0,0,1]` (unchanged) instead of expected `[-1,0,0,1]`.

## Goal
Replace the rotation computation with exact quaternion sandwich formula while preserving all existing contracts:
- Branchless arithmetic (0 executable branches in geometric core)
- `f32x4` SIMD dense arrays (Odd/Even split preserved)
- Zero allocations in arithmetic path
- Exact geometric arithmetic (no approximations)
- Multi-backend independence (WGSL + CUDA emission unchanged in structure)

## Scope

### In Scope
- `src/lib.rs:69-84` — `sandwich()` function rotation math
- `src/parser/emit.rs:15-24` — WGSL `sandwich_point` emission
- `src/parser/emit.rs:20-28` — WGSL `sandwich_plane` emission
- `src/parser/emit_cuda.rs:14-22` — CUDA `sandwich_point_cuda` emission
- `src/parser/emit_cuda.rs:24-31` — CUDA `sandwich_plane_cuda` emission
- Verify existing tests pass: `quaternion_rotation_norm_preserved`, `agile_eye_pan_tilt_demo`, `agile_eye_spherical_ik_demo`

### Out of Scope
- Translation coupling terms (`vx + r*vx - ux*pw` etc.) — these are separate from rotation
- `motor_chain` translation coupling (separate issue #2)
- `rotor` function (separate issue #3)
- Database/parser layers (independent)

## Acceptance Criteria

1. **Mathematical correctness:**
   - 180° Z-rotor `Motor{dir:[0,0,0,1], mom:[0,0,0,0]}` applied to point `[1,0,0,1]` yields `[-1,0,0,1]`
   - 90° Z-rotor `Motor{dir:[√0.5,0,0,√0.5], mom:[0,0,0,0]}` applied to `[1,0,0,1]` yields `[0,1,0,1]` (or `[0,-1,0,1]` depending on convention)
   - Pure translation motors (dir=[1,0,0,0]) still produce correct translation

2. **Contract preservation:**
   - `cargo +nightly test` passes (all 11 tests)
   - `cargo +nightly check --all-targets` exits 0
   - Zero executable branches in `sandwich()` and emission functions
   - `f32x4` SIMD usage unchanged (no array fallbacks)
   - No heap allocations in arithmetic path

3. **Emission parity:**
   - WGSL emission (`emit.rs`) and CUDA emission (`emit_cuda.rs`) produce mathematically equivalent shader code
   - Both backends implement the exact same quaternion sandwich formula

4. **No regressions:**
   - `wedge_2d_determinant_equality` still passes
   - `wedge_antisymmetry` still passes
   - All `agile_eye` demo tests pass

## Constraints

- Must use `core::simd::f32x4` native operations (branchless FMA-friendly)
- Must preserve `Odd` (1 register) / `Even` (2 register split) layout
- Translation terms (`vx + r_rot*vx - ux*pw`) remain unchanged — only rotation sub-expression changes
- No `if`/`else`/ternary in arithmetic loop