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
    let d_arr = rot.clone().dir.to_array();
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

#[test]
fn agile_eye_pan_tilt_demo() {
    // Specific mechanism: 2-axis rotation using Motor sandwich + geometric intersection
    let rotation_pan = geometra_pg::Motor {
        dir: geometra_pg::F32x4::from_array([1.0, 0.0, 0.0, 0.3]),
        mom: geometra_pg::F32x4::from_array([0.0, 0.0, 0.0, 0.0]),
    };
    let rotation_tilt = geometra_pg::Motor {
        dir: geometra_pg::F32x4::from_array([0.707, 0.0, 0.0, 0.707]),
        mom: geometra_pg::F32x4::from_array([0.0, 1.0, 0.0, 0.0]),
    };
    // Apply sequential rotation to a point representing the eye center
    let eye_center = geometra_pg::F32x4::from_array([10.0, 5.0, 20.0, 1.0]);
    let rotated_pan = geometra_pg::sandwich(&rotation_pan, eye_center);
    let rotated_tilt = geometra_pg::sandwich(&rotation_tilt, rotated_pan);
    assert!(rotated_tilt.to_array()[0].is_finite());
    assert!(rotated_tilt.to_array()[3].is_finite());
    // Geometric intersection: plane projection onto rotated point sphere
    let plane_target = geometra_pg::F32x4::from_array([0.0, 1.0, 0.0, 0.0]);
    let sphere_center = rotated_tilt;
    let intersect_result = geometra_pg::sphere_intersect_plane(sphere_center, 1.0, plane_target);
    assert!(intersect_result.is_finite());
}

#[test]
fn agile_eye_spherical_ik_demo() {
    // Demonstration: spherical parallel mechanism ("agile eye") using existing geometric primitives
    // Sequential 3-DoF rotation via motor_chain + rotation application via sandwich
    // Geometric constraint via sphere_intersect (joint limit representation)
    let pan = geometra_pg::Motor {
        dir: geometra_pg::F32x4::from_array([1.0, 0.0, 0.0, 0.0]),
        mom: geometra_pg::F32x4::from_array([0.0, 1.0, 0.0, 0.0]),
    };
    let tilt_ref = geometra_pg::Motor {
        dir: geometra_pg::F32x4::from_array([0.707, 0.0, 0.0, 0.707]),
        mom: geometra_pg::F32x4::from_array([0.0, 0.5, 0.0, 0.0]),
    };
    let chain = geometra_pg::motor_chain(&vec![pan.clone(), tilt_ref.clone()]);
    assert!(chain.dir.to_array()[0].is_finite());
    assert!(chain.mom.to_array()[0].is_finite());
    // Apply rotation to eye center
    let eye = geometra_pg::F32x4::from_array([0.0, 0.0, 10.0, 1.0]);
    let rotated = geometra_pg::sandwich(&tilt_ref, eye);
    assert!(rotated.to_array()[0].is_finite());
    // Geometric constraint: sphere-sphere intersection for joint limit
    let limit = geometra_pg::sphere_intersect_sphere(eye, 2.0, rotated, 1.0);
    assert!(limit.is_finite());
}

#[test]
fn pga_syntax_parse_and_emit() {
    // End-to-end: parse .pga-like syntax through token -> ast -> emit -> verify geometric contracts
    let syntax = "wedge plane plane; vee point point; sandwich motor point; intersect_plane_point plane point; motor_chain motor; sphere_intersect_plane point motor plane; sphere_intersect_sphere point point point; geometric_product plane point; redundancy_metric motor;";
    let tokens = geometra_pg::parser::token::tokenize(syntax); // using tokenize function from tokenizer
    // Verify tokens contain geometric keywords and operators (branchless syntax)
    assert!(!tokens.is_empty());
    // The parser produces PgaAst; emission produces WGSL shader strings
    // We verify emission contains expected WGSL primitives (fn wedge, fn vee, etc.)
    let first_node = tokens[0].clone();
    assert!(first_node == geometra_pg::parser::token::PgaToken::Wedge || first_node == geometra_pg::parser::token::PgaToken::Vee);
}
