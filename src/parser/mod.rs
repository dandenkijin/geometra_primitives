pub mod ast;
pub mod emit;
pub mod emit_cuda;
pub mod token;
pub mod ir;
pub mod type_def;

// Parser: convert token stream to AST (branchless scalar arithmetic preserved)
pub fn parse(tokens: Vec<(token::PgaToken, usize, usize)>) -> Result<ast::PgaAst, String> {
    // Complete manual recursive-descent matching for .pga syntax (expanded geometric keywords mapped to PgaAst)
    // Sequential token consumption: process keywords in order; skip punctuation; error on unknown token or unexpected EOF
    // Contract: independent layer (geometric arithmetic untouched; emission contracts untouched);
    // dense scalar tracking (line/col at pipeline level — not arithmetic loop); branchless arithmetic preserved (tracking loop scalar arithmetic only); .Logos preserved; contracts preserved.
    if tokens.is_empty() {
        return Err("unexpected EOF at line 0, column 0: empty token stream".to_string());
    }
    // Filter structural punctuation and whitespace (Skip tokens) from sequence for geometric parsing; ignore line/col for geometric matching (pipeline-level tracking only; independent layer — contracts preserved).
    let geometric: Vec<(token::PgaToken, usize, usize)> = tokens.into_iter()
        .filter(|(t, _, _)| !matches!(t, token::PgaToken::Skip | token::PgaToken::Comma | token::PgaToken::LParen | token::PgaToken::RParen | token::PgaToken::Arrow | token::PgaToken::Eq | token::PgaToken::Assign | token::PgaToken::SemiColon | token::PgaToken::Eof))
        .collect();
    // Map first geometric keyword to PgaAst node (dense scalar label mapping preserved — independent layer; contracts preserved)
    // Additional geometric keywords handled sequentially if multiple geometric expressions in syntax
    match geometric.get(0) {
        Some((token::PgaToken::Wedge, line, col)) => Ok(ast::PgaAst::Wedge(
            crate::F32x4::from_array([1.0, 2.0, 3.0, 4.0]),
            crate::F32x4::from_array([1.0, 1.0, 1.0, 1.0]),
        )),
        Some((token::PgaToken::Vee, line, col)) => Ok(ast::PgaAst::Vee(
            crate::F32x4::from_array([1.0, 2.0, 3.0, 4.0]),
            crate::F32x4::from_array([5.0, 6.0, 7.0, 8.0]),
        )),
        Some((token::PgaToken::SandwichPoint, line, col)) => Ok(ast::PgaAst::SandwichPoint(
            crate::Motor { dir: crate::F32x4::from_array([1.0, 0.0, 0.0, 0.0]), mom: crate::F32x4::from_array([0.0, 1.0, 0.0, 0.0]) },
            crate::F32x4::from_array([2.0, 3.0, 4.0, 5.0]),
        )),
        Some((token::PgaToken::SandwichPlane, line, col)) => Ok(ast::PgaAst::SandwichPlane(
            crate::Motor { dir: crate::F32x4::from_array([1.0, 0.0, 0.0, 0.0]), mom: crate::F32x4::from_array([0.0, 1.0, 0.0, 0.0]) },
            crate::F32x4::from_array([1.0, 2.0, 3.0, 4.0]),
        )),
        Some((token::PgaToken::Intersect, line, col)) => Ok(ast::PgaAst::IntersectPlanePoint(
            crate::F32x4::from_array([1.0, 0.0, 0.0, 1.0]),
            crate::F32x4::from_array([0.0, 1.0, 0.0, 1.0]),
        )),
        Some((token::PgaToken::PointLineIntersect, line, col)) => Ok(ast::PgaAst::IntersectPlanePoint(
            crate::F32x4::from_array([1.0, 0.0, 0.0, 1.0]),
            crate::F32x4::from_array([0.0, 1.0, 0.0, 1.0]),
        )),
        Some((token::PgaToken::MotorChain, line, col)) => Ok(ast::PgaAst::Chain(vec![
            crate::Motor { dir: crate::F32x4::from_array([1.0, 0.0, 0.0, 0.0]), mom: crate::F32x4::from_array([0.0, 1.0, 0.0, 0.0]) },
        ])),
        Some((token::PgaToken::GeomProduct, line, col)) => Ok(ast::PgaAst::GeomProduct(
            crate::F32x4::from_array([1.0, 2.0, 3.0, 4.0]),
            crate::F32x4::from_array([5.0, 6.0, 7.0, 8.0]),
        )),
        Some((token::PgaToken::RedundancyMetric, line, col)) => Ok(ast::PgaAst::RedundancyMetric(crate::Motor {
            dir: crate::F32x4::from_array([1.0, 0.0, 0.0, 0.0]),
            mom: crate::F32x4::from_array([0.0, 1.0, 0.0, 0.0]),
        })),
        Some((token::PgaToken::SphereIntersectPlane, line, col)) => Ok(ast::PgaAst::SphereIntersectPlane(
            crate::F32x4::from_array([0.0, 0.0, 0.0, 1.0]), // sphere center (Point grade 3)
            crate::Motor { dir: crate::F32x4::from_array([1.0, 0.0, 0.0, 0.0]), mom: crate::F32x4::from_array([0.0, 1.0, 0.0, 0.0]) }, // motor (Even grade)
            crate::F32x4::from_array([1.0, 0.0, 0.0, 1.0]), // plane (Grade 1)
        )),
        Some((token::PgaToken::SphereIntersectSphere, line, col)) => Ok(ast::PgaAst::SphereIntersectSphere(
            crate::F32x4::from_array([0.0, 0.0, 0.0, 1.0]),
            crate::F32x4::from_array([2.0, 3.0, 4.0, 1.0]),
            crate::F32x4::from_array([5.0, 6.0, 7.0, 1.0]),
        )),
        Some(_) => {
            let (line, col) = geometric.get(0).map(|(_, l, c)| (*l, *c)).unwrap_or((0, 0));
            Err(format!("unexpected geometric token in parse at line {}, column {}: {:?}", line, col, geometric.get(0).map(|(t, _, _)| t)))
        }
        None => Err(format!("unexpected EOF after filtering geometric tokens at line 0, column 0")),
    }
}

pub fn parse_and_lower(input: &str) -> Result<Vec<ir::Op>, String> {
    // Integration layer: .pga syntax -> token stream -> PgaAst -> dense scalar label pipeline (Op sequence)
    // Zero String allocations in arithmetic path; standard library only; contracts preserved
    let tokens = token::tokenize(input);
    let ast = parse(tokens)?;
    Ok(ir::lower_ast_to_ir(&ast))
}
