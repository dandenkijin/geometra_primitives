# Design — SIMD Layout + Algebra Contracts
- Odd f32x4: Planes/Points (dense, no empty grades)
- Even 2×f32x4: Lines/Motors (dir + mom split)
- Unrolled FMA; singularities via metric signature

## Rust / Raw SIMD Contract (updated Phase 2)
- No reefer macro dependency: raw f32x4 SIMD is faster.
- Language: Rust (native SIMD via std::simd / core::simd, or scalar FMA unroll).
- Structure: dense arrays; branchless loops; singularity handled by metric zero.
- Baseline reference: Python `clifford` (structure/reference only, not runtime dependency).
