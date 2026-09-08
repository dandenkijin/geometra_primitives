use crate::{Plane, Point, Motor};

pub enum PgaAst {
    Wedge(Plane, Plane),
    Vee(Point, Point),
    SandwichPoint(Motor, Point),
    SandwichPlane(Motor, Plane),
    IntersectPlanePoint(Plane, Point),
    Chain(Vec<Motor>),
}
