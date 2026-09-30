# PRD: Fix Parser/IR Pipeline — Real Operand Extraction & Grade-Aware Lowering

## Problem
The parser pipeline (`src/parser/*`) currently operates as a hardcoded stub:
- `parse()` returns synthetic AST values regardless of input tokens
- Symbol table and operand tracking are discarded during `parse_and_lower`
- IR lowering (`src/parser/ir.rs`) uses hardcoded indices and `validate_op_grade` is a stub
- AST variants don't match runtime function signatures (e.g., `PointLineIntersect` modeled as plane-point, `SphereIntersectPlane/Sphere` use `Motor`/`Point` placeholders instead of scalar radii)

## Requirements

### 1. AST Signature Alignment
- Unify `PgaAst` variants to match actual runtime function signatures in `src/lib.rs`
- `PointLineIntersect` → line-point (not plane-point)
- `SphereIntersectPlane` → center `Point`, scalar `radius`, `Plane`
- `SphereIntersectSphere` → center1 `Point`, radius1 `f32`, center2 `Point`, radius2 `f32`
- Add `Debug` impl for `PgaAst`

### 2. Real Operand Extraction in `parse()`
- Consume keyword token, parse following operands based on operation type
- Return actual `Plane`, `Point`, `Motor`, or scalar `f32` values (not placeholder arrays)
- Surgical implementation — no full grammar overengineering

### 3. Symbol Table Preservation
- `parse_and_lower` must pass symbol-table/operand metadata to IR lowering
- Operand context (slots, grades) available during lowering

### 4. Operand-Aware IR Lowering
- Replace hardcoded index mapping with operand context / slot allocator
- Use actual operand slots and grades when building `Op` values
- `validate_op_grade` becomes grade-aware and meaningful (not stub `true`)

### 5. Pipeline Coherence
- AST → Symbol Table → IR lowering operates consistently
- Function signatures, AST variants, and emitted operations agree with `src/lib.rs`

### 6. Minor Compliance Cleanup (if needed)
- `pseudoscalar_normalize` — already fixed to branchless `p * p` ✓
- `sphere_intersect_plane` — verify branchless contract

### 7. Tests
- Parser extracts real operands (not placeholders)
- IR lowering uses operand context (not fixed indices)
- AST ↔ runtime signature alignment
- Geometric property checks (wedge antisymmetry, etc.)
- Parser round-trip sanity check

## Acceptance Criteria
- [ ] `cargo +nightly check --all-targets` passes
- [ ] `cargo +nightly test` passes
- [ ] No hardcoded placeholder AST values remain in `parse()`
- [ ] Symbol table and operand tracking used in lowering
- [ ] `validate_op_grade` is grade-aware, not stub
- [ ] AST signatures and runtime semantics agree
- [ ] Existing geometry behavior intact (all 11 tests pass)
- [ ] Branchless arithmetic contracts preserved

## Constraints
- Do not regress geometry fixes in `src/lib.rs`
- Preserve: `f32x4` SIMD, branchless arithmetic, zero-allocation path, explicit boundary checks only
- Surgical edits in parser/IR layer first
- No broad architecture rewrite