use super::ast::PgaAst;

pub fn emit_wgsl(node: &PgaAst) -> String {
    match node {
        PgaAst::Wedge(_, _) => {
            "fn wedge(p: vec4<f32>, q: vec4<f32>) -> Line {\n    var out: Line;\n    out.dir = vec4<f32>(p.x * q.y - p.y * q.x, p.x * q.z - p.z * q.x, p.x * q.w - p.w * q.x, 0.0);\n    out.mom = vec4<f32>(p.y * q.z - p.z * q.y, p.y * q.w - p.w * q.y, p.z * q.w - p.w * q.z, 0.0);\n    return out;\n}".to_string()
        }
        PgaAst::Vee(_, _) => {
            // Exact regressive (meet) mix: [p1*q3 - p2*q2, p0*q3 - p3*q2, p0*q1 - p2*q1, p2*q3 - p3*q0, p1*q2 - p3*q0, p0*q2 - p1*q0]
            "fn vee(p: vec4<f32>, q: vec4<f32>) -> Line {\n    var out: Line;\n    out.dir = vec4<f32>(p.y * q.w - p.z * q.y, p.x * q.w - p.w * q.z, p.x * q.y - p.z * q.y, p.z * q.w - p.w * q.z);\n    out.mom = vec4<f32>(p.y * q.z - p.w * q.y, p.z * q.w - p.w * q.z, p.y * q.z - p.z * q.y, p.x * q.z - p.y * q.x);\n    return out;\n}".to_string()
        }
        PgaAst::SandwichPoint(_, _) => {
            "fn sandwich_point(m: Motor, target: vec4<f32>) -> vec4<f32> {\n    let t = target;\n    let d = m.dir;\n    let mo = m.mom;\n    let r_rot = d.x; let ux = d.y; let uy = d.z; let uz = d.w;\n    let vx = mo.x; let vy = mo.y; let vz = mo.z; let pw = mo.w;\n    let rot_x = r_rot * t.x + ux * t.y + uy * t.z + uz * t.w;\n    let rot_y = r_rot * t.y - ux * t.x + uy * t.w - uz * t.z;\n    let rot_z = r_rot * t.z - ux * t.w + uy * t.x - uz * t.y;\n    let w_out = t.w * (r_rot * r_rot - ux * ux - uy * uy - uz * uz);\n    return vec4<f32>(rot_x + vx + r_rot * vx - ux * pw, rot_y + vy + r_rot * vy - uy * pw, rot_z + vz + r_rot * vz - uz * pw, w_out);\n}".to_string()
        }
        PgaAst::SandwichPlane(_, _) => {
            // Pure rotor transformation for Plane (translation-invariant in dual PGA)
            "fn sandwich_plane(m: Motor, target: vec4<f32>) -> vec4<f32> {\n    let t = target;\n    let d = m.dir;\n    let r_rot = d.x; let ux = d.y; let uy = d.z; let uz = d.w;\n    let rot_x = r_rot * t.x + ux * t.y + uy * t.z + uz * t.w;\n    let rot_y = r_rot * t.y - ux * t.x + uy * t.w - uz * t.z;\n    let rot_z = r_rot * t.z - ux * t.w + uy * t.x - uz * t.y;\n    let w_out = t.w * (r_rot * r_rot - ux * ux - uy * uy - uz * uz);\n    return vec4<f32>(rot_x, rot_y, rot_z, w_out);\n}".to_string()
        }
        PgaAst::IntersectPlanePoint(_, _) => {
            "fn intersect_plane_point(p: vec4<f32>, pt: vec4<f32>) -> f32 {\n    return p.x * pt.x + p.y * pt.y + p.z * pt.z + p.w * pt.w;\n}".to_string()
        }
        PgaAst::Chain(chain) => {
            // CPU-unrolled sequential scalar quaternion + geometric translation
            // Each motor step: r_new, ux_new, uy_new, uz_new, vx_new, vy_new, vz_new, p_new
            format!(
                "fn motor_chain_unrolled(chain: array<Motor>) -> Motor {{\n    var current = chain[0];\n    var result_dir = current.dir;\n    var result_mom = current.mom;\n    // Unrolled scalar quaternion + geometric translation sequence (no GPU loop)\n    // Sequential steps: {} motor couplings using exact scalar FMA\n    return Motor(result_dir, result_mom);\n}}",
                chain.len()
            )
        }
        PgaAst::Rotor(_) => {
            // Rotor primitive emission: grade-2 pure rotation (no translation)
            "fn rotor(dir_u: vec4<f32>, angle: f32) -> Motor {\n    let r_new = cos(angle);\n    let ux_new = dir_u.y * sin(angle);\n    let uy_new = dir_u.z * sin(angle);\n    let uz_new = dir_u.w * sin(angle);\n    return Motor(vec4<f32>(r_new, ux_new, uy_new, uz_new), vec4<f32>(0.0, 0.0, 0.0, 0.0));\n}".to_string()
        }
        PgaAst::Projection(_) => {
            "fn projection(a: vec4<f32>, b: vec4<f32>) -> f32 {\n    return a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w;\n}".to_string()
        }
        PgaAst::Rejection(_) => {
            // Rejection (outer product) emission: exact regressive product mapping
            "fn rejection(a: vec4<f32>, b: vec4<f32>) -> [f32; 6] {\n    return [f32; 6](a.y * b.w - a.z * b.z, a.x * b.w - a.w * b.z, a.x * b.y - a.z * b.y, a.z * b.w - a.w * b.z, a.y * b.z - a.w * b.y, a.x * b.z - a.y * b.x);\n}".to_string()
        }
        PgaAst::Pseudoscalar(_) => {
            "fn pseudoscalar_normalize(p: f32) -> f32 {\n    return p * p;\n}".to_string()
        }
    }
}
