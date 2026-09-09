use crate::parser::ast::PgaAst;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Wedge, Vee, GeomProduct, Sandwich,
    Intersect, PointLineIntersect,
    MotorChain, RedundancyMetric,
    SphereIntersectPlane, SphereIntersectSphere,
    Plane, Point, Motor, Line,
    Num(f32),
    Comma, LParen, RParen, Arrow, Eq,
    Ident(String),
    Eof,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for line in input.lines() {
        for word in line.split_whitespace() {
            match word {
                "wedge" => tokens.push(Token::Wedge),
                "vee" => tokens.push(Token::Vee),
                "geometric_product" => tokens.push(Token::GeomProduct),
                "sandwich" => tokens.push(Token::Sandwich),
                "intersect_plane_point" => tokens.push(Token::Intersect),
                "point_line_intersect" => tokens.push(Token::PointLineIntersect),
                "motor_chain" => tokens.push(Token::MotorChain),
                "redundancy_metric" => tokens.push(Token::RedundancyMetric),
                "sphere_intersect_plane" => tokens.push(Token::SphereIntersectPlane),
                "sphere_intersect_sphere" => tokens.push(Token::SphereIntersectSphere),
                "Plane" => tokens.push(Token::Plane),
                "Point" => tokens.push(Token::Point),
                "Motor" => tokens.push(Token::Motor),
                "Line" => tokens.push(Token::Line),
                _ => {
                    if word.contains(',') {
                        tokens.push(Token::Comma);
                    } else if let Ok(n) = word.parse::<f32>() {
                        tokens.push(Token::Num(n));
                    } else {
                        tokens.push(Token::Ident(word.to_string()));
                    }
                }
            }
        }
    }
    tokens.push(Token::Eof);
    tokens
}
