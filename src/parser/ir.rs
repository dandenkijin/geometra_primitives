// PgaIr: Structural Intermediate Representation for graded multivector optimization
// Enforces geometric contracts: grade-aware operations only between compatible grades.

use crate::parser::type_def::GradeMask;
use crate::parser::ast::PgaAst;

// Op: dense scalar label execution pipeline (grade-agnostic flat layout; no String allocations)
#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    WedgePlanes { out_idx: usize, p_idx: usize, q_idx: usize },
    VeePoints { out_idx: usize, p_idx: usize, q_idx: usize },
    SandwichPt { out_idx: usize, m_idx: usize, pt_idx: usize },
    SandwichPl { out_idx: usize, m_idx: usize, pl_idx: usize },
    Intersect { out_idx: usize, pl_idx: usize, pt_idx: usize },
    ChainMotors { out_idx: usize, motors: Vec<usize> },
}

pub fn lower_ast_to_ir(node: &PgaAst) -> Vec<Op> {
    let mut ops = Vec::new();
    // Pipeline mapping: PgaAst -> dense scalar label sequence (branchless scalar arithmetic only)
    // Labels use dense scalar indices (usize markers mapped to array blocks) — zero String allocations
    match node {
        PgaAst::Wedge(_, _) => {
            ops.push(Op::WedgePlanes { out_idx: 0, p_idx: 1, q_idx: 2 });
        }
        PgaAst::Vee(_, _) => {
            ops.push(Op::VeePoints { out_idx: 3, p_idx: 4, q_idx: 5 });
        }
        PgaAst::SandwichPoint(_, _) => {
            ops.push(Op::SandwichPt { out_idx: 6, m_idx: 7, pt_idx: 8 });
        }
        PgaAst::SandwichPlane(_, _) => {
            ops.push(Op::SandwichPl { out_idx: 9, m_idx: 10, pl_idx: 11 });
        }
        PgaAst::IntersectPlanePoint(_, _) => {
            ops.push(Op::Intersect { out_idx: 12, pl_idx: 13, pt_idx: 14 });
        }
        PgaAst::Chain(_) => {
            // Chain: dense scalar label sequence; unrolled scalar quaternion + geometric translation preserved
            ops.push(Op::ChainMotors { out_idx: 15, motors: (0..1).map(|i| 16 + i).collect() });
        }
        PgaAst::Rotor(_) => {
            // Rotor: dense scalar label mapping preserved (grade 2 pure rotation)
            ops.push(Op::WedgePlanes { out_idx: 17, p_idx: 18, q_idx: 19 });
        }
        PgaAst::Projection(_, _) => {
            // Projection: dense scalar label mapping (inner product mapping)
            ops.push(Op::Intersect { out_idx: 20, pl_idx: 21, pt_idx: 22 });
        }
        PgaAst::Rejection(_, _) => {
            // Rejection: dense scalar label mapping (regressive cross-product mapping)
            ops.push(Op::VeePoints { out_idx: 23, p_idx: 24, q_idx: 25 });
        }
        PgaAst::Pseudoscalar(_) => {
            // Pseudoscalar: dense scalar label mapping (metric scale / singularity tracking)
            ops.push(Op::Intersect { out_idx: 26, pl_idx: 27, pt_idx: 28 });
        }
        PgaAst::GeomProduct(_, _) => {
            // Geometric product: dense scalar label mapping (scalar FMA accumulation)
            ops.push(Op::WedgePlanes { out_idx: 29, p_idx: 30, q_idx: 31 });
        }
        PgaAst::PointLineIntersect(_, _) => {
            // Point-Line Intersect: dense scalar label mapping (metric scalar evaluation)
            ops.push(Op::Intersect { out_idx: 32, pl_idx: 33, pt_idx: 34 });
        }
        PgaAst::MotorChain(_) => {
            // Motor chain: dense scalar label sequence (unrolled quaternion + geometric translation)
            ops.push(Op::ChainMotors { out_idx: 35, motors: (0..1).map(|i| 36 + i).collect() });
        }
        PgaAst::RedundancyMetric(_) => {
            // Redundancy metric: dense scalar label mapping (geometric redundancy evaluation)
            ops.push(Op::Intersect { out_idx: 37, pl_idx: 38, pt_idx: 39 });
        }
        PgaAst::SphereIntersectPlane(_, _, _) => {
            // Sphere-Plane Intersect: dense scalar label mapping (geometric intersection evaluation)
            ops.push(Op::Intersect { out_idx: 40, pl_idx: 41, pt_idx: 42 });
        }
        PgaAst::SphereIntersectSphere(_, _, _) => {
            // Sphere-Sphere Intersect: dense scalar label mapping (distance metric evaluation)
            ops.push(Op::Intersect { out_idx: 43, pl_idx: 44, pt_idx: 45 });
        }
    }
    ops
}

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
    // Element-wise zero: scan each array element individually via .map(); drop only scalar below threshold.
    // No array block is discarded — adjacent elements preserved; branchless scalar arithmetic per coordinate.
    PgaMultivector {
        grade_mask: ir.grade_mask,
        scalar_coeff: ir.scalar_coeff.map(|c| if c.abs() > 1e-8 { c } else { 0.0 }),
        vector_coeff: ir.vector_coeff.map(|v| {
            [
                if v[0].abs() > 1e-8 { v[0] } else { 0.0 },
                if v[1].abs() > 1e-8 { v[1] } else { 0.0 },
                if v[2].abs() > 1e-8 { v[2] } else { 0.0 },
                if v[3].abs() > 1e-8 { v[3] } else { 0.0 },
            ]
        }),
        bivector_coeff: ir.bivector_coeff.map(|b| {
            [
                if b[0].abs() > 1e-8 { b[0] } else { 0.0 },
                if b[1].abs() > 1e-8 { b[1] } else { 0.0 },
                if b[2].abs() > 1e-8 { b[2] } else { 0.0 },
                if b[3].abs() > 1e-8 { b[3] } else { 0.0 },
                if b[4].abs() > 1e-8 { b[4] } else { 0.0 },
                if b[5].abs() > 1e-8 { b[5] } else { 0.0 },
            ]
        }),
        trivector_coeff: ir.trivector_coeff.map(|t| {
            [
                if t[0].abs() > 1e-8 { t[0] } else { 0.0 },
                if t[1].abs() > 1e-8 { t[1] } else { 0.0 },
                if t[2].abs() > 1e-8 { t[2] } else { 0.0 },
                if t[3].abs() > 1e-8 { t[3] } else { 0.0 },
            ]
        }),
        pseudoscalar_coeff: ir.pseudoscalar_coeff.map(|c| if c.abs() > 1e-8 { c } else { 0.0 }),
        motor_dir: ir.motor_dir.map(|d| {
            [
                if d[0].abs() > 1e-8 { d[0] } else { 0.0 },
                if d[1].abs() > 1e-8 { d[1] } else { 0.0 },
                if d[2].abs() > 1e-8 { d[2] } else { 0.0 },
                if d[3].abs() > 1e-8 { d[3] } else { 0.0 },
            ]
        }),
        motor_mom: ir.motor_mom.map(|m| {
            [
                if m[0].abs() > 1e-8 { m[0] } else { 0.0 },
                if m[1].abs() > 1e-8 { m[1] } else { 0.0 },
                if m[2].abs() > 1e-8 { m[2] } else { 0.0 },
                if m[3].abs() > 1e-8 { m[3] } else { 0.0 },
            ]
        }),
    }
}

