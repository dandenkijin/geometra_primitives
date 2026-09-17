# Implement — Fix sphere_intersect_sphere

## Ordered Checklist

### Phase 1: Core Implementation

- [ ] **1.1** Edit `src/lib.rs:138-155` — replace `sphere_intersect_sphere` with correct branchless product formula
- [ ] **1.2** Verify `cargo +nightly check` passes

### Phase 2: Emission Updates

- [ ] **2.1** Edit `src/parser/emit.rs` — update WGSL `sphere_intersect_sphere` emission to accept radii
- [ ] **2.2** Edit `src/parser/emit_cuda.rs` — update CUDA `sphere_intersect_sphere_cuda` emission
- [ ] **2.3** Verify `cargo +nightly check --all-targets` passes

### Phase 3: Verification

- [ ] **3.1** Run harness to confirm tangent equal spheres return 0:
  ```bash
  cargo +nightly run --manifest-path /tmp/geometra-audit-crate/Cargo.toml 2>&1 | grep sphere_tangent_equal
  # Expected: sphere_tangent_equal=0
  ```
- [ ] **3.2** Run full test suite: `cargo +nightly test` — all 11 tests must pass
- [ ] **3.3** Verify branchless contract: `grep -A 20 "pub fn sphere_intersect_sphere" src/lib.rs | grep -E "if|else|\?" | grep -v "//"`

### Phase 4: Regression Check

- [ ] **4.1** Run `cargo +nightly test --test test_pga` — geometric tests pass
- [ ] **4.2** Confirm `agile_eye_spherical_ik_demo` still passes (uses sphere_intersect_sphere)

## Validation Commands

```bash
# Type-check
cargo +nightly check

# Harness verification
cargo +nightly run --manifest-path /tmp/geometra-audit-crate/Cargo.toml 2>&1 | grep -E "sphere_tangent_equal|sandwich_180|rotor_pi2|exp_z1"

# Branchless check
grep -A 25 "pub fn sphere_intersect_sphere" src/lib.rs
```

## Rollback Points

| Step | If Fails | Rollback |
|------|----------|----------|
| 1.1 | Compilation error | `git checkout src/lib.rs` |
| 2.1-2.2 | Emission mismatch | `git checkout src/parser/emit.rs src/parser/emit_cuda.rs` |
| 3.1 | Test regression | Revert all changes |

## Files Modified

1. `src/lib.rs` — core function (lines 138-155)
2. `src/parser/emit.rs` — WGSL emission
3. `src/parser/emit_cuda.rs` — CUDA emission

## Estimated Effort

- Core fix: ~15 min
- Emission updates: ~15 min
- Verification: ~15 min
- **Total: ~45 min**