# Implement — Dual-DB Cascade (Ordered Checklist)

## Phase 1: Planning Artifacts (this session — completed)
- [x] PRD (`prd.md`) — cascading focus, read-only governance, contracts preserved.
- [x] Design (`design.md`) — architecture, SIMD contracts, TileDB schema, data flow.
- [x] User approval for task creation (`dual-db-cascade`) — granted (`create the task ya`).
- [x] Read-only governance confirmed (`Read-only (recommended)`).

## Phase 2: Design Finalization (before `task.py start`)
- [ ] Verify `design.md` matches user clarification: cascading boundaries + read-only.
- [ ] Confirm `TileDB` 3D array schema attributes (`instructions_emitted`, `cache_misses`, `pass_duration_ms`) — 64-byte aligned.
- [ ] Confirm `LadybugDB` flat index array design (`u32x4` / `u32x8` SIMD-aligned IDs) — branchless variant checks.
- [ ] Confirm forecasting function contract (`std::simd` over `f32x4` aligned slices; exponential smoothing or rolling linear regression; zero `Vec` growth inside loop).
- [ ] Confirm ring-buffer: fixed-size aligned buffer (`[f32; 64]` or `f32x4` array) for background flush; no disk stalls on critical path.

## Phase 3: Implementation (after `task.py start` — deferred until user approves final summary)
- [ ] Create `.trellis/tasks/09-10-dual-db-cascade/` artifacts complete (`prd.md`, `design.md`, `implement.md`).
- [ ] Implement `LadybugDB` interface skeleton (flat index arrays, SIMD-aligned IDs, dependency graph tracking for cascading boundaries).
- [ ] Implement `TileDB` 3D array interface skeleton (schema: `[Module_ID, Pass_ID, Timestamp]`, 64-byte aligned cell blocks, attributes).
- [ ] Implement forecasting function (`f32x4` aligned slices from TileDB memory; branchless arithmetic; zero allocations in loop).
- [ ] Implement asynchronous ring-buffer mechanism (synchronous read at pass start; background flush; fixed-size aligned buffer; no critical-path stalls).
- [ ] Implement integration contract (read-only observation: forecasting reads `PgaAst`/`PgaIr` state; predictions guide externally; no arithmetic path modification; contracts preserved).
- [ ] Verify geometric contracts preserved (`branchless`; dense arrays; `std-lib`; `zero allocations` in arithmetic path; `exact arithmetic`; `multi-backend` independent; `.Logos` dependency preserved; `.gitignore` excludes artifacts).
- [ ] Verify `tests/test_pga.rs` and `tests/test_pga_pipeline.rs` still pass (pipeline unchanged by database layer — read-only observer).
- [ ] Verify `rustc +nightly --edition 2021 --crate-type lib src/lib.rs` passes (`0` errors, `1` harmless warning preserved).

## Phase 4: Verification / Final Review (before archive)
- [ ] `design.md` matches user clarification (cascading + read-only).
- [ ] `implement.md` checklist complete or explicitly deferred (no placeholders).
- [ ] No placeholders / stubs in database layer skeleton (interface contracts defined; forecasting / ring-buffer contracts present).
- [ ] User approves final planning summary (required before `task.py start`).

## Validation Commands
```bash
python3 ./.trellis/scripts/get_context.py --mode phase --step 1.1  # planning artifacts
rustc +nightly --edition 2021 --crate-type lib src/lib.rs          # build (contracts preserved)
```

## Rollback Points
- If forecasting contract breaks `branchless` / `dense arrays`: revert to read-only observation (already confirmed); no arithmetic path changes.
- If `TileDB` 3D array schema conflicts with SIMD alignment: adjust cell attribute layout to 64-byte blocks; forecasting function adapts to new alignment.
- If ring-buffer introduces allocations: enforce fixed-size `const` array (`[f32; 64]`); no `Vec` or `String` in loop.
