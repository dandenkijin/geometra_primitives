# Compiler Full — Design

## Architecture
Parser (`token` -> `ast` -> `ir`) -> Emission (`emit`) -> `WGSL`. Multi-backend slot preserved.

## Graded Multivector (Full R_3_0_1)
PgaIr models full 16-dimensional R_3_0_1 graded multivector:
- Scalar (f32) — Grade 0
- Vector ([f32; 4]) — Grade 1 (Plane coefficients)
- Bivector ([f32; 6]) — Grade 2 (Line / rotation axes / attention cross-terms)
- Trivector ([f32; 4]) — Grade 3 (Point homogeneous coordinates)
- Pseudoscalar (f32) — Grade 4 (metric scale / singularity)

## Optimization Passes
1. `prune_zeros`: drop blade coefficients evaluating to zero (singularity metric signature).
2. `fold_constants`: evaluate constant geometric expressions at compile time.
3. `merge_terms`: combine like-grade blade coefficients with compatible masks.

All passes: branchless, unrolled scalar FMA (`a*b+c`), dense arrays.

## Backend / Compatibility
- `WGSL` shader emission (`emit.rs`) exact contracts; multi-backend independent.
- `rustc` nightly (`#![feature(portable_simd)]`); `core::simd::f32x4` native.
- No unstable `.Logos` conflicts at compile/build time (manual `.pga` tokenizer accurate).
- FIKA (`Machines 2024, 12, 78`) inverse kinematics primitive demonstrated (`agile_eye_pan_tilt_demo`, `agile_eye_spherical_ik_demo`).

## Trade-offs / Risks
- Full graded multivector requires more optimization logic than sub-algebra family.
- `PgaIr` must track grade masks explicitly to prevent invalid geometric products across grades.
- Emission layer (`WGSL`) remains independent of geometric contracts — future backends (`SPIR-V`, `Metal`) can reuse `PgaAst` + `PgaIr`.
