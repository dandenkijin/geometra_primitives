# Compiler Pipeline Graded Multivector — PRD (Draft)

## User Request
Upgrade compiler (`geometra_primitives`) to model graded multivectors and emit optimized SIMD code:
- Graded multivector representation (`PgaIr`)
- Optimization passes: zero pruning, constant folding, term merging
- Unrolled SIMD emission via `emit_wgsl`
- Multi-backend slot preserved

## Confirmed Facts (from repository)
- `f32x4` dense SIMD (`src/lib.rs`); `PgaToken` tokenizer (`token.rs`); `PgaAst` split (`ast.rs`); `PgaIr` stub (`ir.rs`); `emit.rs` produces exact WGSL shader strings.
- Reference: `arXiv:2311.04744` (Euclidean / Projective `R_3_0_1` / Conformal GA); FIKA (`Machines 2024, 12, 78` — inverse kinematics via geometric primitives + motor sandwich).
- No branches in geometric core (`0` executable branches verified).
- Pipeline architecture: `.pga` syntax -> tokenizer -> AST -> IR -> emission (multi-backend independent).

## Open Scope / Risk Decisions (highest-value question)
The user mentioned Euclidean / Projective / Conformal algebras. Should `PgaIr` model:
- Full 16-dimensional `R_3_0_1` only (current scope)?
- A family of sub-algebras (`R_3_0_0` Euclidean, `R_4_1_0` Conformal) — requires grade-filtering optimization passes?

This impacts optimization pass design (`prune_zeros`, `merge_terms`) significantly.

## Recommended Approach (Recommended)
Model full `R_3_0_1` graded multivector (`Scalar` through `Pseudoscalar`) with grade-aware optimization passes. This aligns with FIKA's geometric primitive approach and preserves multi-backend slot.
Trade-off: broader scope requires more comprehensive grade-filtering logic; narrower sub-algebra family reduces complexity but limits future backends.

## Out of Scope (for this PRD)
- Changing geometric algebra contracts (`src/lib.rs` SIMD primitives unchanged).
- Changing emission architecture (multi-backend preserved; emission strings exact).
- Changing syntax/tokenizer structure (`token.rs` accurate `.pga` mapping preserved).

## Final Scope Decision: Full R_3_0_1 graded multivector (recommended).
Includes grade-aware optimization: prune_zeros, fold_constants, merge_terms.
Not doing: sub-algebra family restriction; GUI/IDE integration; performance benchmarking (deferred).
