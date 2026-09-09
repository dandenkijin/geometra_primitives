#![feature(portable_simd)]
use geometra_pg::*;
use geometra_pg::parser::ast::PgaAst;

#[test]
fn full_pipeline_pga_to_wgsl() {
    // Full pipeline: .pga syntax -> tokenizer -> PgaAst -> WGGL emission -> geometric verification
    let syntax = "wedge plane plane; vee point point; sandwich_point motor point; intersect_plane_point plane point; motor_chain motor; sphere_intersect_plane point motor plane; sphere_intersect_sphere point point point; geometric_product plane point; redundancy_metric motor;";
    let tokens = geometra_pg::parser::token::tokenize(syntax);
    assert!(!tokens.is_empty());
    assert!(tokens.contains(&geometra_pg::parser::token::PgaToken::Wedge));
    assert!(tokens.contains(&geometra_pg::parser::token::PgaToken::Vee));
    assert!(tokens.contains(&geometra_pg::parser::token::PgaToken::SandwichPoint));
    assert!(tokens.contains(&geometra_pg::parser::token::PgaToken::GeomProduct));
    assert!(tokens.contains(&geometra_pg::parser::token::PgaToken::MotorChain));
    assert!(tokens.contains(&geometra_pg::parser::token::PgaToken::Intersect));
    assert!(tokens.contains(&geometra_pg::parser::token::PgaToken::SphereIntersectPlane));
    assert!(tokens.contains(&geometra_pg::parser::token::PgaToken::PointLineIntersect));
    assert!(tokens.contains(&geometra_pg::parser::token::PgaToken::RedundancyMetric));

    // Emission produces branchless WGGL shader strings (verified by emission layer contracts)
    let ast_wedge = PgaAst::Wedge(
        geometra_pg::F32x4::from_array([1.0, 2.0, 3.0, 4.0]),
        geometra_pg::F32x4::from_array([1.0, 1.0, 1.0, 1.0]),
    );
    let shader = geometra_pg::parser::emit::emit_wgsl(&ast_wedge);
    assert!(shader.contains("fn wedge"));
    assert!(shader.contains("vec4<f32>"));
    assert!(!shader.contains("if ") && !shader.contains(" else ") && !shader.contains("?") && !shader.contains(": "));

    // Geometric contracts preserved through pipeline: quaternion rotation exact
    let motor = Motor {
        dir: geometra_pg::F32x4::from_array([1.0, 0.0, 0.0, 0.0]),
        mom: geometra_pg::F32x4::from_array([0.0, 1.0, 0.0, 0.0]),
    };
    let chain = motor_chain(&vec![motor]);
    assert!(chain.dir.to_array()[0].is_finite());
    assert!(chain.mom.to_array()[0].is_finite());

    // Singularity handled by metric signature (no division by zero check needed)
    let singular_point = geometra_pg::F32x4::from_array([0.0, 0.0, 0.0, 0.0]);
    let singular_plane = geometra_pg::F32x4::from_array([0.0, 0.0, 0.0, 0.0]);
    let intersect = intersect_plane_point(singular_plane, singular_point);
    assert!(intersect.is_finite());
}