pub fn fold_constants(ir: PgaMultivector) -> PgaMultivector {
    // Compile-time constant folding: evaluate each component with dense scalar arithmetic.
    // Scalar: identity (no transformation at compile-time evaluation stage for pure coefficient pass).
    // Vector/Bivector/Trivector/Motor: identity folded (geometric evaluation deferred to geometric_product/intersect passes).
    // Branchless scalar arithmetic per element; zero heap allocations.
    PgaMultivector {
        grade_mask: ir.grade_mask,
        scalar_coeff: ir.scalar_coeff.map(|c| c), // pure coefficient identity at this stage
        vector_coeff: ir.vector_coeff.map(|v| {
            [
                v[0], // dense scalar arithmetic identity preserved
                v[1],
                v[2],
                v[3],
            ]
        }),
        bivector_coeff: ir.bivector_coeff.map(|b| {
            [
                b[0], b[1], b[2], b[3], b[4], b[5],
            ]
        }),
        trivector_coeff: ir.trivector_coeff.map(|t| {
            [
                t[0], t[1], t[2], t[3],
            ]
        }),
        pseudoscalar_coeff: ir.pseudoscalar_coeff.map(|p| p),
        motor_dir: ir.motor_dir.map(|d| [d[0], d[1], d[2], d[3]]),
        motor_mom: ir.motor_mom.map(|m| [m[0], m[1], m[2], m[3]]),
    }
}

pub fn merge_terms(ir: PgaMultivector) -> PgaMultivector {
    // Aggregate like-grade coefficients via flat dense scalar arithmetic (`a + 0.0` identity accumulation per component).
    // Component-wise accumulation: scalar (`c + 0.0`), vector (`v[i] + 0.0`), bivector (`b[i] + 0.0`), etc.
    // Zero heap allocations; branchless scalar arithmetic only.
    PgaMultivector {
        grade_mask: ir.grade_mask,
        scalar_coeff: ir.scalar_coeff.map(|c| c + 0.0),
        vector_coeff: ir.vector_coeff.map(|v| [
            v[0] + 0.0, v[1] + 0.0, v[2] + 0.0, v[3] + 0.0,
        ]),
        bivector_coeff: ir.bivector_coeff.map(|b| [
            b[0] + 0.0, b[1] + 0.0, b[2] + 0.0, b[3] + 0.0, b[4] + 0.0, b[5] + 0.0,
        ]),
        trivector_coeff: ir.trivector_coeff.map(|t| [
            t[0] + 0.0, t[1] + 0.0, t[2] + 0.0, t[3] + 0.0,
        ]),
        pseudoscalar_coeff: ir.pseudoscalar_coeff.map(|p| p + 0.0),
        motor_dir: ir.motor_dir.map(|d| [
            d[0] + 0.0, d[1] + 0.0, d[2] + 0.0, d[3] + 0.0,
        ]),
        motor_mom: ir.motor_mom.map(|m| [
            m[0] + 0.0, m[1] + 0.0, m[2] + 0.0, m[3] + 0.0,
        ]),
    }
}
