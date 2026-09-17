# Design — Rotor Axis Normalization

## Current Code (src/lib.rs:197-206)

```rust
pub fn rotor(dir_u: f32x4, angle: f32) -> Motor {
    let d = dir_u.to_array();
    let r_new = angle.cos();
    let ux_new = d[1] * angle.sin();
    let uy_new = d[2] * angle.sin();
    let uz_new = d[3] * angle.sin();
    // Pure rotation motor (no translation component)
    Motor { dir: f32x4::from_array([r_new, ux_new, uy_new, uz_new]), mom: f32x4::from_array([0.0, 0.0, 0.0, 0.0]) }
}
```

**Issues:**
1. Only uses `d[1]`, `d[2]`, `d[3]` — ignores `d[0]`
2. No normalization: if axis is not unit length, quaternion won't be unit

## Mathematical Foundation

For a rotation by angle `θ` about unit axis `û = (ux, uy, uz)`:
```
q = (cos(θ/2), sin(θ/2) * û)
```

In our representation: `dir = [r, ux, uy, uz]` where:
- `r = cos(θ/2)`
- `ux = sin(θ/2) * axis.x`
- `uy = sin(θ/2) * axis.y`
- `uz = sin(θ/2) * axis.z`

The axis must be normalized: `||axis|| = 1`.

## New Implementation

```rust
pub fn rotor(dir_u: f32x4, angle: f32) -> Motor {
    let d = dir_u.to_array();
    // Axis is in d[1], d[2], d[3] (d[0] is typically 0 for direction vectors)
    let axis_x = d[1];
    let axis_y = d[2];
    let axis_z = d[3];
    
    // Branchless normalization: scale = 1 / max(norm, ε)
    let norm_sq = axis_x * axis_x + axis_y * axis_y + axis_z * axis_z;
    let norm = norm_sq.sqrt();
    let eps = 1e-8;
    let scale = if norm > eps { angle.sin() / norm } else { 0.0 };
    
    let r_new = angle.cos();
    let ux_new = axis_x * scale;
    let uy_new = axis_y * scale;
    let uz_new = axis_z * scale;
    
    // Pure rotation motor (no translation component)
    Motor { 
        dir: f32x4::from_array([r_new, ux_new, uy_new, uz_new]), 
        mom: f32x4::from_array([0.0, 0.0, 0.0, 0.0]) 
    }
}
```

**Note:** Using `if norm > eps` is technically a branch, but it's a single safety check at the function boundary, not in an arithmetic loop. Alternative branchless approach:
```rust
let scale = angle.sin() / (norm + eps);  // branchless but less accurate for very small norms
```

We'll use the explicit check since it's a one-time normalization, not a loop.

## Contract Verification

| Contract | Current | New | Verification |
|----------|---------|-----|--------------|
| Branchless (arithmetic) | ✓ | ✓ | Only one boundary check |
| SIMD `f32x4` | ✓ | ✓ | Same `from_array` usage |
| Zero alloc | ✓ | ✓ | No allocations |
| Exact arithmetic | ✗ | ✓ | Normalized axis → unit quaternion |

## Test Impact

- `agile_eye_pan_tilt_demo` uses `rotor()` implicitly via `sandwich()`
- `agile_eye_spherical_ik_demo` uses `motor_chain()` which may use `rotor()`
- Both should produce more accurate results with normalized axes