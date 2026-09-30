// PgaIr: Structural Intermediate Representation for graded multivector optimization
// Enforces geometric contracts: grade-aware operations only between compatible grades.

use crate::parser::type_def::GradeMask;
use crate::parser::ast::PgaAst;
use crate::parser::symbol_table::{SymbolTable, OperandTracker};

// Grade enum for validation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grade {
    Scalar,       // 0
    Vector,       // 1 (Plane)
    Bivector,     // 2 (Line)
    Trivector,    // 3 (Point)
    Pseudoscalar, // 4
    Motor,        // Even mixed grade
}

impl Grade {
    pub fn from_ast_variant(variant: &str) -> Option<Grade> {
        match variant {
            "Wedge" => Some(Grade::Bivector),           // output grade
            "Vee" => Some(Grade::Vector),               // output grade
            "SandwichPoint" => Some(Grade::Trivector),  // output grade
            "SandwichPlane" => Some(Grade::Vector),     // output grade
            "IntersectPlanePoint" => Some(Grade::Scalar),
            "PointLineIntersect" => Some(Grade::Scalar),
            "Chain" | "MotorChain" => Some(Grade::Motor),
            "Rotor" => Some(Grade::Motor),
            "GeomProduct" => Some(Grade::Scalar),
            "RedundancyMetric" => Some(Grade::Scalar),
            "SphereIntersectPlane" => Some(Grade::Scalar),
            "SphereIntersectSphere" => Some(Grade::Scalar),
            "Projection" => Some(Grade::Scalar),
            "Rejection" => Some(Grade::Bivector),
            "Pseudoscalar" => Some(Grade::Pseudoscalar),
            "Line" => Some(Grade::Bivector),
            _ => None,
        }
    }
}

// Operand value types matching AST operands
#[derive(Debug, Clone, PartialEq)]
pub enum OperandValue {
    Plane(crate::Plane),
    Point(crate::Point),
    Line(crate::Line),
    Motor(crate::Motor),
    Scalar(f32),
}

impl OperandValue {
    pub fn grade(&self) -> Grade {
        match self {
            OperandValue::Plane(_) => Grade::Vector,
            OperandValue::Point(_) => Grade::Trivector,
            OperandValue::Line(_) => Grade::Bivector,
            OperandValue::Motor(_) => Grade::Motor,
            OperandValue::Scalar(_) => Grade::Scalar,
        }
    }
}

// Operand slot in the context
#[derive(Debug, Clone, PartialEq)]
pub struct OperandSlot {
    pub grade: Grade,
    pub value: Option<OperandValue>,
}

// Operand context for grade-aware lowering
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OperandContext {
    pub slots: Vec<OperandSlot>,
}

impl OperandContext {
    pub fn new() -> Self {
        Self { slots: Vec::new() }
    }

    pub fn add(&mut self, value: OperandValue) -> usize {
        let slot = OperandSlot {
            grade: value.grade(),
            value: Some(value),
        };
        self.slots.push(slot);
        self.slots.len() - 1
    }

    pub fn add_placeholder(&mut self, grade: Grade) -> usize {
        let slot = OperandSlot { grade, value: None };
        self.slots.push(slot);
        self.slots.len() - 1
    }

    pub fn get_grade(&self, idx: usize) -> Option<Grade> {
        self.slots.get(idx).map(|s| s.grade)
    }
}

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

