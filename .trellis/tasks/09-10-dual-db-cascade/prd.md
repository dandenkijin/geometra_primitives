# PRD — Dual-DB Cascade (LadybugDB + TileDB Predictive Cache)

## Goal
Embed an intelligent predictive compilation cache using a dual-database architecture:
- LadybugDB: in-memory graph (syntax trees, dependency graphs, structural invariants)
- TileDB: 3D sparse array [Module_ID, Pass_ID, Timestamp] for telemetry
The forecasting engine predicts cascading compilation impact across downstream modules (not per-pass selection) to dynamically skip/optimize passes without heap allocations or disk stalls.

## Confirmed Facts (from repository evidence)
- Existing compiler: `geometra_primitives`, `f32x4` SIMD (`core::simd`), branchless arithmetic (`wedge`, `vee`, `intersect`, `motor_chain`), `PgaAst` pipeline (`token` → `parse()` → `PgaIr` dense `Op` layer → `emit` WGGL + CUDA).
- Contracts preserved: `branchless`; dense arrays (`Odd`: 1 reg, `Even`: 2 split); `std-lib`; `zero allocations` in arithmetic path; `exact geometric arithmetic`; `multi-backend` independent (`emit.rs` + `emit_cuda.rs`); `.Logos` unstable interaction documented (`manual .pga` tokenizer accurate; `.Logos` derive preserved); `.gitignore` excludes artifacts.
- No database / cache / forecast layer exists in `.trellis/tasks/` or source.

## In Scope
- Design `LadybugDB` graph schema (flat parallel index arrays, SIMD-aligned IDs for branchless variant checks).
- Design `TileDB` 3D sparse array schema (`Module_ID`, `Pass_ID`, `Timestamp`) + cell attributes (instructions emitted, cache misses, pass duration).
- Vectorized forecasting function over flat memory arrays (`std::simd` / `f32x4` aligned slices) — exponential smoothing or rolling linear regression.
- Zero-allocation prediction loop (raw buffers, no `Vec` growth, no `String` building inside pass loop).
- Asynchronous ring-buffer: synchronous read from DB at module pass start; background flush of telemetry updates to TileDB.
- Integration contract: how database state IDs align with / govern `PgaAst` / `PgaIr` execution (read-only observation preferred per clarification; active governance deferred unless explicitly approved).
- Schema configurations (TileDB array attributes) + Rust code skeleton (database interfaces, forecasting, ring-buffer).
- Preserve geometric contracts (`branchless`, dense arrays, zero allocations in arithmetic path, exact arithmetic, multi-backend independent).

## Out of Scope (explicit)
- Per-function-block optimization pass selection (user selected cascading boundaries over per-pass).
- Actual `TileDB` binary storage / disk I/O (schema design only; no disk-write stalls on critical path — background flush only).
- `.Logos` stability fixes (preserved as reference only).
- Changes to geometric arithmetic contracts (`f32x4` SIMD, `wedge`/`vee`/`intersect`/`motor_chain` exact arithmetic preserved).
- Active governance of `PgaIr` (read-only observation preferred unless user explicitly overrides).
- Full compiler optimization pipeline (`prune_zeros`/`fold_constants`/`merge_terms` complete expression evaluation — deferred to future session).

## Key Decisions (resolved / pending)
- Cascading boundaries (inter-module dependency tracking) — confirmed by user.
- Read-only observation of `PgaAst`/`PgaIr` preferred; active governance deferred — PENDING confirmation from user.
- SIMD-aligned flat arrays (`f32x4` / `u32x8` / `u64x4`) for `LadybugDB`; 64-byte aligned slices for `TileDB` — design requirement.
- Zero allocation in pass loop; `format!` only for emission `Chain`; dense scalar arithmetic only.

## Open Questions (blocking)
1. Read-only vs active governance: should the forecasting engine observe `PgaAst`/`PgaIr` only, or write predictions back into the pipeline (e.g., skip passes based on forecast)? (Default: read-only; user to confirm.)
2. Does the ring-buffer need a fixed-size `const` buffer (e.g., `[f32; 64]` aligned) or dynamic growth allowed in background thread (not critical path)?
3. Should `TileDB` attributes include `Module_ID` → `Pass_ID` dependency mapping directly, or only telemetry metrics?

## Acceptance Criteria
- `.trellis/tasks/09-10-dual-db-cascade/prd.md` complete with cascading boundary focus.
- `.trellis/tasks/09-10-dual-db-cascade/design.md` covers `LadybugDB` graph + `TileDB` array schema + SIMD alignment contracts.
- `.trellis/tasks/09-10-dual-db-cascade/implement.md` has ordered checklist + validation commands.
- `design.md` specifies 64-byte aligned `TileDB` slices (`Simd::from_slice`) + flat index arrays for `LadybugDB`.
- `prd.md` preserves geometric contracts (`branchless`, dense arrays, `std-lib`, zero allocations, exact arithmetic, multi-backend independent).
- User approves final planning summary before `task.py start`.
