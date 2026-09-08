# R_3_0_1 PGA Spec — PRD
## Constraints
- No branches in product loops or solvers.
- All bilinear products unrolled to scalar FMA / SIMD swizzle.
- Singularities (parallel, infinity, zero-magnitude) drop out via metric signature; no division-by-zero guard.
## Deliverables
- f32x4 layout (Odd/Even split)
- Unrolled wedge ^, vee v, geometric *
- Branchless intersection solver
