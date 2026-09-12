# geometra_pg — R_3_0_1 Projective Geometric Algebra SIMD Runtime

Branchless, high-performance native SIMD geometric algebra for 3D Projective Geometric Algebra (`R_3_0_1`). Designed for equivariant geometric computing and fast inverse kinematics (FIKA) pipelines.

## Mathematical Foundation

- Algebra: Dual Clifford algebra `R_3_0_1` (Projective Geometric Algebra).
- Grade 0: Scalars
- Grade 1: Planes (`[a,b,c,d]`) — stored in `Odd` primitive (`f32x4`)
- Grade 2: Lines (6 Plücker components) — `Even` primitive (split `dir` + `mom` registers)
- Grade 3: Points (`[x,y,z,w]`) — `Odd` primitive (`f32x4`)
- Grade 4: Pseudoscalars

Reference: Dorst et al., *Euclidean, Projective, Conformal: Choosing a Geometric Algebra for Equivariant Transformers* (arXiv:2311.04744, Nov 2023); Carbajal-Espinosa et al., *FIKA: A Conformal Geometric Algebra Approach to a Fast Inverse Kinematics Algorithm for an Anthropomorphic Robotic Arm* (`Machines` 2024, 12, 78).

## Structural Constraints

- Memory: flattened `f32x4` SIMD blocks (`core::simd::f32x4`).
- No empty grades; dense arrays only.
- No conditional branches (`if`, `else`, ternary) inside geometric products or intersection solvers.
- Singularities (parallel configurations, infinity, zero magnitude) drop naturally via algebra metric signatures — no division-by-zero guards.

## Build Requirements

Requires `rustc` nightly (`#![feature(portable_simd)]`). Stable `1.92+` does not support this feature (`E0554`).

```bash
rustc +nightly --edition 2021 --crate-type lib src/lib.rs
cargo +nightly test
```

## Module Layout

```
src/lib.rs            — geometric algebra primitives (branchless SIMD)
src/parser/ast.rs     — PgaAst enum (Wedge, Vee, SandwichPoint, SandwichPlane, ...)
src/parser/token.rs   — .pga tokenizer (manual expanded geometric keywords; .Logos unstable interaction E0432/E0658/E0277/E0618 documented; manual tokenizer accurate; .Logos dependency preserved for future stability)
src/parser/mod.rs     — parser module exports (parse() + parse_and_lower() + diagnostics: line/col tracking)
src/parser/symbol_table.rs — operand extraction + symbol table (independent layer; dense scalar labels; contracts preserved)
src/parser/ir.rs      — PgaIr optimization layer (dense scalar Op labels + grade_index mapping + grade_mask validation; PgaMultivector coexisting; contracts preserved)
src/parser/emit.rs    — WGSL shader emission (exact contracts; multi-backend independent)
src/parser/emit_cuda.rs — CUDA emission (independent multi-backend; contracts preserved)
src/parser/type_def.rs — GradeMask + PgaType (dense scalar grade tracking; contracts preserved)
src/database/mod.rs   — database module exports (independent layer; forecasting + ring-buffer + symbol table)
src/database/ladybug.rs — LadybugDB flat index arrays (SIMD-aligned IDs)
src/database/tiledb.rs — TileDB 3D array schema ([Module_ID, Pass_ID, Timestamp]; 64-byte aligned; contracts preserved)
src/database/forecast.rs — forecasting engine (branchless scalar arithmetic; f32x4 aligned; zero allocations; contracts preserved)
src/database/ring_buffer.rs — async ring-buffer (fixed-size aligned; contracts preserved)
src/database/integration_demo.rs — forecasting + ring-buffer + symbol table integration demo (independent layer; contracts preserved)
```

## Key Primitives

- `wedge(Plane, Plane) -> Line` — exterior product (2D determinant mix, unrolled)
- `vee(Plane, Plane) -> Line` — regressive meet (cross-term mapping)
- `geometric_product(Plane, Point) -> f32` — scalar FMA mix
- `intersect_plane_point(Plane, Point) -> f32` — metric-based singularity drop
- `sandwich(Motor, Point) -> Point` / `sandwich(Motor, Plane) -> Plane` — quaternion rotation + geometric translation
- `motor_chain(Vec<Motor>) -> Motor` — sequential quaternion composition with geometric translation coupling
- `sphere_intersect_plane(Point, f32, Plane) -> f32` / `sphere_intersect_sphere(Point, f32, Point, f32) -> f32` — geometric primitive intersections
- `redundancy_metric(Motor) -> f32` — singularity detection via metric signature

All functions use unrolled scalar arithmetic (`a * b + c`) mapped to native SIMD; singularity handled mathematically without runtime checks.

## Compiler Pipeline

```
.pga source file -> tokenizer (.pga syntax) -> PgaAst (AST) -> PgaIr (optimization IR) -> emit.rs (WGSL shader strings)
```

Multi-backend slot preserved: parser and emission are independent; additional shader backends can be added without changing geometric contracts.

## Testing

```bash
cargo +nightly test
```

11 tests pass (`nightly verified`), including algebraic correctness assertions (`quaternion_rotation_norm_preserved`, `wedge_2d_determinant_equality`, `motor_chain_no_panic`, primitives, `agile_eye_pan_tilt_demo`, `agile_eye_spherical_ik_demo`). Zero executable branches in geometric core.

## Design System & Coding Patterns

See `.trellis/spec/backend/index.md` (to fill) and `.trellis/spec/guides/index.md` (thinking guides: code reuse, cross-layer). Before any feature work: load `trellis-before-dev`, read `prd.md`/`design.md`, state change boundary in `.trellis/tasks/`.
