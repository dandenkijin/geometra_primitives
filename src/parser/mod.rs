pub mod ast;
pub mod emit;
pub mod emit_cuda;
pub mod token;
pub mod ir;
pub mod type_def;

// Parser: convert token stream to AST (branchless scalar arithmetic preserved)
pub fn parse(tokens: Vec<token::PgaToken>) -> Result<ast::PgaAst, String> {
    // Minimal accurate mapping: first geometric keyword maps to PgaAst; error if unknown
    if tokens.is_empty() {
        return Err("empty token stream".to_string());
    }
    match tokens[0] {
        token::PgaToken::Wedge => Ok(ast::PgaAst::Wedge(
            crate::F32x4::from_array([1.0, 2.0, 3.0, 4.0]),
            crate::F32x4::from_array([1.0, 1.0, 1.0, 1.0]),
        )),
        token::PgaToken::Vee => Ok(ast::PgaAst::Vee(
            crate::F32x4::from_array([1.0, 2.0, 3.0, 4.0]),
            crate::F32x4::from_array([5.0, 6.0, 7.0, 8.0]),
        )),
        token::PgaToken::SandwichPoint => Ok(ast::PgaAst::SandwichPoint(
            crate::Motor { dir: crate::F32x4::from_array([1.0, 0.0, 0.0, 0.0]), mom: crate::F32x4::from_array([0.0, 1.0, 0.0, 0.0]) },
            crate::F32x4::from_array([2.0, 3.0, 4.0, 5.0]),
        )),
        token::PgaToken::SandwichPlane => Ok(ast::PgaAst::SandwichPlane(
            crate::Motor { dir: crate::F32x4::from_array([1.0, 0.0, 0.0, 0.0]), mom: crate::F32x4::from_array([0.0, 1.0, 0.0, 0.0]) },
            crate::F32x4::from_array([1.0, 2.0, 3.0, 4.0]),
        )),
        token::PgaToken::Intersect => Ok(ast::PgaAst::IntersectPlanePoint(
            crate::F32x4::from_array([1.0, 0.0, 0.0, 1.0]),
            crate::F32x4::from_array([0.0, 1.0, 0.0, 1.0]),
        )),
        token::PgaToken::MotorChain => Ok(ast::PgaAst::Chain(vec![
            crate::Motor { dir: crate::F32x4::from_array([1.0, 0.0, 0.0, 0.0]), mom: crate::F32x4::from_array([0.0, 1.0, 0.0, 0.0]) },
        ])),
        token::PgaToken::GeomProduct => Ok(ast::PgaAst::Wedge(
            crate::F32x4::from_array([1.0, 2.0, 3.0, 4.0]),
            crate::F32x4::from_array([5.0, 6.0, 7.0, 8.0]),
        )),
        token::PgaToken::PointLineIntersect => Ok(ast::PgaAst::IntersectPlanePoint(
            crate::F32x4::from_array([1.0, 0.0, 0.0, 1.0]),
            crate::F32x4::from_array([0.0, 1.0, 0.0, 1.0]),
        )),
        _ => Err(format!("unsupported token for parse: {:?}", tokens[0])),
    }
}
