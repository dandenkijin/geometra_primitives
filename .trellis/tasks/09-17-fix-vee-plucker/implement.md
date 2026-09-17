# Implement — Fix vee: Unify with Standard Plücker Meet Formula

## Ordered Checklist

### Phase 1: Core Implementation

- [ ] **1.1** Edit `src/lib.rs:47-57` — fix `vee()` signature to `Point, Point` and implement correct Plücker formula
- [ ] **1.2** Verify `cargo +nightly check` passes

### Phase 2: Emission Updates

- [ ] **2.1** Edit `src/parser/emit.rs:9-14` — update WGSL `vee` emission to match standard formula
- [ ] **2.2** Edit `src/parser/emit_cuda.rs:17-22` — update CUDA `vee_cuda` emission to match
- [ ] **2.3** Verify `cargo +nightly check --all-targets` passes

### Phase 3: Verification

- [ ] **3.1** Run harness to confirm Vee produces finite results for test inputs
- [ ] **3.2** Run full test suite: `cargo +nightly test` — all 11 tests must pass
- [ ] **3.3** Verify branchless contract: `grep -A 20 "pub fn vee" src/lib.rs | grep -E "if|else|\?" | grep -v "//"`

### Phase 4: Regression Check

- [ ] **4.1** Run `cargo +nightly test --test test_pga` — geometric tests pass
- [ ] **4.2** Run `cargo +nightly test --test test_pga_pipeline` — emission tests pass
- [ ] **4.3** Confirm `wedge_vee_no_branch` test still passes

## Validation Commands

```bash
# Type-check
cargo +nightly check --all-targets

# Full test suite
cargo +nightly test

# Harness verification
cargo +nightly run --manifest-path /tmp/geometra-audit-crate/Cargo.toml 2>&1 | grep -E "vee|wedge"

# Branchless check
grep -A 25 "pub fn vee" src/lib.rs | grep -E "if|else|\?" | grep -v "//" || echo "OK: no branches"
```

## Rollback Points

| Step | If Fails | Rollback |
|------|----------|----------|
| 1.1 | Compilation error | `git checkout src/lib.rs` |
| 2.1-2.2 | Emission test failure | `git checkout src/parser/emit.rs src/parser/emit_cuda.rs` |
| 3.1 | Test regression | Revert all changes; analyze test expectations |

## Files Modified

1. `src/lib.rs` — core `vee()` function (lines 47-57)
2. `src/parser/emit.rs` — WGSL emission
3. `src/parser/emit_cuda.rs` — CUDA emission

## Estimated Effort

- Core fix: ~15 min
- Emission updates: ~20 min
- Verification: ~15 min
- **Total: ~50 min**