Branchless geometric product (f32x4, FMA unrolled, singularities drop via metric):
odd_plane = f32x4(a,b,c,d)
even_dir = f32x4(ux,uy,uz,vx)
even_mom = f32x4(...) // split
// Wedge: direct scalar mix; no if/ternary; infinity -> zero coeff
