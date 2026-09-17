# Implement — Fix Rotor Axis Normalization

## Ordered Checklist

### Phase 1: Core Implementation

- [ ] **1.1** Edit `src/lib.rs:197-206` — replace `rotor()` with normalized axis version (per design.md)
- [ ] **1.2** Verify `cargo +nightly check` passes

### Phase 2: Verification

- [ ] **2.1** Run full test suite: `cargo +nightly test` — all 11 tests must pass
- [ ] **2.2** Run harness to confirm normalization works:
  ```bash
  cargo +nightly run --manifest-path /tmp/geometra-audit-crate/Cargo.toml 2>&1 | grep rotor
  ```
- [ ] **2.3** Verify branchless contract: `grep -A 15 "pub fn rotor" src/lib.rs | grep -E "if|else|\?" || echo "OK: no branches in arithmetic"`
- [ ] **2.4** Test non-unit axis:
  ```bash
  # Test that [0, 2, 0, 0] gives same result as [0, 1, 0, 0]
  ```

### Phase 3: Regression Check

- [ ] **3.1** Run `cargo +nightly test --test test_pga` — geometric tests pass
- [ ] **3.2** Run `cargo +nightly test --test test_pga_pipeline` — emission tests pass
- [ ] **3.3** Confirm `agile_eye_pan_tilt_demo`, `agile_eye_spherical_ik_demo` pass

## Validation Commands

```bash
# Type-check
cargo +nightly check

# Full test suite
cargo +nightly test

# Harness verification
cargo +nightly run --manifest-path /tmp/geometra-audit-crate/Cargo.toml 2>&1 | grep rotor

# Branchless check (arithmetic loop only)
grep -A 20 "pub fn rotor" src/lib.rs
```

## Rollback Points

| Step | If Fails | Rollback |
|------|----------|----------|
| 1.1 | Compilation error | `git checkout src/lib.rs` |
| 2.1 | Test regression | Revert `src/lib.rs` |

## Files Modified

1. `src/lib.rs` — `rotor()` function only (lines 197-206)

## Estimated Effort

- Core fix: ~15 min
- Verification: ~15 min
- **Total: ~30 min**