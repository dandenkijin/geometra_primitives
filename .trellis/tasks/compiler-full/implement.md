# Compiler Full — Implementation Plan

## Ordered Execution (Full Graded Multivector)
1. Define `PgaIr` grades (`Scalar`, `Vector`, `Bivector`, `Trivector`, `Pseudoscalar`).
2. Implement `prune_zeros`: branchless coefficient check; singularity -> metric zero.
3. Implement `fold_constants`: compile-time geometric evaluation; no runtime branches.
4. Implement `merge_terms`: accumulate like-grade terms; dense FMA arrays.
5. Update emission layer (`emit.rs`): emit `PgaIr` nodes (not just `PgaAst`) to `WGSL` shader strings with unrolled SIMD expressions.
6. Verify pipeline end-to-end: `.pga` syntax -> tokenizer -> AST -> IR -> emission -> geometric result assertion.
7. Multi-backend independence preserved (`PgaAst` + `PgaIr` independent of emission target).

## Validation Commands
- `rustc +nightly --edition 2021 --crate-type lib src/lib.rs`
- `cargo +nightly build`
- `cargo +nightly test` (11 geometric assertions + pipeline tests; 0 executable branches)

## Rollback Points
- If `PgaIr` optimization introduces branch errors: rollback to `PgaAst`-only emission (`emit.rs`).
- If full graded multivector exceeds scope: narrow to `R_3_0_1` core grades (0-4) only.
