# Design: Parser/IR Pipeline — Real Operand Extraction & Grade-Aware Lowering

## Overview
Replace the hardcoded stub parser with a real token→AST→symbol-table→IR pipeline that extracts actual operands, preserves symbol-table context through lowering, and validates grade contracts.

## Architecture

### Data Flow
```
.pga source
    ↓ tokenize() → Vec<(PgaToken, line, col)>
    ↓ parse() → PgaAst (with REAL operands extracted)
    ↓ parse_with_symbol_table() → (PgaAst, SymbolTable, OperandTracker)
    ↓ lower_ast_to_ir() with OperandContext → Vec<Op>
    ↓ validate_op_grade() (grade-aware) → bool
```

### Key Types

#### `PgaAst` — Aligned with Runtime Signatures
```rust
pub enum PgaAst {
    Wedge(Plane, Plane),                              // wedge(p: Plane, q: Plane)
    Vee(Point, Point),                                // vee(p: Point, q: Point)
    SandwichPoint(Motor, Point),                      // sandwich(m: &Motor, target: Point)
    SandwichPlane(Motor, Plane),                      // sandwich(m: &Motor, target: Plane) — NEW
    IntersectPlanePoint(Plane, Point),                // intersect_plane_point(p: Plane, pt: Point)
    Chain(Vec<Motor>),                                // motor_chain(chain: &[Motor])
    Rotor(Motor),                                     // rotor(dir_u: f32x4, angle: f32) → Motor
    GeomProduct(Plane, Point),                        // geometric_product(p: Plane, pt: Point)
    PointLineIntersect(Point, Line),                  // point_line_intersect(pt: Point, ln: &Line)
    MotorChain(Vec<Motor>),                           // motor_chain(chain: &[Motor])
    RedundancyMetric(Motor),                          // redundancy_metric(m: &Motor)
    SphereIntersectPlane(Point, f32, Plane),          // sphere_intersect_plane(center: Point, radius: f32, plane: Plane)
    SphereIntersectSphere(Point, f32, Point, f32),    // sphere_intersect_sphere(c1: Point, r1: f32, c2: Point, r2: f32)
    Projection(Plane, Plane),                         // projection(a: Plane, b: Plane)
    Rejection(Plane, Plane),                          // rejection(a: Plane, b: Plane)
    Pseudoscalar(f32),                                // pseudoscalar_normalize(p: f32)
    Line([f32; 6]),                                   // NEW: Line literal for PointLineIntersect
}
```

#### `OperandContext` — Bridges Symbol Table → IR
```rust
pub struct OperandContext {
    /// Maps operand slot index → (grade, concrete value or placeholder)
    pub slots: Vec<OperandSlot>,
}

pub struct OperandSlot {
    pub grade: Grade,  // Scalar/Vector/Bivector/Trivector/Pseudoscalar/Motor
    pub value: Option<OperandValue>,
}

pub enum OperandValue {
    Plane(Plane),
    Point(Point),
    Line([f32; 6]),
    Motor(Motor),
    Scalar(f32),
}
```

#### `Grade` — For Validation
```rust
pub enum Grade {
    Scalar,       // 0
    Vector,       // 1 (Plane)
    Bivector,     // 2 (Line)
    Trivector,    // 3 (Point)
    Pseudoscalar, // 4
    Motor,        // Even mixed
}
```

## Component Changes

### 1. `src/parser/ast.rs`
- Add `Debug` derive
- Fix `PointLineIntersect(Point, Line)` 
- Fix `SphereIntersectPlane(Point, f32, Plane)`
- Fix `SphereIntersectSphere(Point, f32, Point, f32)`
- Add `Line([f32; 6])` variant
- Add `SandwichPlane(Motor, Plane)` — already exists, keep

### 2. `src/parser/mod.rs` — `parse()`
- Consume first keyword token
- Parse subsequent operands based on operation arity/type
- Return `PgaAst` with **actual parsed values** (not placeholders)
- Minimal recursive descent: keyword → operand list
- Use `token::tokenize` output directly

### 3. `src/parser/symbol_table.rs` — `parse_with_symbol_table()`
- Already builds `SymbolTable` + `OperandTracker`
- Return these alongside `PgaAst` (already does)
- **Fix**: `parse_and_lower()` in `mod.rs` must consume and pass them to IR lowering

### 4. `src/parser/ir.rs` — Lowering & Validation
- **New**: `lower_ast_to_ir_with_context(ast, ctx: &OperandContext) → Vec<Op>`
- Replace hardcoded `grade_index` magic with operand context slot indices
- **New**: `validate_op_grade(op: &Op, ctx: &OperandContext) → bool` — checks operand grades match Op expectations
- Add grade field to `Op` variants or derive from context

### 5. `src/parser/type_def.rs`
- Add `Grade` enum for validation
- Keep `GradeMask` for IR optimization passes

### 6. Integration: `parse_and_lower()`
```rust
pub fn parse_and_lower(input: &str) -> Result<Vec<Op>, String> {
    let tokens = token::tokenize(input);
    let (ast, sym_table, op_tracker) = symbol_table::parse_with_symbol_table(...);
    let ctx = build_operand_context(&ast, &sym_table, &op_tracker);
    let ops = ir::lower_ast_to_ir_with_context(&ast, &ctx);
    // Validate each op
    for op in &ops {
        if !ir::validate_op_grade(op, &ctx) {
            return Err("grade mismatch".into());
        }
    }
    Ok(ops)
}
```

## Emission Compatibility
- `emit.rs` / `emit_cuda.rs` already emit branchless WGSL/CUDA from `PgaAst`
- AST changes must not break emission — emission matches AST variants by name
- Verify: `emit_wgsl(&ast)` still produces correct shader strings

## Test Strategy
1. **Parser extraction**: `parse("wedge Plane Point")` → `Wedge(parsed_plane, parsed_point)`
2. **Symbol table flow**: `parse_and_lower` returns ops with real indices from context
3. **Grade validation**: `validate_op_grade` rejects `Wedge` with Point operands
4. **Round-trip**: syntax → tokens → AST → IR → validate → emit
5. **Geometric properties**: wedge antisymmetry via parsed values

## Risk Mitigation
- Keep changes surgical: parser/IR layer only
- Don't touch `src/lib.rs` geometric functions
- Preserve `f32x4` SIMD, branchless arithmetic contracts
- Run `cargo +nightly test` after each file change