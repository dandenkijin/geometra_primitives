# Geometra_PG Audit Findings — 23 Issues

**Repository:** geometra_pg @ f14996b (main)  
**Audit Date:** 2026-09-10  
**Method:** Static analysis, git history, runtime harness, test inspection  
**Constraint:** Read-only — no source modifications

---

## CRITICAL (6) — Geometric Arithmetic Contract Violations

| # | Issue | File:Line | Evidence |
|---|-------|-----------|----------|
| 1 | `sandwich` rotation formula wrong — implements left-multiply, not quaternion sandwich | `src/lib.rs:69-84` | Harness: 180° Z-rotor leaves point `[1,0,0,1]` unchanged (expected `[-1,0,0,1]`) |
| 2 | `motor_chain` omits dual-quaternion cross-terms (`v1×u2`, dual-scalar) | `src/lib.rs:119-124` | Translation coupling missing `v1 × u2` and `v2 × u1` terms |
| 3 | `rotor` ignores axis x-component, doesn't normalize | `src/lib.rs:197-206` | Harness: axis `[0,0,0,1]` produces `[-4e-8,0,0,1]` with norm 1 but x-component ignored |
| 4 | `exp` is first-order Taylor, not exact geometric exponential | `src/lib.rs:248-258` | Harness: `exp([0,0,0,1],[0,0,0,0])` = `[1,0,0,1]` (should be identity) |
| 5 | `sphere_intersect_sphere` returns 1 for tangent equal spheres | `src/lib.rs:138-155` | Harness: `sphere_tangent_equal=1` (expected 0) |
| 6 | `vee` Plücker formula doesn't match standard meet / emission | `src/lib.rs:47-57` | `emit.rs` and `emit_cuda.rs` implement different formulas |

---

## HIGH (6) — Parser / IR Pipeline Defects

| # | Issue | File:Line | Evidence |
|---|-------|-----------|----------|
| 7 | Parser ignores operands; returns hardcoded constants | `src/parser/mod.rs:19-68` | `"wedge 9 8 7 6 1 2 3 4"` → same AST as `"wedge plane plane"` |
| 8 | `parse_and_lower` discards symbol table & operand tracker | `src/parser/mod.rs:84-89` | Both bound as `_sym_table`, `_op_tracker` (commit f14996b) |
| 9 | `parse_with_symbol_table` loses operands on delegation | `src/parser/symbol_table.rs:149-150` | Delegates to `parse()` which ignores tracked operands |
| 10 | IR uses fixed indices instead of AST-derived values | `src/parser/ir.rs:27-48` | All `WedgePlanes` get `out_idx:2, p_idx:4, q_idx:4` |
| 11 | `PointLineIntersect` token → `IntersectPlanePoint` AST | `src/parser/mod.rs:45-47` | AST declares `PointLineIntersect(Plane,Point)` but parser produces wrong variant |
| 12 | Token coverage gaps — `Sandwich`, `Mul`, `Inverse` unhandled | `src/parser/token.rs:119,130-131` vs `mod.rs:24-69` | Parser falls through to error arm |

---

## MEDIUM (5) — Database Layer Defects

| # | Issue | File:Line | Evidence |
|---|-------|-----------|----------|
| 13 | TileDB missing 3D coordinates (`Module_ID`, `Pass_ID`, `Timestamp`) | `src/database/tiledb.rs:23-61` | `TileCell` has only metrics; `insert_cell` takes no coordinates |
| 14 | Forecasting divides cell count by 4 incorrectly | `src/database/forecast.rs:96` | `max_slices = count/4` but each load = 1 cell, not 4 |
| 15 | LadybugDB has no graph semantics (adjacency, reachability) | `src/database/ladybug.rs:36-72` | Only raw vectors; no `descendants(src)` query |
| 16 | Ring buffer overflow makes 64 entries unreadable | `src/database/ring_buffer.rs:42-58` | Harness: after 65 pushes, read returns index 1 only |
| 17 | `forecast_cascading_impact` ignores LadybugDB graph | `src/database/forecast.rs:90-117` | Signature takes only `TileDBStore` + `alpha` |

---

## LOW (6) — Test Gaps & Code Quality

| # | Issue | File:Line | Evidence |
|---|-------|-----------|----------|
| 18 | Branch in `pseudoscalar_normalize` violates branchless contract | `src/lib.rs:231-232` | `if p == 0.0` branch; README claims "zero executable branches" |
| 19 | Missing `Debug` impl for `PgaAst` | `src/parser/ast.rs:1` | Harness & `test_pga_pipeline.rs:34` can't format AST |
| 20 | Integration demo not compiled/testable | `src/database/integration_demo.rs` | Uses private `_pad`; calls non-existent `simd_f32x4` |
| 21 | 26 compiler warnings (unused vars/imports) | Various | Harness output shows all warnings |
| 22 | `redundancy_metric` emission returns arbitrary `r_sq + 1.0` | `src/parser/emit.rs:61-64` | Should match `lib.rs:95-96` (`||M||²`) |
| 23 | `sphere_intersect_plane` assumes normalized plane | `src/lib.rs:133-136` | Returns `r² - d²` without plane normalization |

---

## Verification Commands

```bash
# Geometry defects (CRITICAL #1-6)
cargo +nightly run --manifest-path /tmp/geometra-audit-crate/Cargo.toml 2>&1 | grep -E "sandwich_180|sphere_tangent_equal|rotor_pi2|exp_z1"

# Parser/IR defects (HIGH #7-12)
cargo +nightly test --test test_pga 2>&1 | grep -E "FAILED|panicked"
cargo +nightly test --test test_pga_pipeline 2>&1 | grep -E "FAILED|panicked"

# Database defects (MEDIUM #13-17)
cargo +nightly test --lib 2>&1 | grep -E "database|forecast|ring"
```