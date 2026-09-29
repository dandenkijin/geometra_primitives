# PRD: Fix pseudoscalar_normalize branchless contract

## Problem
The `pseudoscalar_normalize` function in `src/lib.rs:273-275` violates the branchless arithmetic contract used throughout `geometra_primitives`:

```rust
pub fn pseudoscalar_normalize(p: f32) -> f32 {
    if p == 0.0 { 0.0 } else { 1.0 / p }  // ❌ BRANCH
}
```

Meanwhile, the code emitter (both WGSL and CUDA) emits a branchless version:
```rust
// src/parser/emit.rs:46
"fn pseudoscalar_normalize(p: f32) -> f32 {\n    return p * p;\n}"

// src/parser/emit_cuda.rs:42
"__device__ float pseudoscalar_normalize_cuda(float p) {\n    return p * p;\n}"
```

## Requirements
1. **Branchless implementation**: The Rust library function must be branchless (no `if`, `match`, `? :`, or short-circuit `&&`/`||` on the hot path)
2. **Consistency with emission**: The library implementation should match the emitted `p * p` behavior, or if `1/p` is mathematically correct, implement it branchlessly
3. **Singularity handling**: `p = 0` must return `0` (metric zero) without branching
4. **Preserve function signature**: `pub fn pseudoscalar_normalize(p: f32) -> f32`

## Acceptance Criteria
- [ ] `cargo +nightly build` succeeds
- [ ] `cargo +nightly test` passes (all 11 tests)
- [ ] No branches in `pseudoscalar_normalize` implementation (verified by inspection)
- [ ] Behavior at `p = 0.0` returns `0.0`
- [ ] Behavior at `p != 0.0` matches chosen mathematical definition (either `p * p` or branchless `1/p`)
- [ ] Emission code in `emit.rs` and `emit_cuda.rs` is consistent with library implementation

## Notes
- The comment says "Metric inverse: normalize by pseudoscalar magnitude" suggesting `1/p`, but emitted code uses `p * p`
- In R_3_0_1 PGA, pseudoscalar I satisfies I² = 0, so `p * p` could represent the metric scale factor
- Aligning library with emission (`p * p`) is the simplest fix and maintains consistency
- If keeping `1/p` semantics, use branchless: `p.recip() * (p != 0.0) as f32`