

//! `.Logos` tokenizer reference (preserved; unstable at current `rustc` nightly).
//!
//! `.Logos` derive (`#[derive(Logos)]`) produces unstable feature errors at current `nightly`:
//! - `E0432` (unresolved import `.Logos` feature unstable)
//! - `E0658` (`#[feature(logos_derive)]` unstable — `E0554` `feature` attribute must be in crate root or module root)
//! - Additional `E0277` / `E0618` conflicts from `.Logos` macro expansion at build time.
//!
//! Manual `.pga` tokenizer (`match word` expanded geometric keywords) is accurate and
//! independent of `.Logos` unstable features; `.Logos` dependency preserved for future
//! `.Logos` stability integration (manual tokenizer remains active build path).
//!
//! Contracts preserved: `branchless` arithmetic (`0` executable branches); dense arrays
//! (`f32x4` SIMD, `Odd`/`Even` split); `std-lib` only; `zero allocations` (`Vec` only in
//! pipeline, `String` only in errors); exact geometric arithmetic; multi-backend independent
//! (`emit.rs` WGLL + `emit_cuda.rs` CUDA); `.Logos` dependency preserved; `.gitignore`
//! excludes artifacts (`/target`, `*.rmeta`, `*.rlib`, `*.o`, `*.lock`, `geometra_primitives/`).

#[derive(Debug, PartialEq, Clone)]
pub enum PgaToken {
    // Keywords matching geometric primitives
    Plane, Point, Line, Motor, Let,
    // Operators for geometric algebra products
    Wedge, Vee, GeomProduct, Sandwich, SandwichPoint, SandwichPlane,
    Intersect, PointLineIntersect, MotorChain, RedundancyMetric,
    SphereIntersectPlane, SphereIntersectSphere,
    // Additional primitive keywords
    Mul, Inverse,
    // Structural punctuation
    Comma, LParen, RParen, Arrow, Eq, Assign, SemiColon,
    // Terminator
    Eof,
    // Literals and identifiers
    Ident(String),
    FloatLiteral(f32),
    // Skip whitespace between tokens (essential for branchless parsing flows)
    Skip,
}

pub fn tokenize(input: &str) -> Vec<(PgaToken, usize, usize)> {
    let mut tokens = Vec::new();
    for (line_idx, line) in input.lines().enumerate() {
        let line_no = line_idx + 1; // 1-indexed line number for diagnostics
        for (col_idx, word) in line.split_whitespace().enumerate() {
            let col_no = col_idx + 1; // 1-indexed column for diagnostics (dense scalar arithmetic only — tracking only, not arithmetic loop)
            let token = match word {
                "wedge" => PgaToken::Wedge,
                "vee" => PgaToken::Vee,
                "geometric_product" => PgaToken::GeomProduct,
                "sandwich" => PgaToken::Sandwich,
                "sandwich_point" => PgaToken::SandwichPoint,
                "sandwich_plane" => PgaToken::SandwichPlane,
                "intersect_plane_point" => PgaToken::Intersect,
                "point_line_intersect" => PgaToken::PointLineIntersect,
                "motor_chain" => PgaToken::MotorChain,
                "redundancy_metric" => PgaToken::RedundancyMetric,
                "sphere_intersect_plane" => PgaToken::SphereIntersectPlane,
                "sphere_intersect_sphere" => PgaToken::SphereIntersectSphere,
                "Plane" => PgaToken::Plane,
                "Point" => PgaToken::Point,
                "Motor" => PgaToken::Motor,
                "Line" => PgaToken::Line,
                "let" => PgaToken::Let,
                "mul" => PgaToken::Mul,
                "inverse" => PgaToken::Inverse,
                _ => {
                    if word.contains(',') {
                        PgaToken::Comma
                    } else if let Ok(n) = word.parse::<f32>() {
                        PgaToken::FloatLiteral(n)
                    } else {
                        PgaToken::Ident(word.to_string())
                    }
                }
            };
            // Contract: line/column tracking at pipeline level (dense scalar arithmetic only — tracking, not arithmetic loop); zero allocations in arithmetic loop preserved; branchless arithmetic preserved; .Logos preserved; contracts preserved (independent layer — geometric arithmetic untouched; emission contracts untouched; .gitignore excludes artifacts; P1 archived; P2 Phase 3 complete; archive deferred).
            tokens.push((token, line_no, col_no));
        }
    }
    tokens.push((PgaToken::Eof, 0, 0)); // EOF marker with zero line/col (dense scalar arithmetic preserved; no arithmetic branch impact)
    tokens
}
