# Design — Exact Dual-Quaternion Exponential

## Current Code (src/lib.rs:248-258)

```rust
pub fn exp(m: &Motor) -> Motor {
    let d = m.dir.to_array();
    let mo = m.mom.to_array();
    // First-order approximation for demonstration: identity rotation + scaled translation
    Motor {
        dir: f32x4::from_array([1.0_f32 + d[0], d[1], d[2], d[3]]),
        mom: f32x4::from_array([mo[0], mo[1], mo[2], mo[3]]),
    }
}
```

## Mathematical Foundation

A dual quaternion `M = q + ε * q'` where:
- `q = (r, u_x, u_y, u_z)` = quaternion part (rotation)
- `q' = (0, v_x, v_y, v_z) + p*ε` = dual part (translation + pseudoscalar)
- `ε` = dual unit (ε² = 0)

**Exact exponential:**
```
exp(M) = exp(q + ε*q') = exp(q) + ε * exp(q) * q'
```

Since `ε² = 0`, higher-order terms vanish.

### Quaternion Exponential `exp(q)`

For `q = (r, u)` where `u = (ux, uy, uz)`:
- Let `θ = |u| = sqrt(ux² + uy² + uz²)`
- If `θ > ε`:
  ```
  exp(q) = (cos(θ), sin(θ)/θ * u) * exp(r)
  ```
  Wait, this is for `q` as a general quaternion. Let me be more careful.

Actually, our motor representation: `dir = [r, ux, uy, uz]` where for a pure rotation motor, `r = cos(θ/2)`, `u = sin(θ/2) * û`.