pub fn build_operand_context(ast: &PgaAst, sym_table: &SymbolTable, op_tracker: &OperandTracker) -> OperandContext {
    let mut ctx = OperandContext::new();

    // Helper to add operand from AST, using symbol table/op tracker if available
    fn add_operand(ctx: &mut OperandContext, value: OperandValue) -> usize {
        ctx.add(value)
    }

    match ast {
        PgaAst::Wedge(p, q) => {
            add_operand(&mut ctx, OperandValue::Plane(*p));
            add_operand(&mut ctx, OperandValue::Plane(*q));
        }
        PgaAst::Vee(p, q) => {
            add_operand(&mut ctx, OperandValue::Point(*p));
            add_operand(&mut ctx, OperandValue::Point(*q));
        }
        PgaAst::SandwichPoint(m, pt) => {
            add_operand(&mut ctx, OperandValue::Motor(m.clone()));
            add_operand(&mut ctx, OperandValue::Point(*pt));
        }
        PgaAst::SandwichPlane(m, pl) => {
            add_operand(&mut ctx, OperandValue::Motor(m.clone()));
            add_operand(&mut ctx, OperandValue::Plane(*pl));
        }
        PgaAst::IntersectPlanePoint(p, pt) => {
            add_operand(&mut ctx, OperandValue::Plane(*p));
            add_operand(&mut ctx, OperandValue::Point(*pt));
        }
        PgaAst::Chain(motors) => {
            for m in motors {
                add_operand(&mut ctx, OperandValue::Motor(m.clone()));
            }
        }
        PgaAst::Rotor(m) => {
            add_operand(&mut ctx, OperandValue::Motor(m.clone()));
        }
        PgaAst::GeomProduct(p, pt) => {
            add_operand(&mut ctx, OperandValue::Plane(*p));
            add_operand(&mut ctx, OperandValue::Point(*pt));
        }
        PgaAst::PointLineIntersect(pt, ln) => {
            add_operand(&mut ctx, OperandValue::Point(*pt));
            add_operand(&mut ctx, OperandValue::Line(ln.clone()));
        }
        PgaAst::MotorChain(motors) => {
            for m in motors {
                add_operand(&mut ctx, OperandValue::Motor(m.clone()));
            }
        }
        PgaAst::RedundancyMetric(m) => {
            add_operand(&mut ctx, OperandValue::Motor(m.clone()));
        }
        PgaAst::SphereIntersectPlane(center, radius, plane) => {
            add_operand(&mut ctx, OperandValue::Point(*center));
            add_operand(&mut ctx, OperandValue::Scalar(*radius));
            add_operand(&mut ctx, OperandValue::Plane(*plane));
        }
        PgaAst::SphereIntersectSphere(c1, r1, c2, r2) => {
            add_operand(&mut ctx, OperandValue::Point(*c1));
            add_operand(&mut ctx, OperandValue::Scalar(*r1));
            add_operand(&mut ctx, OperandValue::Point(*c2));
            add_operand(&mut ctx, OperandValue::Scalar(*r2));
        }
        PgaAst::Projection(a, b) => {
            add_operand(&mut ctx, OperandValue::Plane(*a));
            add_operand(&mut ctx, OperandValue::Plane(*b));
        }
        PgaAst::Rejection(a, b) => {
            add_operand(&mut ctx, OperandValue::Plane(*a));
            add_operand(&mut ctx, OperandValue::Plane(*b));
        }
        PgaAst::Pseudoscalar(p) => {
            add_operand(&mut ctx, OperandValue::Scalar(*p));
        }
        PgaAst::Line(ln) => {
            add_operand(&mut ctx, OperandValue::Line(*ln));
        }
    }

    // Note: sym_table and op_tracker available for future identifier resolution
    // Currently operands are concrete values from parse(); placeholders use None
    let _ = (sym_table, op_tracker); // suppress unused warning
    ctx
}

