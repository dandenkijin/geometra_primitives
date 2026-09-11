# Design — Dual-DB Cascade (Cascading Boundaries, Read-Only)

## Architecture (confirmed from user: cascading + read-only)
- LadybugDB (in-memory graph): flat parallel index arrays (`u32x4` / `u32x8` SIMD-aligned IDs) tracking syntax trees (`PgaAst` nodes) and inter-module dependency graphs. No pointer-heavy nodes; raw IDs load directly into vector registers.
- TileDB (3D sparse array): `[Module_ID, Pass_ID, Timestamp]` with 64-byte aligned slices (`Simd::from_slice`). Cell attributes: `instructions_emitted`, `cache_misses`, `pass_duration`.
- Integration: forecasting reads DB synchronously at module pass start; predictions do NOT write back to `PgaAst`/`PgaIr` (read-only per user confirmation). Geometric contracts (`branchless`, dense arrays, `std-lib`, `zero allocations`, exact arithmetic, multi-backend) fully preserved.

## SIMD / Alignment Contracts (from P1 specs)
- `f32x4` dense arrays (`core::simd` nightly `portable_simd`); `Odd` (`Plane`/`Point`) 1 reg; `Even` (`Line`/`Motor`) 2 split (`dir` + `mom`).
- `LadybugDB`: flat index arrays (`u32x8` / `u64x4`) for branchless variant checks (SIMD-aligned IDs load directly into vector registers).
- `TileDB`: 64-byte aligned slices for zero-copy vector loads (`Simd::from_slice` into native LLVM registers).
- Zero allocations in arithmetic / prediction loop (`format!` only for emission `Chain`; dense scalar arithmetic only; no `Vec` growth inside loop).

## Data Flow (read-only observation)
1. Module pass starts → synchronous read from `LadybugDB` (dependency graph IDs) + `TileDB` (telemetry for [Module_ID, Pass_ID, Timestamp]).
2. Vectorized forecasting over flat TileDB memory arrays (`f32x4` aligned slices) — exponential smoothing or rolling linear regression — predicts cascading impact thresholds.
3. Predictions guide scheduling / optimization recommendations externally; do NOT alter `PgaAst` / `PgaIr` execution path.
4. Background thread: asynchronous flush of new telemetry metrics to TileDB (ring-buffer mechanism, fixed-size aligned buffer, no disk stalls on critical path).

## Schema (TileDB array attributes, 3D sparse)
Attributes per cell: `{instructions_emitted: f32, cache_misses: u32, pass_duration_ms: f32}` — 64-byte aligned block to match `f32x4` / `Simd` loads.

## Compatibility / Migration
- No changes to geometric arithmetic contracts (`wedge`, `vee`, `intersect`, `sandwich`, `motor_chain`, primitives). `.Logos` dependency preserved as reference.
- Multi-backend emission (`emit.rs` WGGL + `emit_cuda.rs` CUDA) independent; database layer does not modify emission contracts.
- `PgaIr` dense `Op` layer (`lower_ast_to_ir`) preserved; forecasting is read-only observer.

## Trade-offs / Risks
- Read-only (chosen): lower risk, faster verification, contracts fully preserved. Deferred: active governance (requires contract extension, not approved).
- Cascading focus (chosen): broader scope (inter-module graphs) vs per-pass selection (narrower, faster). User chose broader scope.
- Zero-allocation loop: requires fixed-size aligned ring buffer (`[f32; 64]` or similar) in background thread; dynamic growth deferred to non-critical path.
