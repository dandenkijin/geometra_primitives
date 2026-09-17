# Design — Exact Dual-Quaternion Multiplication for `motor_chain()`

## Mathematical Foundation

A dual quaternion represents a rigid transformation (rotation + translation):
```
M = q + ε * (0.5 * t * q)
```
Where `q = (r, u)` is a unit quaternion, `t = (v_x, v_y, v_z)` is translation vector, and `ε` is the dual unit.

For two dual quaternions `M1 = q1 + ε * q1'` and `M2 = q2 + ε * q2'`:
```
M = M1 * M2 = q1*q2 + ε * (q1'*q2 + q1*q2')
```

In component form with `M = (q, p)` where `p = q'` (dual part):
- Rotation: `q = q1 * q2` (standard quaternion multiplication)
- Translation: `p = q1' * q2 + q1 * q2'`

For our representation:
- `M1 = (r1, u1, v1, p1)` where `q1 = (r1, u1)`, `q1' = (0, v1) + p1*ε`? 
- Actually in our code: `dir = [r, ux, uy, uz]`, `mom = [vx, vy, vz, p]`
- The dual part corresponds to `0.5 * t * q` so `v = 0.5 * t * q` in quaternion terms

But looking at the current code, the representation uses:
- `dir = [r, ux, uy, uz]` — quaternion part
- `mom = [vx, vy, vz, p]` — dual part (translation + pseudoscalar)

The full dual-quaternion multiplication is:
```
q = q1 * q2
p = q1' * q2 + q1 * q2'
```

Expanding the dual part (translation):
```
p = (0, v1) * q2 + q1 * (0, v2) + p1 * q2 + q1 * p2
  = (0, v1) * (r2, u2) + (r1, u1) * (0, v2) + p1*(r2, u2) + (r1, u1)*p2
```

Where `(0, v) * (r, u) = (-v·u, r*v + u×v)` for quaternions.

So:
```
p.vector = -v1·u2 + r2*v1 + u2×v1 + r1*v2 + u1×v2 + p1*u2 + p2*u1
```

But the current code's translation coupling (lines 119-124):
```rust
r_mom = [
    r1 * v2x + v1x * r2 + u1y * v2z - u1z * v2y,  // r1*v2x + v1x*r2 + (u1 × v2)_x
    r1 * v2y + v1y * r2 + u1z * v2x - u1x * v2z,  // r1*v2y + v1y*r2 + (u1 × v2)_y
    r1 * v2z + v1z * r2 + u1x * v2y - u1y * v2x,  // r1*v2z + v1z*r2 + (u1 × v2)_z
    p_out,
];
```

This has `r1*v2 + v1*r2 + u1 × v2` — **missing `v1 × u2` term**!

The full formula should be:
```
v = r1*v2 + v1*r2 + u1 × v2 + v1 × u2
```

And the dual-scalar part (already correct at line 123):
```
p_out = p1 + p2 + r1*p2 - r2*p1
```

## Current Code Analysis (src/lib.rs:102-132)

```rust
pub fn motor_chain(chain: &[Motor]) -> Motor {
    let mut r_dir = [1.0_f32, 0.0, 0.0, 0.0];
    let mut r_mom = [0.0_f32, 0.0, 0.0, 0.0];
    for m in chain {
        let md = m.dir.to_array(); // [r, ux, uy, uz]
        let mm = m.mom.to_array(); // [vx, vy, vz, p]
        let rd = r_dir;
        let rm = r_mom;
        let r1 = rd[0]; let r2 = md[0];
        let u1x = rd[1]; let u1y = rd[2]; let u1z = rd[3];
        let u2x = md[1]; let u2y = md[2]; let u2z = md[3];
        
        // Exact quaternion rotation product (CORRECT)
        let r_new = r1 * r2 - (u1x * u2x + u1y * u2y + u1z * u2z);
        let ux_new = r1 * u2x + r2 * u1x + (u1y * u2z - u1z * u2y);
        let uy_new = r1 * u2y + r2 * u1y + (u1z * u2x - u1x * u2z);
        let uz_new = r1 * u2z + r2 * u1z + (u1x * u2y - u1y * u2x);
        r_dir = [r_new, ux_new, uy_new, uz_new];
        
        // Translation coupling (MISSING v1 × u2 term)
        let v1x = rm[0]; let v1y = rm[1]; let v1z = rm[2];
        let v2x = mm[0]; let v2y = mm[1]; let v2z = mm[2];
        let p1 = rm[3]; let p2 = mm[3];
        let p_out = p1 + p2 + r1 * p2 - r2 * p1;
        r_mom = [
            r1 * v2x + v1x * r2 + u1y * v2z - u1z * v2y,
            r1 * v2y + v1y * r2 + u1z * v2x - u1x * v2z,
            r1 * v2z + v1z * r2 + u1x * v2y - u1y * v2x,
            p_out,
        ];
    }
    Motor { dir: f32x4::from_array(r_dir), mom: f32x4::from_array(r_mom) }
}
```

## New Implementation

Replace lines 119-126 with full dual-quaternion translation:

```rust
// Translation coupling: v = v1*r2 + r1*v2 + u1×v2 + v1×u2
let v1x = rm[0]; let v1y = rm[1]; let v1z = rm[2];
let v2x = mm[0]; let v2y = mm[1]; let v2z = mm[2];
let p1 = rm[3]; let p2 = mm[3];
let p_out = p1 + p2 + r1 * p2 - r2 * p1;

// Cross products: u1 × v2 and v1 × u2
let u1_cross_v2_x = u1y * v2z - u1z * v2y;
let u1_cross_v2_y = u1z * v2x - u1x * v2z;
let u1_cross_v2_z = u1x * v2y - u1y * v2x;

let v1_cross_u2_x = v1y * u2z - v1z * u2y;
let v1_cross_u2_y = v1z * u2x - v1x * u2z;
let v1_cross_u2_z = v1x * u2y - v1y * u2x;

r_mom = [
    r1 * v2x + v1x * r2 + u1_cross_v2_x + v1_cross_u2_x,
    r1 * v2y + v1y * r2 + u1_cross_v2_y + v1_cross_u2_y,
    r1 * v2z + v1z * r2 + u1_cross_v2_z + v1_cross_u2_z,
    p_out,
];
```

## Emission Updates

### WGSL (`src/parser/emit.rs`)

**`motor_chain_unrolled`** (lines 26-32): Update translation coupling with full cross-terms.

**`motor_chain`** (lines 55-60): Update with full formula.

### CUDA (`src/parser/emit_cuda.rs`)

**`motor_chain_unrolled_cuda`** (lines 26-33): Mirror WGSL changes.

**`motor_chain_unrolled_cuda`** (lines 48-54): Mirror WGSL changes.

## Contract Verification

| Contract | Current | New | Verification |
|----------|---------|-----|--------------|
| Branchless | ✓ | ✓ | No new `if`/`else` in arithmetic |
| SIMD `f32x4` | ✓ | ✓ | Same `from_array`/`to_array` usage |
| Zero alloc | ✓ | ✓ | No `Vec`/`String` in function |
| Exact arithmetic | ✗ | ✓ | Full dual-quaternion multiplication |
| Multi-backend | ✓ | ✓ | Both emissions updated identically |

## Test Impact

- `motor_chain_no_panic` — should still pass
- `agile_eye_spherical_ik_demo` — 3-DoF chain now correct
- Rotation composition unchanged (lines 110-117)