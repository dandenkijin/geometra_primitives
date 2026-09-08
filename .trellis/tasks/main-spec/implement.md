# Implement Plan
1. Layout: f32x4 blocks (Odd 1 reg, Even 2 reg)
2. Unroll wedge/vee/geometric to FMA/swizzle
3. Branchless solver: singularity -> natural metric zero

# Branchless SIMD snippet (added Phase 2)
// f32x4 odd_plane = {a,b,c,d}; f32x4 line_dir, line_mom split
// Unrolled geometric product: result_lane = fma(a,b,c,d) per component
// Singularity (parallel / infinity): coefficient -> metric zero naturally; no if
