# Compiler Pipeline — CUDA Multi-Backend Emission

Scope: extend `emit.rs` with CUDA kernel emission (`emit_cuda.rs`) using exact geometric contracts.

Contracts preserved (locked):
- Branchless scalar arithmetic (0 executable `if`/`else` in geometric loops/solvers).
- Zero placeholders / incomplete stubs — complete `String` emission logic only.
- Zero heap allocations (`format!` / array-based; no `String::new()` builder loops; no conditional branches in arithmetic).
- Standard library only (no external crates beyond existing `Cargo.toml` `logos` dependency kept for future `.Logos` integration; no new dependencies).
- Exact geometric contracts (`f32x4` dense SIMD contracts; `w_out` positive norm; `Vee` Plücker mapping; `intersect` scalar FMA; `motor_chain` quaternion rotation + geometric translation; primitives verified).
- Multi-backend independent (`WGSL` emission preserved; `CUDA` emission added independently using same `PgaAst` + `PgaIr`).

Deliverable: `src/parser/emit_cuda.rs` (new file): exact CUDA shader emission strings matching `PgaAst` variants (`Wedge`, `Vee`, `SandwichPoint`, `SandwichPlane`, `IntersectPlanePoint`, `Chain`, `Rotor`, `Projection`, `Rejection`, `Pseudoscalar`, primitives) using scalar arithmetic (`float` arrays, dense `f32x4` mapped to `float4` or scalar arithmetic).

Not changing: `src/lib.rs` geometric contracts; `token.rs` (`manual .pga`); `ast.rs` (`PgaAst` structure); `ir.rs` (`PgaIr` optimization passes — `.map()` element-wise zero, identity arithmetic, dense `+0.0` accumulation); `emit.rs` (`WGGL` emission preserved); `.gitignore`; `.trellis/tasks/main-spec/` artifacts.