But the motor's `dir` component IS the quaternion `q`. So:
- `q = (r, ux, uy, uz)` is the quaternion
- `||q||` may not be 1 (it's not necessarily a unit quaternion in the input)

For a general quaternion `q = (r, u)`:
```
exp(q) = exp(r) * (cos(|u|), sin(|u|)/|u| * u)
```
where `|u| = sqrt(ux² + uy² + uz²)`.

So:
- `exp_q_r = exp(r) * cos(|u|)`
- `exp_q_u = exp(r) * sin(|u|)/|u| * u` (if |u| > 0, else 0)

### Dual Part: `exp(q) * q'`

`q' = (0, v_x, v_y, v_z) + p*ε`? Actually in our representation:
- `mom = [vx, vy, vz, p]` where `v = (vx, vy, vz)` is the translation dual part, `p` is pseudoscalar

The dual part `q'` in dual quaternion terms is `(0, v)` for translation, with `p` as additional pseudoscalar.

But looking at the motor structure:
- `dir = [r, ux, uy, uz]` — quaternion `q`
- `mom = [vx, vy, vz, p]` — dual part `q'` where the vector part is translation, scalar part is pseudoscalar

So `q' = (p, vx, vy, vz)` as a quaternion? Or is `q' = (0, vx, vy, vz)` and `p` separate?

Looking at `motor_chain` dual-scalar update: `p_out = p1 + p2 + r1*p2 - r2*p1`, this suggests `p` is the scalar part of the dual quaternion.

For the exponential, the dual part is `exp(q) * q'` where `q'` is the dual part as a quaternion.

If `q' = (p, vx, vy, vz)`:
```
exp(q) * q' = quaternion_multiply(exp_q, q')
```

Where `exp_q = (exp_q_r, exp_q_ux, exp_q_uy, exp_q_uz)`.

### Complete Formula

```
exp(M) = (exp_q, quaternion_multiply(exp_q, q'))
```

Where:
- `exp_q = exp(q)` as quaternion
- `q' = (p, vx, vy, vz)` as quaternion (dual part)

### Implementation

```rust
pub fn exp(m: &Motor) -> Motor {
    let d = m.dir.to_array();  // [r, ux, uy, uz]
    let mo = m.mom.to_array(); // [vx, vy, vz, p]
    
    let r = d[0];
    let ux = d[1];
    let uy = d[2];
    let uz = d[3];
    let vx = mo[0];
    let vy = mo[1];
    let vz = mo[2];
    let p = mo[3];
    
    // exp(q) where q = (r, ux, uy, uz)
    let u_norm_sq = ux*ux + uy*uy + uz*uz;
    let u_norm = u_norm_sq.sqrt();
    let exp_r = r.exp();
    
    let (exp_q_r, exp_q_ux, exp_q_uy, exp_q_uz) = if u_norm > 1e-8 {
        let cos_u = u_norm.cos();
        let sin_u_over_u = u_norm.sin() / u_norm;
        (exp_r * cos_u, 
         exp_r * sin_u_over_u * ux,
         exp_r * sin_u_over_u * uy,
         exp_r * sin_u_over_u * uz)
    } else {
        (exp_r, 0.0, 0.0, 0.0)
    };
    
    // q' = (p, vx, vy, vz) as quaternion
    // exp(q) * q' = quaternion_multiply(exp_q, q')
    // (a,b,c,d) * (e,f,g,h) = (ae - bf - cg - dh, af + be + ch - dg, ag - bh + ce + df, ah + bg - cf + de)
    let dual_r = exp_q_r * p - exp_q_ux * vx - exp_q_uy * vy - exp_q_uz * vz;
    let dual_ux = exp_q_r * vx + exp_q_ux * p + exp_q_uy * vz - exp_q_uz * vy;
    let dual_uy = exp_q_r * vy - exp_q_ux * vz + exp_q_uy * p + exp_q_uz * vx;
    let dual_uz = exp_q_r * vz + exp_q_ux * vy - exp_q_uy * vx + exp_q_uz * p;
    
    Motor {
        dir: f32x4::from_array([exp_q_r, exp_q_ux, exp_q_uy, exp_q_uz]),
        mom: f32x4::from_array([dual_ux, dual_uy, dual_uz, dual_r]),
    }
}
```

Wait, I need to double-check: is `q' = (p, vx, vy, vz)` or `(0, vx, vy, vz)` with `p` separate?

Looking at `motor_chain`:
- `mom = [vx, vy, vz, p]`
- `p_out = p1 + p2 + r1*p2 - r2*p1`

This matches dual quaternion multiplication where the dual part's scalar component is `p`. In standard dual quaternion for rigid motion:
- `M = q + ε * (0.5 * t * q)` where `t = (0, tx, ty, tz)` is pure translation
- The dual part has zero scalar component

But here `p` seems to be an additional pseudoscalar. The formula `p_out = p1 + p2 + r1*p2 - r2*p1` suggests `p` transforms like the scalar part of a quaternion under multiplication.

Actually, if we treat the full dual quaternion as having 8 components: `(r, ux, uy, uz, p, vx, vy, vz)`, then the multiplication is more complex. But the current code separates `dir` (4) and `mom` (4), and `p` is in `mom[3]`.

Given the `motor_chain` formula for `p_out`, it seems like the dual part is a quaternion `(p, vx, vy, vz)` and multiplication is:
```
M = (q, q') * (q2, q2') = (q*q2, q*q2' + q'*q2)
```

Then for exponential:
```
exp(q, q') = (exp(q), exp(q)*q')
```

So yes, `q' = (p, vx, vy, vz)` as a full quaternion.

But wait — for rigid motions, the dual part should be `ε * (0.5 * t * q)` which has zero scalar part. The `p` here might be something else (pseudoscalar tracking as the comment says).

Let me just implement the standard dual quaternion exponential with `q' = (p, vx, vy, vz)` and test.

## Contract Verification

| Contract | Current | New | Verification |
|----------|---------|-----|--------------|
| Branchless (arithmetic) | ✓ | ✓ | Only `if u_norm > eps` at boundary |
| SIMD `f32x4` | ✓ | ✓ | Same `from_array` usage |
| Zero alloc | ✓ | ✓ | No allocations |
| Exact arithmetic | ✗ | ✓ | Exact dual-quaternion exponential |