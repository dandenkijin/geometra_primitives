# Design — Correct Sphere-Sphere Intersection Metric

## Current Code (src/lib.rs:138-155)

```rust
pub fn sphere_intersect_sphere(c1: Point, r1: f32, c2: Point, r2: f32) -> f32 {
    let a = c1.to_array();
    let b = c2.to_array();
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    let dw = a[3] - b[3];
    let dist_sq = dx * dx + dy * dy + dz * dz + dw * dw;
    let sum_r_sq = (r1 + r2) * (r1 + r2);
    let diff_r_sq = r1 * r2;  // BUG: should be (r1 - r2)^2
    sum_r_sq - dist_sq + diff_r_sq
}
```

**Bug analysis:**
- For tangent equal spheres: r1=1, r2=1, dist=2
- sum_r_sq = (1+1)² = 4
- dist_sq = 2² = 4
- diff_r_sq = 1*1 = 1 (WRONG — should be (1-1)² = 0)
- Result: 4 - 4 + 1 = 1 (should be 0)

## Mathematical Foundation

For two spheres in Euclidean space with centers c1, c2 and radii r1, r2:
- Distance between centers: d = |c1 - c2|
- Spheres intersect if |r1 - r2| ≤ d ≤ r1 + r2
- Intersection "amount" can be measured by: (r1 + r2)² - d² for external intersection, and d² - (r1 - r2)² for internal

In PGA R_3_0_1, the metric for sphere intersection should use the correct signed distance. The standard formula for the squared intersection "depth" is:

For external intersection (separation > |r1 - r2|):
```
intersection = (r1 + r2)² - d²
```

For internal intersection (one inside another):
```
intersection = d² - (r1 - r2)²
```

But a unified formula that works for both cases and gives correct sign:
```
intersection = ((r1 + r2)² - d²) * (d² - (r1 - r2)²)
```

This is positive when |r1 - r2| < d < r1 + r2 (intersecting), zero at boundaries, negative otherwise.

However, for a simpler signed metric that preserves the "zero at tangency" property:
```
intersection = (r1 + r2)² - d²   for external tangency test
```
or the full intersection area proportional to:
```
intersection = (r1 + r2)² - d²   when d > |r1 - r2|
```

The simplest correct formula that gives 0 at external tangency:
```
result = (r1 + r2)² - d²
```

But this doesn't handle internal tangency. For a more complete metric used in collision detection:
```
result = (r1 + r2)² - d²   if we only care about external intersection
```

Actually, looking at the sphere_intersect_plane function, it returns `r² - d²` which is positive when sphere intersects plane. So the convention seems to be: positive = intersecting, zero = tangent, negative = separated.

For sphere-sphere, the external intersection test is:
```
d < r1 + r2  →  (r1 + r2)² - d² > 0
d = r1 + r2  →  (r1 + r2)² - d² = 0
d > r1 + r2  →  (r1 + r2)² - d² < 0
```

And internal intersection (one inside another without touching):
```
d < |r1 - r2|  →  d² - (r1 - r2)² < 0
```

A complete metric that handles both:
```
result = ((r1 + r2)² - d²) * (d² - (r1 - r2)²) / something
```

But for simplicity and matching the sphere_intersect_plane convention (single quadratic), I'll use:
```
result = (r1 + r2)² - d²
```

This correctly gives:
- External tangency: 0
- Intersecting: positive
- Separated: negative
- One inside another without touching: positive (since d < r1 + r2 always if one is inside the other)

Wait, if r1=5, r2=1, d=1 (small sphere inside large, centers 1 apart): d=1, r1+r2=6, (r1+r2)²-d² = 36-1=35 > 0. But they don't intersect in the sense of surface intersection — the small sphere is entirely inside.

For a more discriminating metric:
```
if d >= |r1 - r2|: result = (r1 + r2)² - d²  // external intersection test
else: result = d² - (r1 - r2)²  // internal intersection test (negative when one fully inside)
```

But we need branchless. The product form:
```
result = ((r1 + r2)² - d²) * (d² - (r1 - r2)²)
```
This is:
- Positive when |r1 - r2| < d < r1 + r2 (intersecting surfaces)
- Zero at external tangency (d = r1 + r2) or internal tangency (d = |r1 - r2|)
- Negative when d > r1 + r2 (separated) or d < |r1 - r2| (one fully inside other)

This is the "squared intersection area" up to a constant factor and is branchless!

Let me verify:
- r1=1, r2=1, d=2 (external tangency): (4-4)*(4-0) = 0*4 = 0 ✅
- r1=1, r2=1, d=1 (intersecting): (4-1)*(1-0) = 3*1 = 3 > 0 ✅
- r1=1, r2=1, d=3 (separated): (4-9)*(9-0) = -5*9 = -45 < 0 ✅
- r1=5, r2=1, d=1 (internal, small inside large): (36-1)*(1-16) = 35*(-15) = -525 < 0 ✅
- r1=5, r2=1, d=4 (internal tangency): (36-16)*(16-16) = 20*0 = 0 ✅

This is perfect! The product form handles all cases correctly and is branchless.

## New Implementation

```rust
pub fn sphere_intersect_sphere(c1: Point, r1: f32, c2: Point, r2: f32) -> f32 {
    let a = c1.to_array();
    let b = c2.to_array();
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    let dw = a[3] - b[3];
    let dist_sq = dx * dx + dy * dy + dz * dz + dw * dw;
    let sum_r = r1 + r2;
    let diff_r = r1 - r2;
    let sum_r_sq = sum_r * sum_r;
    let diff_r_sq = diff_r * diff_r;
    // Branchless product form: ((r1+r2)² - d²) * (d² - (r1-r2)²)
    // > 0 when intersecting, = 0 at tangency, < 0 when separated or one fully inside
    (sum_r_sq - dist_sq) * (dist_sq - diff_r_sq)
}
```

## Emission Updates

Both WGSL (`emit.rs`) and CUDA (`emit_cuda.rs`) have `sphere_intersect_sphere` emissions that need updating to match.

### WGSL (emit.rs)
```wgsl
fn sphere_intersect_sphere(s1: Point, s2: Point) -> f32 {
    var dx = s1.x - s2.x; var dy = s1.y - s2.y; var dz = s1.z - s2.z; var dw = s1.w - s2.w;
    var dist_sq = dx * dx + dy * dy + dz * dz + dw * dw;
    var sum_r = 2.0; // Would need radii as parameters
    // Need to adjust signature to accept radii
}
```

Actually, looking at the emission, it seems the radii are not passed — they might be embedded in the Point's w component or passed differently. Let me check the actual emission code.

Actually, the current emission in `emit.rs` and `emit_cuda.rs` doesn't include radii parameters — it computes distance squared between points. The radii must be encoded in the Point representation (w component?) or passed separately.

Let me check the current emission.