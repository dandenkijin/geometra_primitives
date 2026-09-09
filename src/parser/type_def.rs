// PgaType: compile-time graded multivector classification for R_3_0_1
// Used by PgaIr to enforce valid geometric products (prevent cross-grade errors)

use crate::F32x4;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PgaType {
    Scalar,      // Grade 0
    Vector,      // Grade 1 (Plane coefficients: [a,b,c,d])
    Bivector,    // Grade 2 (Line / rotation axes / attention cross-terms: [f32; 6])
    Trivector,   // Grade 3 (Point homogeneous: [x,y,z,w])
    Pseudoscalar, // Grade 4 (metric scale / singularity: f32)
    Motor,       // Even mixed grade (rotation + translation)
}

// Zero-overhead grade mask tracker: which grades this IR node spans
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GradeMask(pub [bool; 5]);

impl GradeMask {
    pub fn scalar_only() -> Self { GradeMask([true, false, false, false, false]) }
    pub fn plane() -> Self { GradeMask([false, true, false, false, false]) }
    pub fn line() -> Self { GradeMask([false, false, true, false, false]) }
    pub fn point() -> Self { GradeMask([false, false, false, true, false]) }
    pub fn pseudoscalar() -> Self { GradeMask([false, false, false, false, true]) }
    pub fn motor() -> Self { Self::mixed_for_motor() }
    fn mixed_for_motor() -> Self {
        // Even grades present: Scalar (0) + Bivector (2) + Pseudoscalar (4)
        GradeMask([true, false, true, false, true])
    }
}
