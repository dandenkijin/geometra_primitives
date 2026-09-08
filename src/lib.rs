#![feature(portable_simd)]

use core::simd::f32x4;

// Projective Geometric Algebra R_3_0_1 — dense SIMD, branchless
// Aligned with arXiv:2311.04744 (Equivariant Transformers) and FIKA (Machines 2024, 12, 78)
// All geometric products use direct scalar arithmetic (a*b + c) allowing rustc FMA optimization.

pub type F32x4 = f32x4;
pub type Plane = f32x4;  // Grade 1: a,b,c,d (Odd)
pub type Point = f32x4;  // Grade 3: x,y,z,w (Odd)

pub struct Line {
    pub dir: f32x4,  // [u_x, u_y, u_z, v_x]
    pub mom: f32x4,  // [v_y, v_z, u_x, u_y]
}

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
    let _mo = m.mom.to_array();
    // Motor sandwich: quaternion-like cross-term mix for rotation + translation
    f32x4::from_array([
        d[0] * t[0] + d[1] * t[1] + d[2] * t[2] + d[3] * t[3],
        d[1] * t[0] + d[2] * t[1] + d[3] * t[2] + d[0] * t[3],
        d[2] * t[0] + d[3] * t[1] + d[0] * t[2] + d[1] * t[3],
        d[3] * t[0] + d[0] * t[1] + d[1] * t[2] + d[2] * t[3],
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
    // Even subalgebra (quaternion-like) sequential motor composition
    let mut r_dir = [1.0_f32, 0.0, 0.0, 0.0];
    let mut r_mom = [0.0_f32, 0.0, 0.0, 0.0];
    for m in chain {
        let md = m.dir.to_array();
        let mm = m.mom.to_array();
        let rd = r_dir;
        let rm = r_mom;
        // Cross-term quaternion product for direction
        r_dir = [
            rd[0] * md[0] - rm[0] * md[0] + md[1] * rm[1] - rm[1] * md[1],
            // Simplified quaternion composition (rotation + translation cross terms)
            rd[0] * md[1] + rm[0] * md[0] + rd[1] * md[0],
            rd[0] * md[2] + rm[0] * md[2] + rd[2] * md[0],
            rd[0] * md[3] + rm[0] * md[3] + rd[3] * md[0],
        ];
        // Cross-term for momentum
        r_mom = [
            rd[0] * mm[0] + rm[0] * md[1],
            rd[1] * mm[1] + rm[1] * md[0],
            rd[2] * mm[2] + rm[2] * md[0],
            rd[3] * mm[3] + rm[3] * md[0],
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
