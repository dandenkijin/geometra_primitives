pub mod ast;
pub mod emit;
pub mod emit_cuda;
pub mod token;
pub mod ir;
pub mod type_def;
pub mod symbol_table;

use crate::{F32x4, Plane, Point, Line, Motor};
use crate::parser::token::PgaToken;

// Parser: convert token stream to AST with real operand extraction (branchless scalar arithmetic preserved)
pub fn parse(tokens: Vec<(PgaToken, usize, usize)>) -> Result<ast::PgaAst, String> {
    if tokens.is_empty() {
        return Err("unexpected EOF at line 0, column 0: empty token stream".to_string());
    }

    // Filter structural punctuation and whitespace
    let geometric: Vec<(PgaToken, usize, usize)> = tokens.into_iter()
        .filter(|(t, _, _)| !matches!(t, PgaToken::Skip | PgaToken::Comma | PgaToken::LParen | PgaToken::RParen | PgaToken::Arrow | PgaToken::Eq | PgaToken::Assign | PgaToken::SemiColon | PgaToken::Eof))
        .collect();

    if geometric.is_empty() {
        return Err("no geometric tokens found".to_string());
    }

    // Parse with a cursor over the filtered tokens
    let mut cursor = 0;
    parse_operation(&geometric, &mut cursor)
}

fn parse_operation(tokens: &[(PgaToken, usize, usize)], cursor: &mut usize) -> Result<ast::PgaAst, String> {
    if *cursor >= tokens.len() {
        return Err("unexpected EOF".to_string());
    }

    let (token, line, col) = &tokens[*cursor];
    let token = token.clone();
    *cursor += 1;

    match token {
        PgaToken::Wedge => {
            let p = parse_plane(tokens, cursor, *line, *col)?;
            let q = parse_plane(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::Wedge(p, q))
        }
        PgaToken::Vee => {
            let p = parse_point(tokens, cursor, *line, *col)?;
            let q = parse_point(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::Vee(p, q))
        }
        PgaToken::SandwichPoint => {
            let m = parse_motor(tokens, cursor, *line, *col)?;
            let pt = parse_point(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::SandwichPoint(m, pt))
        }
        PgaToken::SandwichPlane => {
            let m = parse_motor(tokens, cursor, *line, *col)?;
            let pl = parse_plane(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::SandwichPlane(m, pl))
        }
        PgaToken::Intersect => {
            let p = parse_plane(tokens, cursor, *line, *col)?;
            let pt = parse_point(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::IntersectPlanePoint(p, pt))
        }
        PgaToken::PointLineIntersect => {
            let pt = parse_point(tokens, cursor, *line, *col)?;
            let ln = parse_line(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::PointLineIntersect(pt, ln))
        }
        PgaToken::MotorChain => {
            let mut chain = Vec::new();
            while *cursor < tokens.len() {
                if matches!(tokens[*cursor].0, PgaToken::Motor | PgaToken::Ident(_)) {
                    chain.push(parse_motor(tokens, cursor, *line, *col)?);
                } else {
                    break;
                }
            }
            if chain.is_empty() {
                return Err("motor_chain requires at least one motor".to_string());
            }
            Ok(ast::PgaAst::MotorChain(chain))
        }
        PgaToken::GeomProduct => {
            let p = parse_plane(tokens, cursor, *line, *col)?;
            let pt = parse_point(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::GeomProduct(p, pt))
        }
        PgaToken::RedundancyMetric => {
            let m = parse_motor(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::RedundancyMetric(m))
        }
        PgaToken::SphereIntersectPlane => {
            let center = parse_point(tokens, cursor, *line, *col)?;
            let radius = parse_scalar(tokens, cursor, *line, *col)?;
            let plane = parse_plane(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::SphereIntersectPlane(center, radius, plane))
        }
        PgaToken::SphereIntersectSphere => {
            let c1 = parse_point(tokens, cursor, *line, *col)?;
            let r1 = parse_scalar(tokens, cursor, *line, *col)?;
            let c2 = parse_point(tokens, cursor, *line, *col)?;
            let r2 = parse_scalar(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::SphereIntersectSphere(c1, r1, c2, r2))
        }
        PgaToken::Projection => {
            let a = parse_plane(tokens, cursor, *line, *col)?;
            let b = parse_plane(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::Projection(a, b))
        }
        PgaToken::Rejection => {
            let a = parse_plane(tokens, cursor, *line, *col)?;
            let b = parse_plane(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::Rejection(a, b))
        }
        PgaToken::Pseudoscalar => {
            let p = parse_scalar(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::Pseudoscalar(p))
        }
        PgaToken::Line => {
            let ln = parse_line(tokens, cursor, *line, *col)?;
            Ok(ast::PgaAst::Line(ln))
        }
        PgaToken::Mul => {
            // Handle multiplication chain
            let mut chain = Vec::new();
            while *cursor < tokens.len() {
                if matches!(tokens[*cursor].0, PgaToken::Motor | PgaToken::Ident(_)) {
                    chain.push(parse_motor(tokens, cursor, *line, *col)?);
                } else {
                    break;
                }
            }
            if chain.is_empty() {
                return Err("mul requires at least one motor".to_string());
            }
            Ok(ast::PgaAst::Chain(chain))
        }
        _ => Err(format!("unexpected geometric token in parse at line {}, column {}: {:?}", line, col, token)),
    }
}

