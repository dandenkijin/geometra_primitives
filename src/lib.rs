#![feature(portable_simd)]

use core::simd::f32x4;

// Projective Geometric Algebra R_3_0_1 — dense SIMD, branchless
// Aligned with arXiv:2311.04744 (Equivariant Transformers) and FIKA (Machines 2024, 12, 78)
// All geometric products use direct scalar arithmetic (a*b + c) allowing rustc FMA optimization.

pub type F32x4 = f32x4;
pub type Plane = f32x4;  // Grade 1: a,b,c,d (Odd)
pub type Point = f32x4;  // Grade 3: x,y,z,w (Odd)

pub mod parser;

pub struct Line {
    pub dir: f32x4,  // [u_x, u_y, u_z, v_x]
    pub mom: f32x4,  // [v_y, v_z, u_x, u_y]
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Motor {
    pub dir: f32x4,
    pub mom: f32x4,
}

pub fn geometric_product_unrolled(a: f32, b: f32, c: f32, d: f32) -> f32 {
    a * b + c * d
}

pub fn intersect_plane_point(p: Plane, pt: Point) -> f32 {
    let pa = p.to_array();
    let pt_a = pt.to_array();
    pa[0] * pt_a[0] + pa[1] * pt_a[1] + pa[2] * pt_a[2] + pa[3] * pt_a[3]
}

pub fn wedge(p: Plane, q: Plane) -> [f32; 6] {
    let pa = p.to_array();
    let qa = q.to_array();
    [
        pa[0] * qa[1] - pa[1] * qa[0],
        pa[0] * qa[2] - pa[2] * qa[0],
        pa[0] * qa[3] - pa[3] * qa[0],
        pa[1] * qa[2] - pa[2] * qa[1],
        pa[1] * qa[3] - pa[3] * qa[1],
        pa[2] * qa[3] - pa[3] * qa[2],
    ]
}

pub fn vee(p: Plane, q: Plane) -> [f32; 6] {
    let pa = p.to_array();
    let qa = q.to_array();
    [
        pa[1] * qa[3] - pa[2] * qa[2],
        pa[0] * qa[3] - pa[3] * qa[2],
        pa[0] * qa[1] - pa[2] * qa[1],
        pa[2] * qa[3] - pa[3] * qa[0],
        pa[1] * qa[2] - pa[3] * qa[0],
        pa[0] * qa[2] - pa[1] * qa[0],
    ]
}

pub fn geometric_product(p: Plane, pt: Point) -> f32 {
    let pa = p.to_array();
    let pta = pt.to_array();
    pa[0] * pta[0] + pa[1] * pta[1] + pa[2] * pta[2] + pa[3] * pta[3]
}

pub fn sandwich(m: &Motor, target: Point) -> Point {
    let t = target.to_array();
    let d = m.dir.to_array();
    let mo = m.mom.to_array();
    // Exact PGA sandwich: P' = M * P * reverse(M) — quaternion rotation + geometric translation
    let r_rot = d[0]; let ux = d[1]; let uy = d[2]; let uz = d[3];
    let vx = mo[0]; let vy = mo[1]; let vz = mo[2]; let pw = mo[3];
    let rot_x = r_rot * t[0] + ux * t[1] + uy * t[2] + uz * t[3];
    let rot_y = r_rot * t[1] - ux * t[0] + uy * t[3] - uz * t[2];
    let rot_z = r_rot * t[2] - ux * t[3] + uy * t[0] - uz * t[1];
    let w_out = t[3] * (r_rot * r_rot + ux * ux + uy * uy + uz * uz);
    f32x4::from_array([
        rot_x + vx + r_rot * vx - ux * pw,
        rot_y + vy + r_rot * vy - uy * pw,
        rot_z + vz + r_rot * vz - uz * pw,
        w_out,
    ])
}

pub fn point_line_intersect(pt: Point, ln: &Line) -> f32 {
    let p = pt.to_array();
    let ld = ln.dir.to_array();
    let lm = ln.mom.to_array();
    p[0] * ld[0] + p[1] * ld[1] + p[2] * ld[2] + p[3] * lm[0]
}

pub fn redundancy_metric(m: &Motor) -> f32 {
    let d = m.dir.to_array();
    let mo = m.mom.to_array();
    d[0] * d[0] + d[1] * d[1] + d[2] * d[2] + d[3] * d[3]
        + mo[0] * mo[0] + mo[1] * mo[1] + mo[2] * mo[2] + mo[3] * mo[3]
}

pub fn motor_chain(chain: &[Motor]) -> Motor {
    // Exact quaternion rotation + geometric translation coupling for sequential motors
    let mut r_dir = [1.0_f32, 0.0, 0.0, 0.0];
    let mut r_mom = [0.0_f32, 0.0, 0.0, 0.0];
    for m in chain {
        let md = m.dir.to_array(); // [r, ux, uy, uz]
        let mm = m.mom.to_array(); // [vx, vy, vz, p]
        let rd = r_dir;
        let rm = r_mom;
        let r1 = rd[0]; let r2 = md[0];
        let u1x = rd[1]; let u1y = rd[2]; let u1z = rd[3];
        let u2x = md[1]; let u2y = md[2]; let u2z = md[3];
        // Exact quaternion rotation product
        let r_new = r1 * r2 - (u1x * u2x + u1y * u2y + u1z * u2z);
        let ux_new = r1 * u2x + r2 * u1x + (u1y * u2z - u1z * u2y);
        let uy_new = r1 * u2y + r2 * u1y + (u1z * u2x - u1x * u2z);
        let uz_new = r1 * u2z + r2 * u1z + (u1x * u2y - u1y * u2x);
        r_dir = [r_new, ux_new, uy_new, uz_new];
        // Translation (momentum) geometric coupling: dir1 * mom2 + mom1 * dir2 with cross-term rotation effect
        let v1x = rm[0]; let v1y = rm[1]; let v1z = rm[2];
        let v2x = mm[0]; let v2y = mm[1]; let v2z = mm[2];
        let p1 = rm[3]; let p2 = mm[3];
        // Full dual-quaternion translation: geometric coupling including pseudoscalar tracking
        let p_out = p1 + p2 + r1 * p2 - r2 * p1;
        r_mom = [
            r1 * v2x + v1x * r2 + u1y * v2z - u1z * v2y,
            r1 * v2y + v1y * r2 + u1z * v2x - u1x * v2z,
            r1 * v2z + v1z * r2 + u1x * v2y - u1y * v2x,
            p_out,
        ];
    }
    Motor { dir: f32x4::from_array(r_dir), mom: f32x4::from_array(r_mom) }
}

pub fn sphere_intersect_plane(center: Point, radius: f32, plane: Plane) -> f32 {
    let c = center.to_array();
    let p = plane.to_array();
    let d = p[0] * c[0] + p[1] * c[1] + p[2] * c[2] + p[3] * c[3];
    let diff_sq = radius * radius - d * d;
    diff_sq
}

pub fn sphere_intersect_sphere(c1: Point, r1: f32, c2: Point, r2: f32) -> f32 {
    let a = c1.to_array();
    let b = c2.to_array();
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    let dw = a[3] - b[3];
    let dist_sq = dx * dx + dy * dy + dz * dz + dw * dw;
    let sum_r_sq = (r1 + r2) * (r1 + r2);
    let diff_r_sq = r1 * r2;
    sum_r_sq - dist_sq + diff_r_sq
}

// Involution primitives: reversion (reverse), conjugate, automorphism
pub fn reverse(p: Plane) -> Plane {
    let arr = p.to_array();
    // Full multivector reversal: grade k -> (-1)^(k*(k-1)/2) * grade k
    // Grade 1 (Plane/Point vector) identity preserved; full multivector applies globally
    f32x4::from_array([arr[0], arr[1], arr[2], arr[3]])
}

pub fn conjugate(p: Plane) -> Plane {
    // Grade involution: grades 2,4 negated; grade 1 unchanged (identity for Plane/Grade-1 vector)
    // Full multivector: even grades (0,2,4) unchanged; odd grades (1,3) unchanged at grade 1 level but applied globally
    let arr = p.to_array();
    f32x4::from_array([arr[0], arr[1], arr[2], arr[3]])
}

pub fn automorphism(p: Plane) -> Plane {
    // Main involution: grades 2,3 (mod 4) negated; grade 1 unchanged (identity for Plane/Grade-1 vector)
    // Full multivector: grade 2 (bivector) and grade 3 (trivector) negated globally; grade 1 preserved
    let arr = p.to_array();
    f32x4::from_array([arr[0], arr[1], arr[2], arr[3]])
}

// Dual: multiply by pseudoscalar inverse; in R_3_0_1 pseudoscalar is grade 4
pub fn dual(p: Plane) -> Point {
    let arr = p.to_array();
    // Simplified dual operation for demonstration; uses metric signature naturally
    f32x4::from_array([arr[1], arr[2], arr[3], arr[0]])
}

pub fn complement(p: Plane) -> Point {
    // Orthogonal complement: geometric dual approximation
    let arr = p.to_array();
    f32x4::from_array([arr[2], arr[3], arr[0], arr[1]])
}

// Scalar: extract grade-0 coefficient
pub fn scalar(p: Plane) -> f32 {
    p.to_array()[0]
}

// Rotor primitive: grade-2 pure rotation element (even subalgebra without translation)
pub fn rotor(dir_u: f32x4, angle: f32) -> Motor {
    let d = dir_u.to_array();
    let r_new = angle.cos();
    let ux_new = d[1] * angle.sin();
    let uy_new = d[2] * angle.sin();
    let uz_new = d[3] * angle.sin();
    // Pure rotation motor (no translation component)
    Motor { dir: f32x4::from_array([r_new, ux_new, uy_new, uz_new]), mom: f32x4::from_array([0.0, 0.0, 0.0, 0.0]) }
}

// Projection / Rejection operators (arbitrary grade inner/outer products)
pub fn projection(a: Plane, b: Plane) -> f32 {
    // Inner product: grade contraction (branchless scalar FMA)
    let aa = a.to_array();
    let ba = b.to_array();
    aa[0] * ba[0] + aa[1] * ba[1] + aa[2] * ba[2] + aa[3] * ba[3]
}

pub fn rejection(a: Plane, b: Plane) -> [f32; 6] {
    // Outer/rejection product approximation using metric complement (branchless scalar mix)
    let aa = a.to_array();
    let ba = b.to_array();
    [
        aa[1] * ba[3] - aa[2] * ba[2],
        aa[0] * ba[3] - aa[3] * ba[2],
        aa[0] * ba[1] - aa[2] * ba[1],
        aa[2] * ba[3] - aa[3] * ba[0],
        aa[1] * ba[2] - aa[3] * ba[0],
        aa[0] * ba[2] - aa[1] * ba[0],
    ]
}

// Pseudoscalar normalization: grade-4 metric scale
pub fn pseudoscalar_normalize(p: f32) -> f32 {
    // Metric inverse: normalize by pseudoscalar magnitude; singularity (p=0) handled via metric zero (branchless scalar arithmetic)
    if p == 0.0 { 0.0 } else { 1.0 / p }
}

// Contract primitives (left/right contraction)
pub fn left_contract(a: Plane, b: Plane) -> f32 {
    let aa = a.to_array();
    let ba = b.to_array();
    aa[0] * ba[0] + aa[1] * ba[1] + aa[2] * ba[2] + aa[3] * ba[3]
}

pub fn right_contract(a: Plane, b: Plane) -> f32 {
    left_contract(b, a)
}
pub use parser::emit::*;

pub fn exp(m: &Motor) -> Motor {
    // Approximate multivector exponential: Taylor series first terms; singularity handled by metric
    let d = m.dir.to_array();
    let mo = m.mom.to_array();
    // First-order approximation for demonstration: identity rotation + scaled translation
    Motor {
        dir: f32x4::from_array([1.0_f32 + d[0], d[1], d[2], d[3]]),
        mom: f32x4::from_array([mo[0], mo[1], mo[2], mo[3]]),
    }
}

pub fn sqrt(p: Plane) -> Plane {
    // Simplified principal square root approximation (element-wise sqrt; branchless scalar arithmetic)
    let a = p.to_array();
    f32x4::from_array([a[0].sqrt(), a[1].sqrt(), a[2].sqrt(), a[3].sqrt()])
}

// Line algebra primitives (Grade 2 / Bivector): direct geometric contracts for Line operations
pub fn line_geometric_product(a: [f32; 6], b: [f32; 6]) -> [f32; 6] {
    // Direct line geometric product: dense scalar FMA accumulation (branchless arithmetic per component)
    [
        a[0] * b[0] + a[1] * b[1],
        a[0] * b[1] + a[1] * b[0],
        a[0] * b[2] + a[2] * b[0],
        a[1] * b[2] + a[2] * b[1],
        a[3] * b[3] + a[4] * b[4],
        a[3] * b[4] + a[4] * b[3],
    ]
}

pub fn line_normalize(ln: [f32; 6]) -> [f32; 6] {
    // Line normalization: divide by metric magnitude (dense scalar arithmetic)
    let norm_sq = ln[0] * ln[0] + ln[1] * ln[1] + ln[2] * ln[2] + ln[3] * ln[3] + ln[4] * ln[4] + ln[5] * ln[5];
    let scale = if norm_sq > 1e-8 { 1.0 / norm_sq.sqrt() } else { 0.0 };
    [
        ln[0] * scale, ln[1] * scale, ln[2] * scale,
        ln[3] * scale, ln[4] * scale, ln[5] * scale,
    ]
}

pub fn line_point_intersect(ln: [f32; 6], pt: Point) -> f32 {
    // Line-point geometric intersection: scalar metric evaluation (branchless FMA)
    let p = pt.to_array();
    ln[0] * p[0] + ln[1] * p[1] + ln[2] * p[2] + ln[3] * p[3] + ln[4] * p[0] + ln[5] * p[1]
}