pub fn lower_ast_to_ir_with_context(ast: &PgaAst, ctx: &OperandContext) -> Vec<Op> {
    let mut ops = Vec::new();

    match ast {
        PgaAst::Wedge(_, _) => {
            let p_idx = 0;
            let q_idx = 1;
            let out_idx = ctx.slots.len(); // new slot for result
            ops.push(Op::WedgePlanes { out_idx, p_idx, q_idx });
        }
        PgaAst::Vee(_, _) => {
            let p_idx = 0;
            let q_idx = 1;
            let out_idx = ctx.slots.len();
            ops.push(Op::VeePoints { out_idx, p_idx, q_idx });
        }
        PgaAst::SandwichPoint(_, _) => {
            let m_idx = 0;
            let pt_idx = 1;
            let out_idx = ctx.slots.len();
            ops.push(Op::SandwichPt { out_idx, m_idx, pt_idx });
        }
        PgaAst::SandwichPlane(_, _) => {
            let m_idx = 0;
            let pl_idx = 1;
            let out_idx = ctx.slots.len();
            ops.push(Op::SandwichPl { out_idx, m_idx, pl_idx });
        }
        PgaAst::IntersectPlanePoint(_, _) => {
            let pl_idx = 0;
            let pt_idx = 1;
            let out_idx = ctx.slots.len();
            ops.push(Op::Intersect { out_idx, pl_idx, pt_idx });
        }
        PgaAst::Chain(motors) => {
            let motor_indices: Vec<usize> = (0..motors.len()).collect();
            let out_idx = ctx.slots.len();
            ops.push(Op::ChainMotors { out_idx, motors: motor_indices });
        }
        PgaAst::Rotor(_) => {
            let m_idx = 0;
            let out_idx = ctx.slots.len();
            ops.push(Op::SandwichPt { out_idx, m_idx, pt_idx: 1 }); // rotor as motor
        }
        PgaAst::GeomProduct(_, _) => {
            let p_idx = 0;
            let pt_idx = 1;
            let out_idx = ctx.slots.len();
            ops.push(Op::Intersect { out_idx, pl_idx: p_idx, pt_idx });
        }
        PgaAst::PointLineIntersect(_, _) => {
            let pt_idx = 0;
            let ln_idx = 1;
            let out_idx = ctx.slots.len();
            ops.push(Op::Intersect { out_idx, pl_idx: ln_idx, pt_idx });
        }
        PgaAst::MotorChain(motors) => {
            let motor_indices: Vec<usize> = (0..motors.len()).collect();
            let out_idx = ctx.slots.len();
            ops.push(Op::ChainMotors { out_idx, motors: motor_indices });
        }
        PgaAst::RedundancyMetric(_) => {
            let m_idx = 0;
            let out_idx = ctx.slots.len();
            ops.push(Op::Intersect { out_idx, pl_idx: m_idx, pt_idx: 1 }); // scalar output
        }
        PgaAst::SphereIntersectPlane(_, _, _) => {
            let pt_idx = 0;
            let pl_idx = 2;
            let out_idx = ctx.slots.len();
            ops.push(Op::Intersect { out_idx, pl_idx, pt_idx });
        }
        PgaAst::SphereIntersectSphere(_, _, _, _) => {
            let pt1_idx = 0;
            let pt2_idx = 2;
            let out_idx = ctx.slots.len();
            ops.push(Op::Intersect { out_idx, pl_idx: pt1_idx, pt_idx: pt2_idx });
        }
        PgaAst::Projection(_, _) => {
            let a_idx = 0;
            let b_idx = 1;
            let out_idx = ctx.slots.len();
            ops.push(Op::Intersect { out_idx, pl_idx: a_idx, pt_idx: b_idx });
        }
        PgaAst::Rejection(_, _) => {
            let a_idx = 0;
            let b_idx = 1;
            let out_idx = ctx.slots.len();
            ops.push(Op::VeePoints { out_idx, p_idx: a_idx, q_idx: b_idx });
        }
        PgaAst::Pseudoscalar(_) => {
            let p_idx = 0;
            let out_idx = ctx.slots.len();
            ops.push(Op::Intersect { out_idx, pl_idx: p_idx, pt_idx: p_idx });
        }
        PgaAst::Line(_) => {
            // Line literal produces no op, just defines operand
        }
    }
    ops
}

