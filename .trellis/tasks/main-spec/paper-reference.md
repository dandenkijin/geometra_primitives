Reference: Dorst et al. / arXiv:2311.04744 — "Euclidean, Projective, Conformal: Choosing a Geometric Algebra for Equivariant Transformers" (Nov 2023).
This runtime implements the Projective R_3_0_1 branch discussed in the paper, optimized as branchless SIMD primitives for equivariant transformer pipelines.

Rust Toolchain Requirement:
- `#![feature(portable_simd)]` requires rustc nightly or newer (unstable feature; stable 1.92 produces E0554).
- See `.trellis/tasks/main-spec/design.md`.
