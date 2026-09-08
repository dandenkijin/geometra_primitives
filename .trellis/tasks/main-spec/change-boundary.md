Boundary (updated for Rust code):
- Change: add src/lib.rs with branchless SIMD geometric product (f32x4, unrolled FMA, singularity via metric zero).
- Design: updated design.md (no reefer, raw SIMD, Rust contract, clifford baseline reference).
- Not doing: full compiler backend, no reefer dependency, no integration tests yet.
- Refactor risk: none — new file only; no existing source modified.
