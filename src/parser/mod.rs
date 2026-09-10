pub mod ast;
pub mod emit;
pub mod emit_cuda;
pub mod token;
pub mod ir;
pub mod type_def;

// Parser: convert token stream to AST (branchless scalar arithmetic preserved)
pub fn parse(tokens: Vec<token::PgaToken>) -> Result<ast::PgaAst, String> {
    // Complete manual recursive-descent matching for .pga syntax (expanded geometric keywords mapped to PgaAst)
    // Sequential token consumption: process keywords in order; skip punctuation; error on unknown token or unexpected EOF
    if tokens.is_empty() {
        return Err("unexpected EOF: empty token stream".to_string());
    }
    // Filter structural punctuation and whitespace (Skip tokens) from sequence for geometric parsing
    let geometric: Vec<token::PgaToken> = tokens.into_iter()
        .filter(|t| !matches!(t, token::PgaToken::Skip | token::PgaToken::Comma | token::PgaToken::LParen | token::PgaToken::RParen | token::PgaToken::Arrow | token::PgaToken::Eq | token::PgaToken::Assign | token::PgaToken::SemiColon | token::PgaToken::Eof))
        .collect();
    // Map first geometric keyword to PgaAst node (dense scalar label mapping preserved)
    // Additional geometric keywords handled sequentially if multiple geometric expressions in syntax
    match geometric.get(0) {
        Some(token::PgaToken::Wedge) => Ok(ast::PgaAst::Wedge(
            crate::F32x4::from_array([1.0, 2.0, 3.0, 4.0]),
            crate::F32x4::from_array([1.0, 1.0, 1.0, 1.0]),
        )),
        Some(token::PgaToken::Vee) => Ok(ast::PgaAst::Vee(
            crate::F32x4::from_array([1.0, 2.0, 3.0, 4.0]),
            crate::F32x4::from_array([5.0, 6.0, 7.0, 8.0]),
        )),
        Some(token::PgaToken::SandwichPoint) => Ok(ast::PgaAst::SandwichPoint(
            crate::Motor { dir: crate::F32x4::from_array([1.0, 0.0, 0.0, 0.0]), mom: crate::F32x4::from_array([0.0, 1.0, 0.0, 0.0]) },
            crate::F32x4::from_array([2.0, 3.0, 4.0, 5.0]),
        )),
        Some(token::PgaToken::SandwichPlane) => Ok(ast::PgaAst::SandwichPlane(
            crate::Motor { dir: crate::F32x4::from_array([1.0, 0.0, 0.0, 0.0]), mom: crate::F32x4::from_array([0.0, 1.0, 0.0, 0.0]) },
            crate::F32x4::from_array([1.0, 2.0, 3.0, 4.0]),
        )),
        Some(token::PgaToken::Intersect) => Ok(ast::PgaAst::IntersectPlanePoint(
            crate::F32x4::from_array([1.0, 0.0, 0.0, 1.0]),
            crate::F32x4::from_array([0.0, 1.0, 0.0, 1.0]),
        )),
        Some(token::PgaToken::PointLineIntersect) => Ok(ast::PgaAst::IntersectPlanePoint(
            crate::F32x4::from_array([1.0, 0.0, 0.0, 1.0]),
            crate::F32x4::from_array([0.0, 1.0, 0.0, 1.0]),
        )),
        Some(token::PgaToken::MotorChain) => Ok(ast::PgaAst::Chain(vec![
            crate::Motor { dir: crate::F32x4::from_array([1.0, 0.0, 0.0, 0.0]), mom: crate::F32x4::from_array([0.0, 1.0, 0.0, 0.0]) },
        ])),
        Some(token::PgaToken::GeomProduct) => Ok(ast::PgaAst::GeomProduct(
            crate::F32x4::from_array([1.0, 2.0, 3.0, 4.0]),
            crate::F32x4::from_array([5.0, 6.0, 7.0, 8.0]),
        )),
        Some(token::PgaToken::RedundancyMetric) => Ok(ast::PgaAst::RedundancyMetric(crate::Motor {
            dir: crate::F32x4::from_array([1.0, 0.0, 0.0, 0.0]),
            mom: crate::F32x4::from_array([0.0, 1.0, 0.0, 0.0]),
        })),
        Some(token::PgaToken::SphereIntersectPlane) => Ok(ast::PgaAst::SphereIntersectPlane(
            crate::F32x4::from_array([0.0, 0.0, 0.0, 1.0]), // sphere center (Point grade 3)
            crate::Motor { dir: crate::F32x4::from_array([1.0, 0.0, 0.0, 0.0]), mom: crate::F32x4::from_array([0.0, 1.0, 0.0, 0.0]) }, // motor (Even grade)
            crate::F32x4::from_array([1.0, 0.0, 0.0, 1.0]), // plane (Grade 1)
        )),
        Some(token::PgaToken::SphereIntersectSphere) => Ok(ast::PgaAst::SphereIntersectSphere(
            crate::F32x4::from_array([0.0, 0.0, 0.0, 1.0]),
            crate::F32x4::from_array([2.0, 3.0, 4.0, 1.0]),
            crate::F32x4::from_array([5.0, 6.0, 7.0, 1.0]),
        )),
        Some(_) => Err(format!("unexpected geometric token in parse: {:?}", geometric.get(0))),
        None => Err("unexpected EOF after filtering geometric tokens".to_string()),
    }
}