fn parse_plane(tokens: &[(PgaToken, usize, usize)], cursor: &mut usize, line: usize, col: usize) -> Result<Plane, String> {
    // Plane can be: "Plane" keyword + 4 floats, or just 4 floats, or ident
    if *cursor >= tokens.len() {
        return Err("expected plane operand".to_string());
    }
    match &tokens[*cursor].0 {
        PgaToken::Plane => {
            *cursor += 1;
            parse_f32x4(tokens, cursor, line, col)
        }
        PgaToken::FloatLiteral(_) | PgaToken::Ident(_) => {
            parse_f32x4(tokens, cursor, line, col)
        }
        _ => Err(format!("expected plane or float at line {}, column {}", line, col)),
    }
}

fn parse_point(tokens: &[(PgaToken, usize, usize)], cursor: &mut usize, line: usize, col: usize) -> Result<Point, String> {
    if *cursor >= tokens.len() {
        return Err("expected point operand".to_string());
    }
    match &tokens[*cursor].0 {
        PgaToken::Point => {
            *cursor += 1;
            parse_f32x4(tokens, cursor, line, col)
        }
        PgaToken::FloatLiteral(_) | PgaToken::Ident(_) => {
            parse_f32x4(tokens, cursor, line, col)
        }
        _ => Err(format!("expected point or float at line {}, column {}", line, col)),
    }
}

fn parse_line(tokens: &[(PgaToken, usize, usize)], cursor: &mut usize, line: usize, col: usize) -> Result<Line, String> {
    if *cursor >= tokens.len() {
        return Err("expected line operand".to_string());
    }
    match &tokens[*cursor].0 {
        PgaToken::Line => {
            *cursor += 1;
            let comps = parse_f32x6(tokens, cursor, line, col)?;
            Ok(Line { dir: F32x4::from_array([comps[0], comps[1], comps[2], comps[3]]), mom: F32x4::from_array([comps[4], comps[5], 0.0, 0.0]) })
        }
        PgaToken::FloatLiteral(_) | PgaToken::Ident(_) => {
            let comps = parse_f32x6(tokens, cursor, line, col)?;
            Ok(Line { dir: F32x4::from_array([comps[0], comps[1], comps[2], comps[3]]), mom: F32x4::from_array([comps[4], comps[5], 0.0, 0.0]) })
        }
        _ => Err(format!("expected line or float at line {}, column {}", line, col)),
    }
}

