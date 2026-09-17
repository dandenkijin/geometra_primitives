# Implement — Fix Sandwich Rotation Formula

## Ordered Checklist

### Phase 1: Core Implementation

- [ ] **1.1** Edit `src/lib.rs:69-84` — replace rotation math in `sandwich()` with exact quaternion sandwich formula (per design.md)
- [ ] **1.2** Verify `cargo +nightly check` passes (type-check only)

### Phase 2: Emission Updates

- [ ] **2.1** Edit `src/parser/emit.rs:15-24` — update `sandwich_point` WGSL emission with exact rotation formula
- [ ] **2.2** Edit `src/parser/emit.rs:20-28` — update `sandwich_plane` WGSL emission with exact rotation formula
- [ ] **2.3** Edit `src/parser/emit_cuda.rs:14-22` — update `sandwich_point_cuda` CUDA emission
- [ ] **2.4** Edit `src/parser/emit_cuda.rs:24-31` — update `sandwich_plane_cuda` CUDA emission
- [ ] **2.5** Verify `cargo +nightly check --all-targets` passes

### Phase 3: Verification

- [ ] **3.1** Run full test suite: `cargo +nightly test` — all 11 tests must pass
- [ ] **3.2** Run harness to confirm mathematical correctness:
  ```bash
  cargo +nightly run --manifest-path /tmp/geometra-audit-crate/Cargo.toml 2>&1 | grep sandwich_180
  # Expected: sandwich_180=[-1.0, 0.0, 0.0, 1.0]
  ```
- [ ] **3.3** Verify branchless contract: `grep -n "if\|else\|?" src/lib.rs | grep -v "//"` — no branches in `sandwich()`
- [ ] **3.4** Verify SIMD usage: `grep -n "f32x4" src/lib.rs` — `from_array`/`to_array` only

### Phase 4: Regression Check

- [ ] **4.1** Run `cargo +nightly test --test test_pga` — all geometric tests pass
- [ ] **4.2** Run `cargo +nightly test --test test_pga_pipeline` — emission tests pass
- [ ] **4.3** Confirm `wedge_2d_determinant_equality`, `wedge_antisymmetry` unchanged

## Validation Commands

```bash
# Type-check
cargo +nightly check --all-targets

# Full test suite
cargo +nightly test

# Harness verification (external crate)
cargo +nightly run --manifest-path /tmp/geometra-audit-crate/Cargo.toml 2>&1 | grep -E "sandwich_180|sphere_tangent_equal|rotor_pi2|exp_z1"

# Branchless check
grep -A 20 "pub fn sandwich" src/lib.rs | grep -E "if|else|\?" || echo "OK: no branches"

# SIMD usage
grep "f32x4::from_array\|to_array" src/lib.rs
```

## Rollback Points

| Step | If Fails | Rollback |
|------|----------|----------|
| 1.1 | Compilation error | `git checkout src/lib.rs` |
| 2.1-2.4 | Emission test failure | `git checkout src/parser/emit.rs src/parser/emit_cuda.rs` |
| 3.1 | Test regression | Revert all changes; analyze test expectations |

## Files Modified

1. `src/lib.rs` — core geometric function
2. `src/parser/emit.rs` — WGSL emission
3. `src/parser/emit_cuda.rs` — CUDA emission

## Estimated Effort

- Core fix: ~30 min
- Emission updates: ~20 min
- Verification: ~20 min
- **Total: ~1.5 hours**