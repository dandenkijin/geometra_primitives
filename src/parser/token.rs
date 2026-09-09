

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

pub fn tokenize(input: &str) -> Vec<PgaToken> {
    let mut tokens = Vec::new();
    for line in input.lines() {
        for word in line.split_whitespace() {
            match word {
                "wedge" => tokens.push(PgaToken::Wedge),
                "vee" => tokens.push(PgaToken::Vee),
                "geometric_product" => tokens.push(PgaToken::GeomProduct),
                "sandwich" => tokens.push(PgaToken::Sandwich),
                "sandwich_point" => tokens.push(PgaToken::SandwichPoint),
                "sandwich_plane" => tokens.push(PgaToken::SandwichPlane),
                "intersect_plane_point" => tokens.push(PgaToken::Intersect),
                "point_line_intersect" => tokens.push(PgaToken::PointLineIntersect),
                "motor_chain" => tokens.push(PgaToken::MotorChain),
                "redundancy_metric" => tokens.push(PgaToken::RedundancyMetric),
                "sphere_intersect_plane" => tokens.push(PgaToken::SphereIntersectPlane),
                "sphere_intersect_sphere" => tokens.push(PgaToken::SphereIntersectSphere),
                "Plane" => tokens.push(PgaToken::Plane),
                "Point" => tokens.push(PgaToken::Point),
                "Motor" => tokens.push(PgaToken::Motor),
                "Line" => tokens.push(PgaToken::Line),
                _ => {
                    if word.contains(',') {
                        tokens.push(PgaToken::Comma);
                    } else if let Ok(n) = word.parse::<f32>() {
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
