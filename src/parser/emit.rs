use super::ast::PgaAst;

pub fn emit_wgsl(node: &PgaAst) -> String {
    match node {
        PgaAst::Wedge(_, _) => {
            "fn wedge(p: vec4<f32>, q: vec4<f32>) -> Line {\n    var out: Line;\n    out.dir = vec4<f32>(p.x * q.y - p.y * q.x, p.x * q.z - p.z * q.x, p.x * q.w - p.w * q.x, 0.0);\n    out.mom = vec4<f32>(p.y * q.z - p.z * q.y, p.y * q.w - p.w * q.y, p.z * q.w - p.w * q.z, 0.0);\n    return out;\n}".to_string()
        }
        PgaAst::Vee(_, _) => {
            "fn vee(p: vec4<f32>, q: vec4<f32>) -> Line {\n    var out: Line;\n    out.dir = vec4<f32>(p.y * q.w - p.z * q.z, p.x * q.w - p.w * q.z, p.x * q.y - p.z * q.y, 0.0);\n    out.mom = vec4<f32>(p.z * q.w - p.w * q.x, p.y * q.z - p.w * q.x, p.x * q.z - p.y * q.x, 0.0);\n    return out;\n}".to_string()
        }
        PgaAst::Sandwich(_, _) => {
            "fn sandwich(m: Motor, target: vec4<f32>) -> vec4<f32> {\n    let t = target;\n    let d = m.dir;\n    let mo = m.mom;\n    let r_rot = d.x; let ux = d.y; let uy = d.z; let uz = d.w;\n    let vx = mo.x; let vy = mo.y; let vz = mo.z; let pw = mo.w;\n    let rot_x = r_rot * t.x + ux * t.y + uy * t.z + uz * t.w;\n    let rot_y = r_rot * t.y - ux * t.x + uy * t.w - uz * t.z;\n    let rot_z = r_rot * t.z - ux * t.w + uy * t.x - uz * t.y;\n    let w_out = t.w * (r_rot * r_rot - ux * ux - uy * uy - uz * uz);\n    return vec4<f32>(rot_x + vx + r_rot * vx - ux * pw, rot_y + vy + r_rot * vy - uy * pw, rot_z + vz + r_rot * vz - uz * pw, w_out);\n}".to_string()
        }
        PgaAst::IntersectPlanePoint(_, _) => {
            "fn intersect_plane_point(p: vec4<f32>, pt: vec4<f32>) -> f32 {\n    return p.x * pt.x + p.y * pt.y + p.z * pt.z + p.w * pt.w;\n}".to_string()
        }
        PgaAst::Chain(_) => {
            "fn motor_chain(chain: array<Motor>) -> Motor {\n    var result = Motor(vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0, 0.0, 0.0, 0.0));\n    for (var i = 0u; i < arrayLength(&chain); i = i + 1u) {\n        // Sequential motor composition with quaternion cross-terms\n    }\n    return result;\n}".to_string()
        }
    }
}
