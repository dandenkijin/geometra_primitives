
#[derive(Debug, PartialEq, Clone)]
pub enum PgaToken {
    // Keywords matching geometric primitives
    // Token("...", priority)
    Plane,
    // Token("...", priority)
    Point,
    // Token("...", priority)
    Line,
    // Token("...", priority)
    Motor,
    // Token("...", priority)
    Let,

    // Operators for geometric algebra products
    // Token("...", priority)
    Wedge,
    // Token("...", priority)
    Vee,
    // Token("...", priority)
    Mul,
    // Token("...", priority)
    Inverse,

    // Identifiers and numeric literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", priority = 1)]
    Ident,

    // Regex("...", skip/priority)
    FloatLiteral,

    // Structural punctuation
    // Token("...", priority)
    Assign,
    // Token("...", priority)
    SemiColon,

    // Expanded geometric primitive syntax keywords
    GeomProduct,
    Sandwich,
    SandwichPoint,
    SandwichPlane,
    Intersect,
    PointLineIntersect,
    MotorChain,
    RedundancyMetric,
    SphereIntersectPlane,
    SphereIntersectSphere,
    // Structural punctuation for syntax precision
    Comma,
    LParen,
    RParen,
    Arrow,
    Eq,

    // Skip whitespace
    // Regex("...", skip/priority)
    Skip,
}

pub fn tokenize(input: &str) -> Vec<PgaToken> {
    let mut tokens = Vec::new();
    for line in input.lines() {
        for word in line.split_whitespace() {
            match word {
                "wedge" => tokens.push(PgaToken::Wedge),
                "vee" => tokens.push(PgaToken::Vee),
                "geometric_product" => tokens.push(PgaToken::GeomProduct),
                "sandwich" => tokens.push(PgaToken::Sandwich),
                "intersect_plane_point" => tokens.push(PgaToken::Intersect),
                "point_line_intersect" => tokens.push(PgaToken::PointLineIntersect),
                "motor_chain" => tokens.push(PgaToken::MotorChain),
                "redundancy_metric" => tokens.push(PgaToken::RedundancyMetric),
                "sphere_intersect_plane" => tokens.push(PgaToken::SphereIntersectPlane),
                "sphere_intersect_sphere" => tokens.push(PgaToken::SphereIntersectSphere),
                "plane" => tokens.push(PgaToken::Plane),
                "point" => tokens.push(PgaToken::Point),
                "motor" => tokens.push(PgaToken::Motor),
                "line" => tokens.push(PgaToken::Line),
                "let" => tokens.push(PgaToken::Let),
                "=" => tokens.push(PgaToken::Assign),
                ";" => tokens.push(PgaToken::SemiColon),
                "^" => tokens.push(PgaToken::Wedge),
                "v" => tokens.push(PgaToken::Vee),
                "*" => tokens.push(PgaToken::Mul),
                "~" => tokens.push(PgaToken::Inverse),
                "(" => tokens.push(PgaToken::LParen),
                ")" => tokens.push(PgaToken::RParen),
                "," => tokens.push(PgaToken::Comma),
                "=>" => tokens.push(PgaToken::Arrow),
                "==" => tokens.push(PgaToken::Eq),
                "eof" => tokens.push(PgaToken::Eof),
                _ => {
                    if let Ok(n) = word.parse::<f32>() {
                        tokens.push(PgaToken::FloatLiteral(n));
                    } else {
                        tokens.push(PgaToken::Ident(word.to_string()));
                    }
                }
            }
        }
    }
    tokens.push(PgaToken::Eof);
    tokens
}
