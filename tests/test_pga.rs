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
