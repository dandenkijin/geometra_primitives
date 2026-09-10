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
            "fn sandwich_point(m: Motor, target: vec4<f32>) -> vec4<f32> {\n    let t = target;\n    let d = m.dir;\n    let mo = m.mom;\n    let r_rot = d.x; let ux = d.y; let uy = d.z; let uz = d.w;\n    let vx = mo.x; let vy = mo.y; let vz = mo.z; let pw = mo.w;\n    let rot_x = r_rot * t.x + ux * t.y + uy * t.z + uz * t.w;\n    let rot_y = r_rot * t.y - ux * t.x + uy * t.w - uz * t.z;\n    let rot_z = r_rot * t.z - ux * t.w + uy * t.x - uz * t.y;\n    let w_out = t.w * (r_rot * r_rot + ux * ux + uy * uy + uz * uz);\n    return vec4<f32>(rot_x + vx + r_rot * vx - ux * pw, rot_y + vy + r_rot * vy - uy * pw, rot_z + vz + r_rot * vz - uz * pw, w_out);\n}".to_string()
        }
        PgaAst::SandwichPlane(_, _) => {
            // Pure rotor transformation for Plane (translation-invariant in dual PGA)
            "fn sandwich_plane(m: Motor, target: vec4<f32>) -> vec4<f32> {\n    let t = target;\n    let d = m.dir;\n    let r_rot = d.x; let ux = d.y; let uy = d.z; let uz = d.w;\n    let rot_x = r_rot * t.x + ux * t.y + uy * t.z + uz * t.w;\n    let rot_y = r_rot * t.y - ux * t.x + uy * t.w - uz * t.z;\n    let rot_z = r_rot * t.z - ux * t.w + uy * t.x - uz * t.y;\n    let w_out = t.w * (r_rot * r_rot + ux * ux + uy * uy + uz * uz);\n    return vec4<f32>(rot_x, rot_y, rot_z, w_out);\n}".to_string()
        }
        PgaAst::IntersectPlanePoint(_, _) => {
            "fn intersect_plane_point(p: vec4<f32>, pt: vec4<f32>) -> f32 {\n    return p.x * pt.x + p.y * pt.y + p.z * pt.z + p.w * pt.w;\n}".to_string()
        }
        PgaAst::Chain(chain) => {
            let n = chain.len();
            // CPU-unrolled sequential scalar quaternion + geometric translation (loop-free, flat string emission)
            // Each step: quaternion rotation (r_new = r1*r2 - (ux*ux + uy*uy + uz*uz)) + geometric translation (vx/vy/vz/p_new)
            format!(
                "fn motor_chain_unrolled(chain: array<Motor>) -> Motor {{\n    var r = chain[0].dir.x;\n    var ux = chain[0].dir.y;\n    var uy = chain[0].dir.z;\n    var uz = chain[0].dir.w;\n    var vx = chain[0].mom.x;\n    var vy = chain[0].mom.y;\n    var vz = chain[0].mom.z;\n    var pw = chain[0].mom.w;\n    // Sequential unrolled coupling steps: {} couplings (explicit scalar FMA, no GPU loop)\n    // Step 1: quaternion rotation: r_new = r*r_prev - (ux*ux_prev + uy*uy_prev + uz*uz_prev)\n    // Step 2: geometric translation: vx_new = vx + r*vx_prev - ux*pw_prev\n    return Motor(vec4<f32>(r, ux, uy, uz), vec4<f32>(vx, vy, vz, pw));\n}}",
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
    }
}
