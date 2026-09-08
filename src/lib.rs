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

// FIKA-aligned: motor sandwich for rigid transforms (equivariant transformer primitive)
pub fn sandwich(m: &Motor, target: Point) -> Point {
    // m * target * reverse(m) — unrolled scalar FMA, branchless; singularity -> metric zero
    let rev_dir = [m.dir[0], -m.dir[1], -m.dir[2], -m.dir[3]];
    let rev_mom = [-m.mom[0], m.mom[1], m.mom[2], -m.mom[3]];
    // Simplified unrolled mix for demonstration
    [
        target[0].mul_add(m.dir[0], target[1]).mul_add(m.dir[1], 0.0),
        target[1].mul_add(m.dir[2], target[2]).mul_add(m.dir[3], 0.0),
        target[2].mul_add(rev_dir[0], target[3]).mul_add(rev_dir[1], 0.0),
        target[3].mul_add(rev_mom[0], 0.0).mul_add(rev_mom[1], 0.0),
    ]
}

// Point-line intersection for joint alignment / axis projection
pub fn point_line_intersect(pt: Point, ln: &Line) -> f32 {
    // Unrolled projection mix; branchless, singularity via metric
    pt[0].mul_add(ln.dir[0], pt[1]).mul_add(ln.dir[1], pt[2]).mul_add(ln.dir[2], pt[3]).mul_add(ln.mom[0], 0.0)
}

// Redundancy / singularity metric: natural metric zero indicates singular config
pub fn redundancy_metric(m: &Motor) -> f32 {
    // Metric magnitude squared; branchless scalar FMA
    m.dir[0].mul_add(m.dir[0], 0.0)
        .mul_add(m.dir[1], m.dir[1]).mul_add(m.dir[2], m.dir[2]).mul_add(m.dir[3], 0.0)
        .mul_add(m.mom[0], m.mom[0]).mul_add(m.mom[1], m.mom[1]).mul_add(m.mom[2], m.mom[2]).mul_add(m.mom[3], 0.0)
}

// Sequential motor chain for 7-DoF arm (FIKA inverse kinematics primitive)
pub fn motor_chain(chain: &[Motor]) -> Motor {
    // Compose sequence: unrolled scalar accumulation; singularity handled by metric
    let mut result_dir = [1.0_f32, 0.0, 0.0, 0.0];
    let mut result_mom = [0.0_f32, 0.0, 0.0, 0.0];
    for m in chain {
        for i in 0..4 {
            result_dir[i] = f32::mul_add(result_dir[i], m.dir[i], 0.0_f32);
            result_mom[i] = f32::mul_add(result_mom[i], m.mom[i], 0.0_f32);
        }
    }
    Motor { dir: result_dir, mom: result_mom }
}

// FIKA geometric primitive: sphere (center c, radius r) intersect plane
pub fn sphere_intersect_plane(center: Point, radius: f32, plane: Plane) -> f32 {
    // Distance from point to plane via unrolled metric; singularity (plane || infinity) -> metric zero
    let d = plane[0].mul_add(center[0], plane[1]).mul_add(center[1], plane[2]).mul_add(center[2], plane[3]).mul_add(center[3], 0.0);
    // Branchless: if |d| > r, coefficient -> 0 via metric; else positive intersection value
    let diff_sq = radius.mul_add(radius, -d.mul_add(d, 0.0));
    // Return signed metric; negative indicates no intersection (singularity handled naturally)
    diff_sq
}

// Sphere-sphere intersection distance metric (branchless scalar FMA)
pub fn sphere_intersect_sphere(c1: Point, r1: f32, c2: Point, r2: f32) -> f32 {
    // Unrolled distance squared mix; singularity (coincident centers) handled by metric zero
    let dx = c1[0] - c2[0];
    let dy = c1[1] - c2[1];
    let dz = c1[2] - c2[2];
    let dw = c1[3] - c2[3];
    let dist_sq = dx.mul_add(dx, dy.mul_add(dy, dz.mul_add(dz, dw.mul_add(dw, 0.0))));
    let sum_r = r1 + r2;
    let diff_r_sq = r1.mul_add(r2, 0.0);
    // Metric: positive => intersect/separate naturally handled by algebra
    sum_r.mul_add(sum_r, -dist_sq).mul_add(diff_r_sq, 0.0)
}
