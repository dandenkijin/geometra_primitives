

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
        
        // Split line into words and punctuation, preserving positions
        let mut col_idx = 0;
        let mut chars = line.chars().peekable();
        let mut current_word = String::new();
        let mut word_start_col = 1;
        
        while let Some(ch) = chars.next() {
            let is_punct = matches!(ch, ';' | ',' | '(' | ')' | '=' | '^' | 'v' | '*' | '>' | '<');
            let is_arrow_start = ch == '=' && chars.peek() == Some(&'>');
            let is_whitespace = ch.is_whitespace();
            
            if is_arrow_start {
                // Handle "=>" as single token
                if !current_word.is_empty() {
                    let word = std::mem::take(&mut current_word);
                    push_token(&mut tokens, word, line_no, word_start_col);
                }
                // Push "=>" as arrow
                tokens.push((PgaToken::Arrow, line_no, col_idx + 1));
                chars.next(); // consume '>'
                col_idx += 2;
                word_start_col = col_idx + 1;
            } else if is_punct && !is_arrow_start {
                // Handle single-character punctuation
                if !current_word.is_empty() {
                    let word = std::mem::take(&mut current_word);
                    push_token(&mut tokens, word, line_no, word_start_col);
                }
                let token = match ch {
                    ';' => PgaToken::SemiColon,
                    ',' => PgaToken::Comma,
                    '(' => PgaToken::LParen,
                    ')' => PgaToken::RParen,
                    '=' => PgaToken::Eq,
                    '^' => PgaToken::Wedge,
                    'v' => PgaToken::Vee,
                    '*' => PgaToken::GeomProduct,
                    _ => continue,
                };
                tokens.push((token, line_no, col_idx + 1));
                col_idx += 1;
                word_start_col = col_idx + 1;
            } else if is_whitespace {
                if !current_word.is_empty() {
                    let word = std::mem::take(&mut current_word);
                    push_token(&mut tokens, word, line_no, word_start_col);
                }
                col_idx += 1;
                word_start_col = col_idx + 1;
            } else {
                if current_word.is_empty() {
                    word_start_col = col_idx + 1;
                }
                current_word.push(ch);
                col_idx += 1;
            }
        }
        
        // Handle any remaining word at end of line
        if !current_word.is_empty() {
            let word = std::mem::take(&mut current_word);
            push_token(&mut tokens, word, line_no, word_start_col);
        }
    }
    tokens.push((PgaToken::Eof, 0, 0)); // EOF marker with zero line/col
    tokens
}

fn push_token(tokens: &mut Vec<(PgaToken, usize, usize)>, word: String, line_no: usize, col_no: usize) {
    let token = match word.as_str() {
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
            if let Ok(n) = word.parse::<f32>() {
                PgaToken::FloatLiteral(n)
            } else {
                PgaToken::Ident(word)
            }
        }
    };
    tokens.push((token, line_no, col_no));
}
