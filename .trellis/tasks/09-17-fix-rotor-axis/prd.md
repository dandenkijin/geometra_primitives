# PRD — Fix Rotor Axis Normalization

## Problem
The `rotor()` function at `src/lib.rs:197-206` creates a pure rotation motor from an axis and angle, but has two defects:
1. **Ignores x-component**: Uses `dir_u.to_array()` but only reads indices 1-3 (`d[1]*sin`, `d[2]*sin`, `d[3]*sin`). The x-component `d[0]` is ignored.
2. **No normalization**: The axis vector `(d[1], d[2], d[3])` is not normalized, so non-unit axes produce incorrect rotation magnitude.

**Evidence:** Harness output for `rotor([0,0,0,1], π/2)` produces `[-4.37e-8, 0.0, 0.0, 1.0]` with norm_sq=1. While this specific case works (axis already unit), arbitrary axes would fail.

## Goal
Fix `rotor()` to:
- Normalize the axis vector `(d[1], d[2], d[3])` before computing sin/cos components
- Use all components correctly (though `d[0]` is typically 0 for a direction vector in this representation)
- Preserve all existing contracts

## Scope

### In Scope
- `src/lib.rs:197-206` — `rotor()` function
- Verify tests still pass: `agile_eye_pan_tilt_demo`, `agile_eye_spherical_ik_demo`

### Out of Scope
- `sandwich()` (issue #1, fixed)
- `motor_chain()` (issue #2, fixed)
- `exp()` (issue #4)
- Emission backends (they call `rotor()` so will automatically use fixed version)

## Acceptance Criteria

1. **Mathematical correctness:**
   - `rotor([0, 2, 0, 0], π/2)` produces same result as `rotor([0, 1, 0, 0], π/2)` (normalized)
   - `rotor([0, 1, 1, 0], π/2)` produces correct 90° rotation about normalized axis
   - Unit axis vectors produce unit quaternion (r² + ux² + uy² + uz² = 1)

2. **Contract preservation:**
   - `cargo +nightly check` passes
   - `cargo +nightly test` passes (all 11 tests)
   - Zero executable branches in `rotor()` (branchless arithmetic)
   - `f32x4` SIMD usage unchanged
   - No heap allocations in arithmetic path

3. **No regressions:**
   - Existing demos using `rotor()` still work correctly

## Constraints

- Must use `core::simd::f32x4` native operations
- Must preserve `Motor { dir: f32x4, mom: f32x4 }` return type with mom = [0,0,0,0]
- No `if`/`else`/ternary in arithmetic — use branchless normalization: `scale = 1.0 / max(norm, ε)`
- Handle zero-norm axis gracefully (return identity motor)