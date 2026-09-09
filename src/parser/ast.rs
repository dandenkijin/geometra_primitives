use crate::{Plane, Point, Motor};

pub enum PgaAst {
    Wedge(Plane, Plane),
    Vee(Point, Point),
    SandwichPoint(Motor, Point),
    SandwichPlane(Motor, Plane),
    IntersectPlanePoint(Plane, Point),
    Chain(Vec<Motor>),
    Rotor(Motor),
    Projection(Plane, Plane),
    Rejection(Plane, Plane),
    Pseudoscalar(f32),
}
