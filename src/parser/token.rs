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

    // Skip whitespace between tokens (essential for branchless parsing flows)
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    Skip,
}
