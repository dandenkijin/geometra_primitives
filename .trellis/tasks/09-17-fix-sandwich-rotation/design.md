# Design — Exact Quaternion Sandwich for `sandwich()`

## Mathematical Foundation

For a unit quaternion `q = (r, u_x, u_y, u_z)` representing rotation, and a vector `v = (x, y, z)` embedded as pure quaternion `v = (0, x, y, z)`, the rotated vector is:

```
v' = q * v * q⁻¹
```

Where `q⁻¹ = (r, -u_x, -u_y, -u_z)` for unit quaternions.

Expanding the sandwich product gives the exact rotation formulas:

```
x' = (r² + ux² - uy² - uz²) * x + 2(ux*uy - r*uz) * y + 2(ux*uz + r*uy) * z
y' = 2(ux*uy + r*uz) * x + (r² - ux² + uy² - uz²) * y + 2(uy*uz - r*ux) * z
z' = 2(ux*uz - r*uy) * x + 2(uy*uz + r*ux) * y + (r² - ux² - uy² + uz²) * z
```

For homogeneous point `P = [x, y, z, w]`, the rotation applies to the vector part, and `w` scales by `||q||² = r² + ux² + uy² + uz²`.

## Current Code Analysis (src/lib.rs:69-84)

```rust
pub fn sandwich(m: &Motor, target: Point) -> Point {
    let t = target.to_array();           // [x, y, z, w]
    let d = m.dir.to_array();            // [r, ux, uy, uz]
    let mo = m.mom.to_array();           // [vx, vy, vz, pw]
    let r_rot = d[0]; let ux = d[1]; let uy = d[2]; let uz = d[3];
    let vx = mo[0]; let vy = mo[1]; let vz = mo[2]; let pw = mo[3];
    
    // CURRENT (WRONG) - left multiplication q * v
    let rot_x = r_rot * t[0] + ux * t[1] + uy * t[2] + uz * t[3];
    let rot_y = r_rot * t[1] - ux * t[0] + uy * t[3] - uz * t[2];
    let rot_z = r_rot * t[2] - ux * t[3] + uy * t[0] - uz * t[1];
    let w_out = t[3] * (r_rot * r_rot + ux * ux + uy * uy + uz * uz);
    
    f32x4::from_array([
        rot_x + vx + r_rot * vx - ux * pw,
        rot_y + vy + r_rot * vy - uy * pw,
        rot_z + vz + r_rot * vz - uz * pw,
        w_out,
    ])
}
```

## New Implementation

Replace the rotation computation (lines 72-76) with exact sandwich:

```rust
// Precompute quaternion components
let r2 = r_rot * r_rot;
let ux2 = ux * ux;
let uy2 = uy * uy;
let uz2 = uz * uz;
let norm = r2 + ux2 + uy2 + uz2;

// Exact quaternion sandwich: v' = q * v * q⁻¹
let rot_x = (r2 + ux2 - uy2 - uz2) * t[0]
          + 2.0 * (ux * uy - r_rot * uz) * t[1]
          + 2.0 * (ux * uz + r_rot * uy) * t[2];

let rot_y = 2.0 * (ux * uy + r_rot * uz) * t[0]
          + (r2 - ux2 + uy2 - uz2) * t[1]
          + 2.0 * (uy * uz - r_rot * ux) * t[2];

let rot_z = 2.0 * (ux * uz - r_rot * uy) * t[0]
          + 2.0 * (uy * uz + r_rot * ux) * t[1]
          + (r2 - ux2 - uy2 + uz2) * t[2];

let w_out = t[3] * norm;
```

The translation terms (lines 77-80) remain **unchanged**:
```rust
rot_x + vx + r_rot * vx - ux * pw,
rot_y + vy + r_rot * vy - uy * pw,
rot_z + vz + r_rot * vz - uz * pw,
w_out,
```

## Emission Updates

### WGSL (`src/parser/emit.rs`)

**`sandwich_point`** (lines 15-24): Replace rotation math with exact formula, keep translation terms.

**`sandwich_plane`** (lines 20-28): Same rotation formula (planes rotate identically to points in dual PGA), no translation terms.

### CUDA (`src/parser/emit_cuda.rs`)

**`sandwich_point_cuda`** (lines 14-22): Mirror WGSL changes.

**`sandwich_plane_cuda`** (lines 24-31): Mirror WGSL changes.

## Contract Verification

| Contract | Current | New | Verification |
|----------|---------|-----|--------------|
| Branchless | ✓ | ✓ | No new `if`/`else` in arithmetic |
| SIMD `f32x4` | ✓ | ✓ | Same `from_array`/`to_array` usage |
| Zero alloc | ✓ | ✓ | No `Vec`/`String` in function |
| Exact arithmetic | ✗ | ✓ | Quaternion sandwich is exact |
| Multi-backend | ✓ | ✓ | Both emissions updated identically |

## Test Impact

Existing tests that will now pass with correct math:
- `quaternion_rotation_norm_preserved` — rotation magnitude preserved
- `agile_eye_pan_tilt_demo` — 2-axis rotation now correct
- `agile_eye_spherical_ik_demo` — 3-DoF chain + rotation correct

No test modifications needed — tests assert correctness properties that were previously failing silently.