use crate::{Plane, Point, Motor, Line};

#[derive(Debug, Clone, PartialEq)]
pub enum PgaAst {
    Wedge(Plane, Plane),
    Vee(Point, Point),
    SandwichPoint(Motor, Point),
    SandwichPlane(Motor, Plane),
    IntersectPlanePoint(Plane, Point),
    Chain(Vec<Motor>),
    Rotor(Motor),
    GeomProduct(Plane, Point),
    PointLineIntersect(Point, Line),
    MotorChain(Vec<Motor>),
    RedundancyMetric(Motor),
    SphereIntersectPlane(Point, f32, Plane),
    SphereIntersectSphere(Point, f32, Point, f32),
    Projection(Plane, Plane),
    Rejection(Plane, Plane),
    Pseudoscalar(f32),
    Line(Line),
}