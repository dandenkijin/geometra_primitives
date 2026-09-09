use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum PgaToken {
    // Keywords matching geometric primitives
    #[token("plane")]
    Plane,
    #[token("point")]
    Point,
    #[token("line")]
    Line,
    #[token("motor")]
    Motor,
    #[token("let")]
    Let,

    // Operators for geometric algebra products
    #[token("^")]
    Wedge,
    #[token("v", priority = 2)]
    Vee,
    #[token("*")]
    Mul,
    #[token("~")]
    Inverse,

    // Additional geometric primitive keywords
    #[token("geometric_product")]
    GeomProduct,
    #[token("sandwich")]
    Sandwich,
    #[token("intersect_plane_point")]
    Intersect,
    #[token("point_line_intersect")]
    PointLineIntersect,
    #[token("motor_chain")]
    MotorChain,
    #[token("redundancy_metric")]
    RedundancyMetric,
    #[token("sphere_intersect_plane")]
    SphereIntersectPlane,
    #[token("sphere_intersect_sphere")]
    SphereIntersectSphere,

    // Identifiers and numeric literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", priority = 1)]
    Ident,

    #[regex(r"-?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?")]
    FloatLiteral,

    // Structural punctuation
    #[token("=")]
    Assign,
    #[token(";")]
    SemiColon,
    #[token(",")]
    Comma,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("=>")]
    Arrow,
    #[token("==")]
    Eq,
    #[token("eof")]
    Eof,

    // Skip whitespace between tokens (essential for branchless parsing flows)
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    Skip,
}

pub fn tokenize(input: &str) -> Vec<PgaToken> {
    // Use logos::Logos iterator; filter whitespace skips, collect valid tokens
    PgaToken::lexer(input)
        .filter_map(|res| res.ok())
        .filter(|tok| !matches!(tok, PgaToken::Skip))
        .collect()
}
