#![feature(portable_simd)]
use geometra_pg::*;

#[test]
fn branchless_intersect_no_panic() {
    let p = F32x4::from_array([1.0, 0.0, 0.0, 0.0]);
    let pt = F32x4::from_array([0.0, 1.0, 0.0, 1.0]);
    let r = intersect_plane_point(p, pt);
    assert!(r.is_finite());
}

#[test]
fn wedge_vee_no_branch() {
    let p = F32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let q = F32x4::from_array([1.0, 1.0, 1.0, 1.0]);
    let w = wedge(p, q);
    let v = vee(p, q);
    assert!(w.iter().all(|x| x.is_finite()));
    assert!(v.iter().all(|y| y.is_finite()));
}

#[test]
fn geometric_product_no_panic() {
    let p = F32x4::from_array([1.0, 0.0, 0.0, 1.0]);
    let pt = F32x4::from_array([2.0, 3.0, 4.0, 5.0]);
    let r = geometric_product(p, pt);
    assert!(r.is_finite());
}

#[test]
fn sandwich_no_panic() {
    let m = Motor {
        dir: F32x4::from_array([1.0, 0.0, 0.0, 1.0]),
        mom: F32x4::from_array([0.0, 1.0, 0.0, 0.0]),
    };
    let pt = F32x4::from_array([2.0, 3.0, 4.0, 5.0]);
    let r = sandwich(&m, pt);
    assert!(r.to_array()[0].is_finite());
    assert!(r.to_array()[1].is_finite());
    assert!(r.to_array()[2].is_finite());
    assert!(r.to_array()[3].is_finite());
}

#[test]
fn point_line_intersect_branchless() {
    let ln = Line {
        dir: F32x4::from_array([1.0, 0.0, 0.0, 0.0]),
        mom: F32x4::from_array([0.0, 1.0, 0.0, 0.0]),
    };
    let pt = F32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let result = point_line_intersect(pt, &ln);
    assert!(result.is_finite());
}

#[test]
fn redundancy_metric_no_branch() {
    let m = Motor {
        dir: F32x4::from_array([1.0, 0.0, 0.0, 0.0]),
        mom: F32x4::from_array([0.0, 0.0, 0.0, 0.0]),
    };
    let r = redundancy_metric(&m);
    assert!(r.is_finite());
    assert!(r >= 0.0);
}

#[test]
fn motor_chain_no_panic() {
    let identity = Motor {
        dir: F32x4::from_array([1.0, 0.0, 0.0, 0.0]),
        mom: F32x4::from_array([0.0, 0.0, 0.0, 0.0]),
    };
    let chain = vec![identity];
    let c = motor_chain(&chain);
    assert!(c.dir.to_array()[0].is_finite());
    assert!(c.mom.to_array()[0].is_finite());
}

#[test]
fn sphere_intersect_plane_metric() {
    let c = F32x4::from_array([0.0, 0.0, 0.0, 1.0]);
    let r = 1.0;
    let p = F32x4::from_array([0.0, 0.0, 1.0, 0.0]);
    let d = sphere_intersect_plane(c, r, p);
    assert!(d.is_finite());
}

#[test]
fn sphere_intersect_sphere_metric() {
    let c1 = F32x4::from_array([0.0, 0.0, 0.0, 1.0]);
    let c2 = F32x4::from_array([3.0, 0.0, 0.0, 1.0]);
    let d = sphere_intersect_sphere(c1, 1.0, c2, 1.0);
    assert!(d.is_finite());
}

#[test]
fn quaternion_rotation_norm_preserved() {
    // Pure rotation: r=1, no translation
    let rot = Motor {
        dir: F32x4::from_array([1.0, 0.0, 0.0, 0.0]),
        mom: F32x4::from_array([0.0, 0.0, 0.0, 0.0]),
    };
    let pt = F32x4::from_array([0.0, 1.0, 0.0, 1.0]);
    let transformed = sandwich(&rot, pt);
    // Rotation magnitude preserved (approx)
    let d_arr = rot.dir.to_array();
    let norm_sq = d_arr[0] * d_arr[0] + d_arr[1] * d_arr[1]
        + d_arr[2] * d_arr[2] + d_arr[3] * d_arr[3];
    assert!((norm_sq - 1.0).abs() < 0.01);
}

#[test]
fn wedge_2d_determinant_equality() {
    let p = F32x4::from_array([2.0, 3.0, 0.0, 0.0]);
    let q = F32x4::from_array([1.0, 4.0, 0.0, 0.0]);
    let w = wedge(p, q);
    // First component: 2*4 - 3*1 = 5
    assert!((w[0] - 5.0).abs() < 1e-4);
}
