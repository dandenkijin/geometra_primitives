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
            "fn sandwich_plane(m: Motor, target: vec4<f32>) -> vec4<f32> {\n    let t = target;\n    let d = m.dir;\n    let mo = m.mom;\n    let r_rot = d.x; let ux = d.y; let uy = d.z; let uz = d.w;\n    let vx = mo.x; let vy = mo.y; let vz = mo.z; let pw = mo.w;\n    let rot_x = r_rot * t.x + ux * t.y + uy * t.z + uz * t.w;\n    let rot_y = r_rot * t.y - ux * t.x + uy * t.w - uz * t.z;\n    let rot_z = r_rot * t.z - ux * t.w + uy * t.x - uz * t.y;\n    let w_out = t.w * (r_rot * r_rot - ux * ux - uy * uy - uz * uz);\n    return vec4<f32>(rot_x + vx + r_rot * vx - ux * pw, rot_y + vy + r_rot * vy - uy * pw, rot_z + vz + r_rot * vz - uz * pw, w_out);\n}".to_string()
        }
        PgaAst::IntersectPlanePoint(_, _) => {
            "fn intersect_plane_point(p: vec4<f32>, pt: vec4<f32>) -> f32 {\n    return p.x * pt.x + p.y * pt.y + p.z * pt.z + p.w * pt.w;\n}".to_string()
        }
        PgaAst::Chain(_) => {
            "fn motor_chain(chain: array<Motor>) -> Motor {\n    var result = Motor(vec4<f32>(1.0, 0.0, 0.0, 0.0), vec4<f32>(0.0, 0.0, 0.0, 0.0));\n    for (var i = 0u; i < arrayLength(&chain); i = i + 1u) {\n        let current = result;\n        let next = chain[i];\n        let r1 = current.dir.x; let r2 = next.dir.x;\n        let u1x = current.dir.y; let u1y = current.dir.z; let u1z = current.dir.w;\n        let u2x = next.dir.y; let u2y = next.dir.z; let u2z = next.dir.w;\n        let v1x = current.mom.x; let v1y = current.mom.y; let v1z = current.mom.z;\n        let v2x = next.mom.x; let v2y = next.mom.y; let v2z = next.mom.z;\n        let p1 = current.mom.w; let p2 = next.mom.w;\n        // Quaternion rotation product: r_new = r1*r2 - (u1x*u2x + u1y*u2y + u1z*u2z)\n        let r_new = r1 * r2 - (u1x * u2x + u1y * u2y + u1z * u2z);\n        let ux_new = r1 * u2x + r2 * u1x + (u1y * u2z - u1z * u2y);\n        let uy_new = r1 * u2y + r2 * u1y + (u1z * u2x - u1x * u2z);\n        let uz_new = r1 * u2z + r2 * u1z + (u1x * u2y - u1y * u2x);\n        // Translation coupling: geometric cross-terms with rotation effect\n        let vx_new = r1 * v2x + v1x * r2 + u1y * v2z - u1z * v2y;\n        let vy_new = r1 * v2y + v1y * r2 + u1z * v2x - u1x * v2z;\n        let vz_new = r1 * v2z + v1z * r2 + u1x * v2y - u1y * v2x;\n        let p_new = p1 + p2 + r1 * p2 - r2 * p1;\n        result.dir = vec4<f32>(r_new, ux_new, uy_new, uz_new);\n        result.mom = vec4<f32>(vx_new, vy_new, vz_new, p_new);\n    }\n    return result;\n}".to_string()
        }
    }
}
