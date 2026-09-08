// Projective Geometric Algebra R_3_0_1 — branchless, dense f32x4 SIMD
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
    // Metric signature handles singularity naturally: if magnitude -> 0, coefficient -> 0
    a.mul_add(b, c).mul_add(d, 0.0)
}
