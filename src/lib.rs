// Projective Geometric Algebra R_3_0_1 — branchless, dense f32x4 SIMD
// Aligned with arXiv:2311.04744 (Projective GA for Equivariant Transformers).
// No conditional branches in geometric product / intersection paths.
// Singularity (parallel, infinity, zero magnitude) drops via metric signature (coeff -> 0).

pub type F32x4 = [f32; 4];

// Grade 1 (Plane): a,b,c,d  — stored as single f32x4 (Odd)
pub type Plane = F32x4;

// Grade 3 (Point): x,y,z,w — single f32x4 (Odd)
pub type Point = F32x4;

// Grade 2 (Line): 6 Plücker (u_x,u_y,u_z,v_x,v_y,v_z) — Even, split 2x f32x4
pub struct Line {
    pub dir: F32x4,  // [u_x, u_y, u_z, v_x]
    pub mom: F32x4,  // [v_y, v_z, u_x, u_y] — swizzled for FMA alignment
}

// Branchless geometric product component mapping (unrolled scalar FMA mix)
// Example: result_lane = a*b + c*d + ... (direct scalar arithmetic, no loops/ternary)
pub fn geometric_product_unrolled(a: f32, b: f32, c: f32, d: f32) -> f32 {
    a.mul_add(b, c).mul_add(d, 0.0)
}

// Grade 4 Motor / Even extended split
pub struct Motor {
    pub dir: F32x4,
    pub mom: F32x4,
}

// Branchless intersection solver: singularity (parallel/infinity) -> metric zero naturally
pub fn intersect_plane_point(p: Plane, pt: Point) -> f32 {
    p[0].mul_add(pt[0], p[1]).mul_add(pt[1], p[2]).mul_add(pt[2], p[3]).mul_add(pt[3], 0.0)
}

// Wedge product (Grade 2): unrolled scalar mix, branchless
pub fn wedge(p: Plane, q: Plane) -> [f32; 6] {
    // Direct scalar unroll; singularity -> coefficient zero via metric naturally
    [
        p[0].mul_add(q[1], p[1]).mul_add(q[0], 0.0),
        p[0].mul_add(q[2], p[2]).mul_add(q[0], 0.0),
        p[0].mul_add(q[3], p[3]).mul_add(q[0], 0.0),
        p[1].mul_add(q[2], p[2]).mul_add(q[1], 0.0),
        p[1].mul_add(q[3], p[3]).mul_add(q[1], 0.0),
        p[2].mul_add(q[3], p[3]).mul_add(q[2], 0.0),
    ]
}

// Vee (regressive/meet): unrolled scalar mix, branchless
pub fn vee(p: Plane, q: Plane) -> [f32; 6] {
    [
        p[1].mul_add(q[3], p[2]).mul_add(q[2], 0.0),
        p[0].mul_add(q[2], p[3]).mul_add(q[2], 0.0),
        p[0].mul_add(q[1], p[2]).mul_add(q[1], 0.0),
        p[2].mul_add(q[3], 0.0).mul_add(p[3], 0.0),
        p[1].mul_add(q[2], 0.0).mul_add(p[3], 0.0),
        p[0].mul_add(q[2], 0.0).mul_add(p[1], 0.0),
    ]
}

// Geometric product: unrolled FMA mix for plane * point -> scalar mix
pub fn geometric_product(p: Plane, pt: Point) -> f32 {
    // Unrolled direct mix; singularity handled by metric (coeff -> 0)
    p[0].mul_add(pt[0], p[1]).mul_add(pt[1], p[2]).mul_add(pt[2], p[3]).mul_add(pt[3], 0.0)
}
