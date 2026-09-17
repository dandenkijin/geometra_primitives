# Implement — Fix motor_chain Translation Coupling

## Ordered Checklist

### Phase 1: Core Implementation

- [ ] **1.1** Edit `src/lib.rs:119-126` — replace translation coupling in `motor_chain()` with full dual-quaternion formula (per design.md)
- [ ] **1.2** Verify `cargo +nightly check` passes (type-check only)

### Phase 2: Emission Updates

- [ ] **2.1** Edit `src/parser/emit.rs:26-32` — update `motor_chain_unrolled` WGSL emission
- [ ] **2.2** Edit `src/parser/emit.rs:55-60` — update `motor_chain` WGSL emission
- [ ] **2.3** Edit `src/parser/emit_cuda.rs:26-33` — update `motor_chain_unrolled_cuda` CUDA emission
- [ ] **2.4** Edit `src/parser/emit_cuda.rs:48-54` — update `motor_chain_unrolled_cuda` CUDA emission
- [ ] **2.5** Verify `cargo +nightly check --all-targets` passes

### Phase 3: Verification

- [ ] **3.1** Run full test suite: `cargo +nightly test` — all 11 tests must pass
- [ ] **3.2** Run harness to confirm no regression in existing behavior
- [ ] **3.3** Verify branchless contract: `grep -A 30 "pub fn motor_chain" src/lib.rs | grep -E "if|else|\?" || echo "OK: no branches"`
- [ ] **3.4** Verify SIMD usage unchanged

### Phase 4: Regression Check

- [ ] **4.1** Run `cargo +nightly test --test test_pga` — geometric tests pass
- [ ] **4.2** Run `cargo +nightly test --test test_pga_pipeline` — emission tests pass
- [ ] **4.3** Confirm `motor_chain_no_panic`, `agile_eye_spherical_ik_demo` pass

## Validation Commands

```bash
# Type-check
cargo +nightly check --all-targets

# Full test suite
cargo +nightly test

# Harness verification
cargo +nightly run --manifest-path /tmp/geometra-audit-crate/Cargo.toml 2>&1 | grep -E "motor_chain|sandwich"

# Branchless check
grep -A 30 "pub fn motor_chain" src/lib.rs | grep -E "if|else|\?" || echo "OK: no branches"

# SIMD usage
grep "f32x4::from_array\|to_array" src/lib.rs | head -20
```

## Rollback Points

| Step | If Fails | Rollback |
|------|----------|----------|
| 1.1 | Compilation error | `git checkout src/lib.rs` |
| 2.1-2.4 | Emission test failure | `git checkout src/parser/emit.rs src/parser/emit_cuda.rs` |
| 3.1 | Test regression | Revert all changes; analyze test expectations |

## Files Modified

1. `src/lib.rs` — core `motor_chain()` function (lines 119-126)
2. `src/parser/emit.rs` — WGSL emission (two functions)
3. `src/parser/emit_cuda.rs` — CUDA emission (two functions)

## Estimated Effort

- Core fix: ~30 min
- Emission updates: ~20 min
- Verification: ~20 min
- **Total: ~1.5 hours**