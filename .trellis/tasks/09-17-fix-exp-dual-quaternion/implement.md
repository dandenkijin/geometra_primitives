# Implement — Fix exp: Exact Dual-Quaternion Exponential

## Ordered Checklist

### Phase 1: Core Implementation

- [ ] **1.1** Edit `src/lib.rs:248-258` — replace `exp()` with exact dual-quaternion exponential (per design.md)
- [ ] **1.2** Verify `cargo +nightly check` passes

### Phase 2: Verification

- [ ] **2.1** Run full test suite: `cargo +nightly test` — all 11 tests must pass
- [ ] **2.2** Run harness to confirm correctness:
  ```bash
  cargo +nightly run --manifest-path /tmp/geometra-audit-crate/Cargo.toml 2>&1 | grep exp_z1
  # Expected: exp of identity motor = identity
  ```
- [ ] **2.3** Test pure rotation motor exponential
- [ ] **2.4** Test pure translation motor exponential
- [ ] **2.5** Verify branchless contract: `grep -A 30 "pub fn exp" src/lib.rs | grep -E "if|else|\?" | grep -v "//"`

### Phase 3: Regression Check

- [ ] **3.1** Run `cargo +nightly test --lib` — library tests pass
- [ ] **3.2** Confirm no other geometric functions affected

## Validation Commands

```bash
# Type-check
cargo +nightly check

# Harness verification
cargo +nightly run --manifest-path /tmp/geometra-audit-crate/Cargo.toml 2>&1 | grep -E "exp|sandwich_180|rotor_pi2|motor_chain"

# Branchless check (arithmetic loop only)
grep -A 35 "pub fn exp" src/lib.rs
```

## Rollback Points

| Step | If Fails | Rollback |
|------|----------|----------|
| 1.1 | Compilation error | `git checkout src/lib.rs` |
| 2.1 | Test regression | Revert `src/lib.rs` |

## Files Modified

1. `src/lib.rs` — `exp()` function only (lines 248-258)

## Estimated Effort

- Core fix: ~20 min
- Verification: ~15 min
- **Total: ~35 min**