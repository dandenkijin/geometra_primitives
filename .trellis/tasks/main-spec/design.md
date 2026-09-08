# Design — SIMD Layout + Algebra Contracts
- Odd f32x4: Planes/Points (dense, no empty grades)
- Even 2×f32x4: Lines/Motors (dir + mom split)
- Unrolled FMA; singularities via metric signature

## Rust / Raw SIMD Contract (updated Phase 2)
- No reefer macro dependency: raw f32x4 SIMD is faster.
- Language: Rust with `#![feature(portable_simd)]` (requires rustc nightly or newer; stable 1.92+ does not support this feature).
- Type: native `core::simd::f32x4` (not array wrapper) for direct register pinning.
- Structure: dense arrays; branchless loops; singularity handled by metric zero.
- Baseline reference: Python `clifford` (structure/reference only, not runtime dependency).
Reference: Dorst et al., arXiv:2311.04744 (Projective GA for Equivariant Transformers) — see .trellis/tasks/main-spec/paper-reference.md
