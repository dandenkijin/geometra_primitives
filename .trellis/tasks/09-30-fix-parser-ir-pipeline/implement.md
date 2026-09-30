# Implementation Plan: Parser/IR Pipeline Fix

## Phase 1: AST Signature Fixes (`src/parser/ast.rs`)

### 1.1 Update `PgaAst` enum
- [ ] Add `#[derive(Debug, Clone, PartialEq)]`
- [ ] Change `PointLineIntersect(Plane, Point)` → `PointLineIntersect(Point, Line)`
- [ ] Change `SphereIntersectPlane(Point, Motor, Plane)` → `SphereIntersectPlane(Point, f32, Plane)`
- [ ] Change `SphereIntersectSphere(Point, Point, Point)` → `SphereIntersectSphere(Point, f32, Point, f32)`
- [ ] Add `Line([f32; 6])` variant for line literals
- [ ] Keep existing: `Wedge`, `Vee`, `SandwichPoint`, `SandwichPlane`, `IntersectPlanePoint`, `Chain`, `Rotor`, `GeomProduct`, `MotorChain`, `RedundancyMetric`, `Projection`, `Rejection`, `Pseudoscalar`

### 1.2 Verify emission compatibility
- [ ] Check `emit.rs` and `emit_cuda.rs` match new AST variants
- [ ] Update emission if needed (variant names must match)

## Phase 2: Real Operand Parsing (`src/parser/mod.rs`)

### 2.1 Implement `parse_operands()` helper
- [ ] Parse `Plane` from 4 floats (or `Plane` keyword + 4 floats)
- [ ] Parse `Point` from 4 floats (or `Point` keyword + 4 floats)
- [ ] Parse `Line` from 6 floats (or `Line` keyword + 6 floats)
- [ ] Parse `Motor` from 8 floats (dir 4 + mom 4) or `Motor` keyword + 8 floats
- [ ] Parse scalar `f32` from `FloatLiteral`

### 2.2 Rewrite `parse()` function
- [ ] Remove all hardcoded placeholder returns
- [ ] Match first token to operation type
- [ ] Call appropriate operand parser for each variant
- [ ] Return `Ok(PgaAst::Variant(real_operands...))` or `Err`
- [ ] Handle all tokens in `PgaToken` enum

### 2.3 Update `parse_and_lower()`
- [ ] Capture `(ast, sym_table, op_tracker)` from `parse_with_symbol_table`
- [ ] Build `OperandContext` from symbol table + tracker + AST
- [ ] Call `ir::lower_ast_to_ir_with_context(&ast, &ctx)`
- [ ] Validate each op with `ir::validate_op_grade(op, &ctx)`
- [ ] Return `Result<Vec<Op>, String>`

## Phase 3: Operand Context & IR Lowering (`src/parser/ir.rs`)

### 3.1 Add `Grade` enum and `OperandContext`
- [ ] Define `Grade` enum (Scalar, Vector, Bivector, Trivector, Pseudoscalar, Motor)
- [ ] Define `OperandSlot { grade: Grade, value: Option<OperandValue> }`
- [ ] Define `OperandValue` enum matching AST operand types
- [ ] Define `OperandContext { slots: Vec<OperandSlot> }`

### 3.2 Implement `build_operand_context()` (in ir.rs or mod.rs)
- [ ] Walk AST and assign slot indices to each operand
- [ ] Record grade for each operand based on its type
- [ ] Return `OperandContext`

### 3.3 Rewrite `lower_ast_to_ir()` → `lower_ast_to_ir_with_context()`
- [ ] Accept `&OperandContext` parameter
- [ ] Use context slot indices instead of hardcoded `grade_index()`
- [ ] Remove `grade_index()` helper
- [ ] Build `Op` variants with real operand indices from context

### 3.4 Implement real `validate_op_grade()`
- [ ] Check each `Op` variant's operand grades against expected grades
- [ ] `WedgePlanes` → both operands must be `Vector` (Plane = grade 1)
- [ ] `VeePoints` → both operands must be `Trivector` (Point = grade 3)
- [ ] `SandwichPt` → motor (Motor) + point (Trivector)
- [ ] `SandwichPl` → motor (Motor) + plane (Vector)
- [ ] `Intersect` → plane (Vector) + point (Trivector) OR line (Bivector) + point (Trivector)
- [ ] `ChainMotors` → all operands must be `Motor`
- [ ] Return `bool` — `true` if all grades match, `false` otherwise

## Phase 4: Symbol Table Integration (`src/parser/symbol_table.rs`)

### 4.1 Ensure `parse_with_symbol_table` returns useful data
- [ ] Verify it builds correct symbol table for identifiers
- [ ] Verify operand tracker maps tokens to operand slots
- [ ] No changes needed if already working — just ensure `parse_and_lower` consumes it

## Phase 5: Tests (`tests/test_pga.rs`, `tests/test_pga_pipeline.rs`)

### 5.1 Add parser operand extraction tests
- [ ] Test `parse("wedge Plane Point")` extracts real Plane/Point
- [ ] Test `parse("vee Point Point")` extracts two Points
- [ ] Test `parse("sphere_intersect_plane Point 1.0 Plane")` extracts Point, f32, Plane

### 5.2 Add IR lowering tests
- [ ] Test `lower_ast_to_ir_with_context` uses operand context indices
- [ ] Test `validate_op_grade` passes for valid grades
- [ ] Test `validate_op_grade` fails for mismatched grades

### 5.3 Add round-trip tests
- [ ] syntax → tokens → AST → context → IR → validate → emit

### 5.4 Verify existing tests still pass
- [ ] All 11 geometry tests in `test_pga.rs`
- [ ] Pipeline test in `test_pga_pipeline.rs`

## Phase 6: Compliance Cleanup

### 6.1 Verify `pseudoscalar_normalize` branchless
- [ ] Already fixed to `p * p` ✓

### 6.2 Verify `sphere_intersect_plane` branchless
- [ ] Check `src/lib.rs:163` implementation

## Validation Commands
```bash
# After each phase
cargo +nightly check --all-targets
cargo +nightly test

# Final verification
cargo +nightly check --all-targets
cargo +nightly test
```

## File Edit Order
1. `src/parser/ast.rs` — AST signatures
2. `src/parser/token.rs` — verify token types cover all operands
3. `src/parser/mod.rs` — parse() + parse_and_lower()
4. `src/parser/ir.rs` — OperandContext + lowering + validation
5. `src/parser/symbol_table.rs` — verify integration
6. `src/parser/emit.rs` — update for AST changes
7. `src/parser/emit_cuda.rs` — update for AST changes
8. `tests/test_pga.rs` — add parser/IR tests
9. `tests/test_pga_pipeline.rs` — update for new pipeline