use geometra_pg::*;

#[test]
fn branchless_intersect_no_panic() {
    let p: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
    let pt: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    let r = intersect_plane_point(p, pt);
    assert!(r.is_finite());
}
#[test]
fn wedge_vee_no_branch() {
    let p = [1.0, 2.0, 3.0, 4.0];
    let q = [1.0, 1.0, 1.0, 1.0];
    let w = geometra_pg::wedge(p, q);
    let v = geometra_pg::vee(p, q);
    assert!(w.iter().all(|x| x.is_finite()));
    assert!(v.iter().all(|y| y.is_finite()));
}

#[test]
fn geometric_product_no_panic() {
    let p = [1.0, 0.0, 0.0, 1.0];
    let pt = [2.0, 3.0, 4.0, 5.0];
    let r = geometra_pg::geometric_product(p, pt);
    assert!(r.is_finite());
}
#[test]
fn sandwich_no_panic() {
    let m = geometra_pg::Motor { dir: [1.0, 0.0, 0.0, 1.0], mom: [0.0, 1.0, 0.0, 0.0] };
    let pt = [2.0, 3.0, 4.0, 5.0];
    let r = geometra_pg::sandwich(&m, pt);
    assert!(r.iter().all(|v| v.is_finite()));
}

#[test]
fn point_line_intersect_branchless() {
    let ln = geometra_pg::Line { dir: [1.0, 0.0, 0.0, 0.0], mom: [0.0, 1.0, 0.0, 0.0] };
    let pt = [1.0, 2.0, 3.0, 4.0];
    let r = geometra_pg::point_line_intersect(pt, &ln);
    assert!(r.is_finite());
}

#[test]
fn redundancy_metric_no_branch() {
    let m = geometra_pg::Motor { dir: [1.0, 0.0, 0.0, 0.0], mom: [0.0, 0.0, 0.0, 0.0] };
    let r = geometra_pg::redundancy_metric(&m);
    assert!(r.is_finite());
    assert!(r >= 0.0);
}

#[test]
fn motor_chain_no_panic() {
    let chain = vec![
        geometra_pg::Motor { dir: [1.0, 0.0, 0.0, 0.0], mom: [0.0, 0.0, 0.0, 0.0] },
    ];
    let c = geometra_pg::motor_chain(&chain);
    assert!(c.dir.iter().all(|v| v.is_finite()));
    assert!(c.mom.iter().all(|v| v.is_finite()));
}
#[test]
fn sphere_intersect_plane_metric() {
    let c = [0.0, 0.0, 0.0, 1.0];
    let r = 1.0;
    let p = [0.0, 0.0, 1.0, 0.0];
    let d = geometra_pg::sphere_intersect_plane(c, r, p);
    assert!(d.is_finite());
}

#[test]
fn sphere_intersect_sphere_metric() {
    let c1 = [0.0, 0.0, 0.0, 1.0];
    let c2 = [3.0, 0.0, 0.0, 1.0];
    let d = geometra_pg::sphere_intersect_sphere(c1, 1.0, c2, 1.0);
    assert!(d.is_finite());
}
