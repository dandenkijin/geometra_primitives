# PRD — Fix sphere_intersect_sphere: Correct Intersection Metric

## Problem
The `sphere_intersect_sphere()` function at `src/lib.rs:138-155` uses an incorrect metric:
```rust
let sum_r_sq = (r1 + r2) * (r1 + r2);
let diff_r_sq = r1 * r2;  // BUG: should be (r1 - r2)^2
sum_r_sq - dist_sq + diff_r_sq
```

**Evidence:** Harness output `sphere_tangent_equal=1` for centers `[0,0,0,1]`, `[2,0,0,1]` with radii `1.0, 1.0`. These spheres are tangent (distance = 2 = r1 + r2), so intersection should be 0 (touching at one point), but returns 1.

## Goal
Replace with correct sphere-sphere intersection metric that:
- Returns 0 for externally tangent spheres (distance = r1 + r2)
- Returns 0 for internally tangent spheres (distance = |r1 - r2|)
- Returns positive for intersecting spheres
- Returns negative for separated spheres
- Handles signed distance appropriately for PGA

## Scope

### In Scope
- `src/lib.rs:138-155` — `sphere_intersect_sphere()` function
- `src/parser/emit.rs` — WGSL emission if it differs
- `src/parser/emit_cuda.rs` — CUDA emission if it differs

### Out of Scope
- `sphere_intersect_plane` (separate issue #23)
- Other geometric primitives

## Acceptance Criteria

1. **Mathematical correctness:**
   - Tangent equal spheres (distance = r1 + r2) → 0
   - Concentric spheres (distance = 0, r1 ≠ r2) → negative (no intersection)
   - Intersecting spheres → positive
   - One sphere inside another without touching → negative

2. **Contract preservation:**
   - `cargo +nightly check` passes
   - `cargo +nightly test` passes
   - Branchless arithmetic
   - `f32x4` SIMD usage unchanged

3. **Emission parity:** WGSL and CUDA emissions updated identically

## Constraints

- Branchless arithmetic (no `if`/`else` in computation)
- `f32x4` SIMD native operations
- Zero allocations in arithmetic path