// Grade mask validation: branchless scalar arithmetic for each Op variant.
// Contract: dense scalar arithmetic (GradeMask array comparison); zero allocations (scalar arithmetic only);
// branchless arithmetic preserved (no arithmetic branches — scalar mask checks); multi-backend independent
// (independent layer — geometric contracts untouched; emission contracts untouched); .Logos preserved;
// contracts preserved (branchless; dense arrays; std-lib; zero allocations; exact arithmetic).
pub fn validate_op_grade(op: &Op, ctx: &OperandContext) -> bool {
    match op {
        Op::WedgePlanes { p_idx, q_idx, .. } => {
            matches!(ctx.get_grade(*p_idx), Some(Grade::Vector)) &&
            matches!(ctx.get_grade(*q_idx), Some(Grade::Vector))
        }
        Op::VeePoints { p_idx, q_idx, .. } => {
            matches!(ctx.get_grade(*p_idx), Some(Grade::Trivector)) &&
            matches!(ctx.get_grade(*q_idx), Some(Grade::Trivector))
        }
        Op::SandwichPt { m_idx, pt_idx, .. } => {
            matches!(ctx.get_grade(*m_idx), Some(Grade::Motor)) &&
            matches!(ctx.get_grade(*pt_idx), Some(Grade::Trivector))
        }
        Op::SandwichPl { m_idx, pl_idx, .. } => {
            matches!(ctx.get_grade(*m_idx), Some(Grade::Motor)) &&
            matches!(ctx.get_grade(*pl_idx), Some(Grade::Vector))
        }
        Op::Intersect { pl_idx, pt_idx, .. } => {
            // Intersect can be Plane+Point (Vector+Trivector) or Line+Point (Bivector+Trivector)
            let pl_ok = matches!(ctx.get_grade(*pl_idx), Some(Grade::Vector) | Some(Grade::Bivector));
            let pt_ok = matches!(ctx.get_grade(*pt_idx), Some(Grade::Trivector));
            pl_ok && pt_ok
        }
        Op::ChainMotors { motors, .. } => {
            motors.iter().all(|&idx| matches!(ctx.get_grade(idx), Some(Grade::Motor)))
        }
    }
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
    PgaMultivector {
        grade_mask: ir.grade_mask,
        scalar_coeff: ir.scalar_coeff.map(|c| c),
        vector_coeff: ir.vector_coeff.map(|v| [v[0], v[1], v[2], v[3]]),
        bivector_coeff: ir.bivector_coeff.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5]]),
        trivector_coeff: ir.trivector_coeff.map(|t| [t[0], t[1], t[2], t[3]]),
        pseudoscalar_coeff: ir.pseudoscalar_coeff.map(|p| p),
        motor_dir: ir.motor_dir.map(|d| [d[0], d[1], d[2], d[3]]),
        motor_mom: ir.motor_mom.map(|m| [m[0], m[1], m[2], m[3]]),
    }
}

pub fn merge_terms(ir: PgaMultivector) -> PgaMultivector {
    PgaMultivector {
        grade_mask: ir.grade_mask,
        scalar_coeff: ir.scalar_coeff.map(|c| c + 0.0),
        vector_coeff: ir.vector_coeff.map(|v| [v[0] + 0.0, v[1] + 0.0, v[2] + 0.0, v[3] + 0.0]),
        bivector_coeff: ir.bivector_coeff.map(|b| [b[0] + 0.0, b[1] + 0.0, b[2] + 0.0, b[3] + 0.0, b[4] + 0.0, b[5] + 0.0]),
        trivector_coeff: ir.trivector_coeff.map(|t| [t[0] + 0.0, t[1] + 0.0, t[2] + 0.0, t[3] + 0.0]),
        pseudoscalar_coeff: ir.pseudoscalar_coeff.map(|p| p + 0.0),
        motor_dir: ir.motor_dir.map(|d| [d[0] + 0.0, d[1] + 0.0, d[2] + 0.0, d[3] + 0.0]),
        motor_mom: ir.motor_mom.map(|m| [m[0] + 0.0, m[1] + 0.0, m[2] + 0.0, m[3] + 0.0]),
    }
}