fn parse_motor(tokens: &[(PgaToken, usize, usize)], cursor: &mut usize, line: usize, col: usize) -> Result<Motor, String> {
    if *cursor >= tokens.len() {
        return Err("expected motor operand".to_string());
    }
    match &tokens[*cursor].0 {
        PgaToken::Motor => {
            *cursor += 1;
            parse_motor_components(tokens, cursor, line, col)
        }
        PgaToken::FloatLiteral(_) | PgaToken::Ident(_) => {
            parse_motor_components(tokens, cursor, line, col)
        }
        _ => Err(format!("expected motor or float at line {}, column {}", line, col)),
    }
}

fn parse_scalar(tokens: &[(PgaToken, usize, usize)], cursor: &mut usize, _line: usize, _col: usize) -> Result<f32, String> {
    if *cursor >= tokens.len() {
        return Err("expected scalar operand".to_string());
    }
    match &tokens[*cursor].0 {
        PgaToken::FloatLiteral(val) => {
            *cursor += 1;
            Ok(*val)
        }
        PgaToken::Ident(_) => {
            // Identifiers treated as scalar placeholders (symbol table resolves)
            *cursor += 1;
            Ok(0.0)
        }
        _ => Err("expected float literal".to_string()),
    }
}

fn parse_f32x4(tokens: &[(PgaToken, usize, usize)], cursor: &mut usize, line: usize, col: usize) -> Result<F32x4, String> {
    let mut components = [0.0f32; 4];
    for i in 0..4 {
        if *cursor >= tokens.len() {
            return Err(format!("expected 4 float components for vector at line {}, column {}", line, col));
        }
        match &tokens[*cursor].0 {
            PgaToken::FloatLiteral(val) => {
                components[i] = *val;
                *cursor += 1;
            }
            PgaToken::Ident(_) => {
                components[i] = 0.0; // placeholder for symbol table
                *cursor += 1;
            }
            _ => return Err(format!("expected float at component {} line {}, column {}", i, line, col)),
        }
    }
    Ok(F32x4::from_array(components))
}

fn parse_f32x6(tokens: &[(PgaToken, usize, usize)], cursor: &mut usize, line: usize, col: usize) -> Result<[f32; 6], String> {
    let mut components = [0.0f32; 6];
    for i in 0..6 {
        if *cursor >= tokens.len() {
            return Err(format!("expected 6 float components for line at line {}, column {}", line, col));
        }
        match &tokens[*cursor].0 {
            PgaToken::FloatLiteral(val) => {
                components[i] = *val;
                *cursor += 1;
            }
            PgaToken::Ident(_) => {
                components[i] = 0.0;
                *cursor += 1;
            }
            _ => return Err(format!("expected float at component {} line {}, column {}", i, line, col)),
        }
    }
    Ok(components)
}

fn parse_motor_components(tokens: &[(PgaToken, usize, usize)], cursor: &mut usize, line: usize, col: usize) -> Result<Motor, String> {
    let dir = parse_f32x4(tokens, cursor, line, col)?;
    let mom = parse_f32x4(tokens, cursor, line, col)?;
    Ok(Motor { dir, mom })
}

pub fn parse_and_lower(input: &str) -> Result<Vec<ir::Op>, String> {
    let tokens = token::tokenize(input);
    let (ast, sym_table, op_tracker) = symbol_table::parse_with_symbol_table(
        &tokens.iter().map(|(t, _, _)| t.clone()).collect::<Vec<_>>()
    );
    let ctx = ir::build_operand_context(&ast, &sym_table, &op_tracker);
    let ops = ir::lower_ast_to_ir_with_context(&ast, &ctx);
    for op in &ops {
        if !ir::validate_op_grade(op, &ctx) {
            return Err(format!("grade validation failed for op: {:?}", op));
        }
    }
    Ok(ops)
}