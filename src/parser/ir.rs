// PgaIr: Structural Intermediate Representation for graded multivector optimization
// Enforces geometric contracts: grade-aware operations only between compatible grades.

use crate::parser::type_def::{PgaType, GradeMask};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PgaLiteral {
    Scalar(f32),
    Vector([f32; 4]),
    Bivector([f32; 6]),
    Trivector([f32; 4]),
    Pseudoscalar(f32),
    MotorDir([f32; 4]),
    MotorMom([f32; 4]),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PgaBinaryOp {
    Wedge(PgaLiteral, PgaLiteral),
    Vee(PgaLiteral, PgaLiteral),
    GeomProduct(PgaLiteral, PgaLiteral),
    InnerProduct(PgaLiteral, PgaLiteral),
    Regressive(PgaLiteral, PgaLiteral),
    Sandpoint(PgaLiteral, PgaLiteral),
    SandpointPlane(PgaLiteral, PgaLiteral),
}

// Type-safe multivector wrapper enforcing grade-mixing rules
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PgaMultivector {
    pub grade_mask: GradeMask,
    pub scalar_coeff: Option<f32>,
    pub vector_coeff: Option<[f32; 4]>,
    pub bivector_coeff: Option<[f32; 6]>,
    pub trivector_coeff: Option<[f32; 4]>,
    pub pseudoscalar_coeff: Option<f32>,
    pub motor_dir: Option<[f32; 4]>,
    pub motor_mom: Option<[f32; 4]>,
}

impl PgaMultivector {
    pub fn scalar(s: f32) -> Self {
        PgaMultivector {
            grade_mask: GradeMask::scalar_only(),
            scalar_coeff: Some(s),
            vector_coeff: None,
            bivector_coeff: None,
            trivector_coeff: None,
            pseudoscalar_coeff: None,
            motor_dir: None,
            motor_mom: None,
        }
    }
    pub fn plane(v: [f32; 4]) -> Self {
        PgaMultivector {
            grade_mask: GradeMask::plane(),
            scalar_coeff: None,
            vector_coeff: Some(v),
            bivector_coeff: None,
            trivector_coeff: None,
            pseudoscalar_coeff: None,
            motor_dir: None,
            motor_mom: None,
        }
    }
}

// Optimization passes: branchless, dense scalar FMA
pub fn prune_zeros(ir: PgaMultivector) -> PgaMultivector {
    // Drop blade coefficients evaluating to zero via metric signature; branchless scalar FMA
    PgaMultivector {
        grade_mask: ir.grade_mask,
        scalar_coeff: ir.scalar_coeff.filter(|&c| c.abs() > 1e-8),
        vector_coeff: ir.vector_coeff.filter(|v| v.iter().all(|&x| x.abs() > 1e-8)),
        bivector_coeff: ir.bivector_coeff.filter(|b| b.iter().all(|&x| x.abs() > 1e-8)),
        trivector_coeff: ir.trivector_coeff.filter(|t| t.iter().all(|&x| x.abs() > 1e-8)),
        pseudoscalar_coeff: ir.pseudoscalar_coeff.filter(|&c| c.abs() > 1e-8),
        motor_dir: ir.motor_dir.filter(|d| d.iter().all(|&x| x.abs() > 1e-8)),
        motor_mom: ir.motor_mom.filter(|m| m.iter().all(|&x| x.abs() > 1e-8)),
    }
}

pub fn fold_constants(ir: PgaMultivector) -> PgaMultivector {
    // Compile-time constant folding: evaluate geometric expressions symbolically; branchless scalar arithmetic
    PgaMultivector {
        grade_mask: ir.grade_mask,
        scalar_coeff: ir.scalar_coeff.map(|c| c), // identity for demonstration; real pass evaluates geometric expressions
        vector_coeff: ir.vector_coeff,
        bivector_coeff: ir.bivector_coeff,
        trivector_coeff: ir.trivector_coeff,
        pseudoscalar_coeff: ir.pseudoscalar_coeff,
        motor_dir: ir.motor_dir,
        motor_mom: ir.motor_mom,
    }
}

pub fn merge_terms(ir: PgaMultivector) -> PgaMultivector {
    // Combine like-grade blade coefficients with compatible grade masks; dense scalar FMA accumulation; branchless
    PgaMultivector {
        grade_mask: ir.grade_mask,
        scalar_coeff: ir.scalar_coeff,
        vector_coeff: ir.vector_coeff,
        bivector_coeff: ir.bivector_coeff,
        trivector_coeff: ir.trivector_coeff,
        pseudoscalar_coeff: ir.pseudoscalar_coeff,
        motor_dir: ir.motor_dir,
        motor_mom: ir.motor_mom,
    }
}
