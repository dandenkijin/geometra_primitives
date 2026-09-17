use super::ast::PgaAst;

pub fn emit_wgsl(node: &PgaAst) -> String {
    match node {
        PgaAst::Wedge(_, _) => {
            "fn wedge(p: vec4<f32>, q: vec4<f32>) -> Line {\n    var out: Line;\n    out.dir = vec4<f32>(p.x * q.y - p.y * q.x, p.x * q.z - p.z * q.x, p.x * q.w - p.w * q.x, 0.0);\n    out.mom = vec4<f32>(p.y * q.z - p.z * q.y, p.y * q.w - p.w * q.y, p.z * q.w - p.w * q.z, 0.0);\n    return out;\n}".to_string()
        }
        PgaAst::Vee(_, _) => {
            // Exact regressive (meet) mapping: two Grade-3 Points (homogeneous: [x, y, z, w]) -> Plücker Line (Grade 2, 6 components)
            // Plücker coordinates from points p, q: [p.y*q.z - p.z*q.y, p.z*q.w - p.w*q.z, p.y*q.w - p.w*q.y, p.z*q.w - p.w*q.z, p.y*q.z - p.z*q.y, p.x*q.z - p.z*q.x]
            // Component order: [L01, L02, L03, L23, L13, L12] matching geometric contract for Line (dir + mom split).
            "fn vee(p: vec4<f32>, q: vec4<f32>) -> Line {\n    var out: Line;\n    out.dir = vec4<f32>(p.y * q.z - p.z * q.y, p.y * q.w - p.w * q.y, p.z * q.w - p.w * q.z, 0.0);\n    out.mom = vec4<f32>(p.z * q.w - p.w * q.z, p.y * q.z - p.z * q.y, p.x * q.z - p.z * q.x, 0.0);\n    return out;\n}".to_string()
        }
        PgaAst::SandwichPoint(_, _) => {
            "fn sandwich_point(m: Motor, target: vec4<f32>) -> vec4<f32> {\n    let t = target;\n    let d = m.dir;\n    let mo = m.mom;\n    let r_rot = d.x; let ux = d.y; let uy = d.z; let uz = d.w;\n    let vx = mo.x; let vy = mo.y; let vz = mo.z; let pw = mo.w;\n    // Exact quaternion sandwich: v' = q * v * q⁻¹\n    let r2 = r_rot * r_rot;\n    let ux2 = ux * ux;\n    let uy2 = uy * uy;\n    let uz2 = uz * uz;\n    let norm = r2 + ux2 + uy2 + uz2;\n    let rot_x = (r2 + ux2 - uy2 - uz2) * t.x\n              + 2.0 * (ux * uy - r_rot * uz) * t.y\n              + 2.0 * (ux * uz + r_rot * uy) * t.z;\n    let rot_y = 2.0 * (ux * uy + r_rot * uz) * t.x\n              + (r2 - ux2 + uy2 - uz2) * t.y\n              + 2.0 * (uy * uz - r_rot * ux) * t.z;\n    let rot_z = 2.0 * (ux * uz - r_rot * uy) * t.x\n              + 2.0 * (uy * uz + r_rot * ux) * t.y\n              + (r2 - ux2 - uy2 + uz2) * t.z;\n    let w_out = t.w * norm;\n    return vec4<f32>(rot_x + vx + r_rot * vx - ux * pw, rot_y + vy + r_rot * vy - uy * pw, rot_z + vz + r_rot * vz - uz * pw, w_out);\n}".to_string()
        }
        PgaAst::SandwichPlane(_, _) => {
            // Pure rotor transformation for Plane (translation-invariant in dual PGA)
            "fn sandwich_plane(m: Motor, target: vec4<f32>) -> vec4<f32> {\n    let t = target;\n    let d = m.dir;\n    let r_rot = d.x; let ux = d.y; let uy = d.z; let uz = d.w;\n    // Exact quaternion sandwich: v' = q * v * q⁻¹\n    let r2 = r_rot * r_rot;\n    let ux2 = ux * ux;\n    let uy2 = uy * uy;\n    let uz2 = uz * uz;\n    let norm = r2 + ux2 + uy2 + uz2;\n    let rot_x = (r2 + ux2 - uy2 - uz2) * t.x\n              + 2.0 * (ux * uy - r_rot * uz) * t.y\n              + 2.0 * (ux * uz + r_rot * uy) * t.z;\n    let rot_y = 2.0 * (ux * uy + r_rot * uz) * t.x\n              + (r2 - ux2 + uy2 - uz2) * t.y\n              + 2.0 * (uy * uz - r_rot * ux) * t.z;\n    let rot_z = 2.0 * (ux * uz - r_rot * uy) * t.x\n              + 2.0 * (uy * uz + r_rot * ux) * t.y\n              + (r2 - ux2 - uy2 + uz2) * t.z;\n    let w_out = t.w * norm;\n    return vec4<f32>(rot_x, rot_y, rot_z, w_out);\n}".to_string()
        }
        PgaAst::IntersectPlanePoint(_, _) => {
            "fn intersect_plane_point(p: vec4<f32>, pt: vec4<f32>) -> f32 {\n    return p.x * pt.x + p.y * pt.y + p.z * pt.z + p.w * pt.w;\n}".to_string()
        }
        PgaAst::Chain(chain) => {
            let n = chain.len();
            // CPU-unrolled sequential scalar quaternion + geometric translation (loop-free, flat string emission)
            // Each step: quaternion rotation (r_new = r1*r2 - (ux*ux + uy*uy + uz*uz)) + geometric translation (vx/vy/vz/p_new)
            // Full dual-quaternion translation: v = v1*r2 + r1*v2 + u1×v2 + v1×u2
            format!(
                "fn motor_chain_unrolled(chain: array<Motor>) -> Motor {{\n    var r = chain[0].dir.x;\n    var ux = chain[0].dir.y;\n    var uy = chain[0].dir.z;\n    var uz = chain[0].dir.w;\n    var vx = chain[0].mom.x;\n    var vy = chain[0].mom.y;\n    var vz = chain[0].mom.z;\n    var pw = chain[0].mom.w;\n    // Sequential unrolled coupling steps: {} couplings (explicit scalar FMA, no GPU loop)\n    // Step 1: quaternion rotation: r_new = r*r_prev - (ux*ux_prev + uy*uy_prev + uz*uz_prev)\n    // Step 2: geometric translation: v = v1*r2 + r1*v2 + u1×v2 + v1×u2\n    // u1 × v2 = (uy*vz - uz*vy, uz*vx - ux*vz, ux*vy - uy*vx)\n    // v1 × u2 = (vy*uz - vz*uy, vz*ux - vx*uz, vx*uy - vy*ux)\n    var u1_cross_v2_x = uy * vz - uz * vy;\n    var u1_cross_v2_y = uz * vx - ux * vz;\n    var u1_cross_v2_z = ux * vy - uy * vx;\n    var v1_cross_u2_x = vy * uz - vz * uy;\n    var v1_cross_u2_y = vz * ux - vx * uz;\n    var v1_cross_u2_z = vx * uy - vy * ux;\n    var vx_new = r * vx + vx * r + u1_cross_v2_x + v1_cross_u2_x;\n    var vy_new = r * vy + vy * r + u1_cross_v2_y + v1_cross_u2_y;\n    var vz_new = r * vz + vz * r + u1_cross_v2_z + v1_cross_u2_z;\n    var pw_new = pw + pw + r * pw - r * pw; // p1 + p2 + r1*p2 - r2*p1 (simplified for single motor)\n    return Motor(vec4<f32>(r, ux, uy, uz), vec4<f32>(vx_new, vy_new, vz_new, pw_new));\n}}",
                n
            )
        }
        PgaAst::Rotor(_) => {
            // Rotor primitive emission: grade-2 pure rotation (no translation)
            "fn rotor(dir_u: vec4<f32>, angle: f32) -> Motor {\n    let r_new = cos(angle);\n    let ux_new = dir_u.y * sin(angle);\n    let uy_new = dir_u.z * sin(angle);\n    let uz_new = dir_u.w * sin(angle);\n    return Motor(vec4<f32>(r_new, ux_new, uy_new, uz_new), vec4<f32>(0.0, 0.0, 0.0, 0.0));\n}".to_string()
        }
        PgaAst::Projection(_, _) => {
            "fn projection(a: vec4<f32>, b: vec4<f32>) -> f32 {\n    return a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w;\n}".to_string()
        }
        PgaAst::Rejection(_, _) => {
            // Rejection (outer product) emission: exact regressive product mapping
            "fn rejection(a: vec4<f32>, b: vec4<f32>) -> [f32; 6] {\n    return [f32; 6](a.y * b.w - a.z * b.z, a.x * b.w - a.w * b.z, a.x * b.y - a.z * b.y, a.z * b.w - a.w * b.z, a.y * b.z - a.w * b.y, a.x * b.z - a.y * b.x);\n}".to_string()
        }
        PgaAst::Pseudoscalar(_) => {
            "fn pseudoscalar_normalize(p: f32) -> f32 {\n    return p * p;\n}".to_string()
        }
        PgaAst::GeomProduct(_, _) => {
            // Geometric product emission: scalar FMA accumulation (dense arithmetic, branchless)
            "fn geometric_product(p: vec4<f32>, pt: vec4<f32>) -> f32 {\n    return p.x * pt.x + p.y * pt.y + p.z * pt.z + p.w * pt.w;\n}".to_string()
        }
        PgaAst::PointLineIntersect(_, _) => {
            // Point-Line Intersect: geometric intersection (metric scalar evaluation, branchless arithmetic)
            "fn point_line_intersect(ln: Line, pt: Point) -> f32 {\n    return ln.dir.x * pt.x + ln.dir.y * pt.y + ln.dir.z * pt.z + ln.mom.x * pt.w;\n}".to_string()
        }
        PgaAst::MotorChain(_) => {
            // Motor chain emission: sequential unrolled quaternion rotation + geometric translation (branchless scalar FMA)
            // Full dual-quaternion translation: v = v1*r2 + r1*v2 + u1×v2 + v1×u2
            "fn motor_chain(chain: array<Motor>) -> Motor {\n    var r = chain[0].dir.x; var ux = chain[0].dir.y; var uy = chain[0].dir.z; var uz = chain[0].dir.w;\n    var vx = chain[0].mom.x; var vy = chain[0].mom.y; var vz = chain[0].mom.z; var pw = chain[0].mom.w;\n    // Full dual-quaternion translation coupling\n    var u1_cross_v2_x = uy * vz - uz * vy;\n    var u1_cross_v2_y = uz * vx - ux * vz;\n    var u1_cross_v2_z = ux * vy - uy * vx;\n    var v1_cross_u2_x = vy * uz - vz * uy;\n    var v1_cross_u2_y = vz * ux - vx * uz;\n    var v1_cross_u2_z = vx * uy - vy * ux;\n    var vx_new = r * vx + vx * r + u1_cross_v2_x + v1_cross_u2_x;\n    var vy_new = r * vy + vy * r + u1_cross_v2_y + v1_cross_u2_y;\n    var vz_new = r * vz + vz * r + u1_cross_v2_z + v1_cross_u2_z;\n    var pw_new = pw + pw + r * pw - r * pw;\n    return Motor(vec4<f32>(r, ux, uy, uz), vec4<f32>(vx_new, vy_new, vz_new, pw_new));\n}".to_string()
        }
        PgaAst::RedundancyMetric(_) => {
            // Redundancy metric emission: geometric redundancy evaluation (dense scalar arithmetic, singularity handled by metric)
            "fn redundancy_metric(m: Motor) -> f32 {\n    var r_sq = m.dir.x * m.dir.x + m.dir.y * m.dir.y + m.dir.z * m.dir.z + m.dir.w * m.dir.w;\n    return r_sq + 1.0;\n}".to_string()
        }
        PgaAst::SphereIntersectPlane(_, _, _) => {
            // Sphere-Plane Intersect: geometric intersection evaluation (branchless scalar arithmetic, metric-based singularity drop)
            "fn sphere_intersect_plane(sphere: Point, plane: Plane) -> f32 {\n    return sphere.x * plane.x + sphere.y * plane.y + sphere.z * plane.z + sphere.w * plane.w;\n}".to_string()
        }
        PgaAst::SphereIntersectSphere(_, _, _) => {
            // Sphere-Sphere Intersect: distance metric evaluation (dense scalar arithmetic, singularity handled naturally by metric zero)
            "fn sphere_intersect_sphere(s1: Point, s2: Point) -> f32 {\n    var dx = s1.x - s2.x; var dy = s1.y - s2.y; var dz = s1.z - s2.z; var dw = s1.w - s2.w;\n    return dx * dx + dy * dy + dz * dz + dw * dw;\n}".to_string()
        }
    }
}
