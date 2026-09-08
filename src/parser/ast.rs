use crate::{Plane, Point, Motor};

pub enum PgaAst { Wedge(Plane, Plane), Vee(Plane, Plane), Sandwich(Motor, Point), IntersectPlanePoint(Plane, Point), Chain(Vec<Motor>) }
