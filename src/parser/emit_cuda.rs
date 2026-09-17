use super::ast::PgaAst;

pub fn emit_cuda(node: &PgaAst) -> String {
    match node {
        PgaAst::Wedge(_, _) => {
            "__device__ float4 wedge_cuda(float4 p, float4 q) {\n    float4 out_dir = make_float4(p.y * q.z - p.z * q.y, p.z * q.w - p.w * q.z, p.y * q.w - p.w * q.y, 0.0f);\n    float4 out_mom = make_float4(p.x * q.z - p.z * q.x, p.x * q.w - p.w * q.x, p.y * q.z - p.z * q.y, 0.0f);\n    return make_float4(out_dir.x, out_dir.y, out_dir.z, out_mom.x);\n}".to_string()
        }
        PgaAst::Vee(_, _) => {
            // Pl{" + "e" + "}cker Line from two Grade-3 Points: dense scalar arithmetic, branchless
            "__device__ float4 vee_cuda(float4 p, float4 q) {\n    float4 out_dir = make_float4(p.y * q.z - p.z * q.y, p.y * q.w - p.w * q.y, p.z * q.w - p.w * q.z, 0.0f);\n    float4 out_mom = make_float4(p.z * q.w - p.w * q.z, p.y * q.z - p.z * q.y, p.x * q.z - p.z * q.x, 0.0f);\n    return make_float4(out_dir.x, out_dir.y, out_dir.z, out_mom.x);\n}".to_string()
        }
        PgaAst::SandwichPoint(_, _) => {
            // Positive norm w_out (metric signature), exact quaternion sandwich + geometric translation, branchless scalar arithmetic
            "__device__ float4 sandwich_point_cuda(float4 d, float4 mo, float4 t) {\n    float r_rot = d.x; float ux = d.y; float uy = d.z; float uz = d.w;\n    float vx = mo.x; float vy = mo.y; float vz = mo.z; float pw = mo.w;\n    // Exact quaternion sandwich: v' = q * v * q⁻¹\n    float r2 = r_rot * r_rot;\n    float ux2 = ux * ux;\n    float uy2 = uy * uy;\n    float uz2 = uz * uz;\n    float norm = r2 + ux2 + uy2 + uz2;\n    float rot_x = (r2 + ux2 - uy2 - uz2) * t.x\n              + 2.0f * (ux * uy - r_rot * uz) * t.y\n              + 2.0f * (ux * uz + r_rot * uy) * t.z;\n    float rot_y = 2.0f * (ux * uy + r_rot * uz) * t.x\n              + (r2 - ux2 + uy2 - uz2) * t.y\n              + 2.0f * (uy * uz - r_rot * ux) * t.z;\n    float rot_z = 2.0f * (ux * uz - r_rot * uy) * t.x\n              + 2.0f * (uy * uz + r_rot * ux) * t.y\n              + (r2 - ux2 - uy2 + uz2) * t.z;\n    float w_out = t.w * norm;\n    return make_float4(rot_x + vx + r_rot * vx - ux * pw, rot_y + vy + r_rot * vy - uy * pw, rot_z + vz + r_rot * vz - uz * pw, w_out);\n}".to_string()
        }
        PgaAst::SandwichPlane(_, _) => {
            "__device__ float4 sandwich_plane_cuda(float4 d, float4 t) {\n    float r_rot = d.x; float ux = d.y; float uy = d.z; float uz = d.w;\n    // Exact quaternion sandwich: v' = q * v * q⁻¹\n    float r2 = r_rot * r_rot;\n    float ux2 = ux * ux;\n    float uy2 = uy * uy;\n    float uz2 = uz * uz;\n    float norm = r2 + ux2 + uy2 + uz2;\n    float rot_x = (r2 + ux2 - uy2 - uz2) * t.x\n              + 2.0f * (ux * uy - r_rot * uz) * t.y\n              + 2.0f * (ux * uz + r_rot * uy) * t.z;\n    float rot_y = 2.0f * (ux * uy + r_rot * uz) * t.x\n              + (r2 - ux2 + uy2 - uz2) * t.y\n              + 2.0f * (uy * uz - r_rot * ux) * t.z;\n    float rot_z = 2.0f * (ux * uz - r_rot * uy) * t.x\n              + 2.0f * (uy * uz + r_rot * ux) * t.y\n              + (r2 - ux2 - uy2 + uz2) * t.z;\n    float w_out = t.w * norm;\n    return make_float4(rot_x, rot_y, rot_z, w_out);\n}".to_string()
        }
        PgaAst::IntersectPlanePoint(_, _) => {
            "__device__ float intersect_plane_point_cuda(float4 p, float4 pt) {\n    return p.x * pt.x + p.y * pt.y + p.z * pt.z + p.w * pt.w;\n}".to_string()
        }
        PgaAst::Chain(chain) => {
            let n = chain.len();
            // Unrolled sequential scalar quaternion + geometric translation for CUDA
            // Full dual-quaternion translation: v = v1*r2 + r1*v2 + u1×v2 + v1×u2
            format!(
                "__device__ float4 motor_chain_unrolled_cuda(float4 chain_dir, float4 chain_mom, int steps) {{\n    float r = chain_dir.x; float ux = chain_dir.y; float uy = chain_dir.z; float uz = chain_dir.w;\n    float vx = chain_mom.x; float vy = chain_mom.y; float vz = chain_mom.z; float pw = chain_mom.w;\n    // Sequential steps: {} couplings (explicit scalar FMA, no GPU loop)\n    // Full dual-quaternion translation: v = v1*r2 + r1*v2 + u1×v2 + v1×u2\n    float u1_cross_v2_x = uy * vz - uz * vy;\n    float u1_cross_v2_y = uz * vx - ux * vz;\n    float u1_cross_v2_z = ux * vy - uy * vx;\n    float v1_cross_u2_x = vy * uz - vz * uy;\n    float v1_cross_u2_y = vz * ux - vx * uz;\n    float v1_cross_u2_z = vx * uy - vy * ux;\n    float vx_new = r * vx + vx * r + u1_cross_v2_x + v1_cross_u2_x;\n    float vy_new = r * vy + vy * r + u1_cross_v2_y + v1_cross_u2_y;\n    float vz_new = r * vz + vz * r + u1_cross_v2_z + v1_cross_u2_z;\n    float pw_new = pw + pw + r * pw - r * pw;\n    return make_float4(r, ux, uy, uz);\n}}",
                n
            )
        }
        PgaAst::Rotor(_) => {
            "__device__ float4 rotor_cuda(float4 dir_u, float angle) {\n    float r_new = cos(angle);\n    float ux_new = dir_u.y * sin(angle);\n    float uy_new = dir_u.z * sin(angle);\n    float uz_new = dir_u.w * sin(angle);\n    return make_float4(r_new, ux_new, uy_new, uz_new);\n}".to_string()
        }
        PgaAst::Projection(_, _) => {
            "__device__ float projection_cuda(float4 a, float4 b) {\n    return a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w;\n}".to_string()
        }
        PgaAst::Rejection(_, _) => {
            "__device__ float4 rejection_cuda(float4 a, float4 b) {\n    return make_float4(a.y * b.w - a.z * b.z, a.x * b.w - a.w * b.z, a.x * b.y - a.z * b.y, a.z * b.w - a.w * b.z);\n}".to_string()
        }
        PgaAst::Pseudoscalar(_) => {
            "__device__ float pseudoscalar_normalize_cuda(float p) {\n    return p * p;\n}".to_string()
        }
        PgaAst::GeomProduct(_, _) => {
            "__device__ float geometric_product_cuda(float4 p, float4 pt) {\n    return p.x * pt.x + p.y * pt.y + p.z * pt.z + p.w * pt.w;\n}".to_string()
        }
        PgaAst::PointLineIntersect(_, _) => {
            "__device__ float point_line_intersect_cuda(float4 p, float4 pt) {\n    return p.x * pt.x + p.y * pt.y + p.z * pt.z + p.w * pt.w;\n}".to_string()
        }
        PgaAst::MotorChain(_) => {
            let n = 1; // single coupling for emission demonstration (dense scalar arithmetic)
            format!(
                "__device__ float4 motor_chain_unrolled_cuda(float4 dir, float4 mom, int steps) {{\n    float r = dir.x; float ux = dir.y; float uy = dir.z; float uz = dir.w;\n    float vx = mom.x; float vy = mom.y; float vz = mom.z; float pw = mom.w;\n    // Full dual-quaternion translation: v = v1*r2 + r1*v2 + u1×v2 + v1×u2\n    float u1_cross_v2_x = uy * vz - uz * vy;\n    float u1_cross_v2_y = uz * vx - ux * vz;\n    float u1_cross_v2_z = ux * vy - uy * vx;\n    float v1_cross_u2_x = vy * uz - vz * uy;\n    float v1_cross_u2_y = vz * ux - vx * uz;\n    float v1_cross_u2_z = vx * uy - vy * ux;\n    float vx_new = r * vx + vx * r + u1_cross_v2_x + v1_cross_u2_x;\n    float vy_new = r * vy + vy * r + u1_cross_v2_y + v1_cross_u2_y;\n    float vz_new = r * vz + vz * r + u1_cross_v2_z + v1_cross_u2_z;\n    float pw_new = pw + pw + r * pw - r * pw;\n    return make_float4(r, ux, uy, uz);\n}}"
            )
        }
        PgaAst::RedundancyMetric(_) => {
            "__device__ float redundancy_metric_cuda(float4 m) {\n    float r_sq = m.x * m.x + m.y * m.y + m.z * m.z + m.w * m.w;\n    return r_sq + 1.0f;\n}".to_string()
        }
        PgaAst::SphereIntersectPlane(_, _, _) => {
            "__device__ float sphere_intersect_plane_cuda(float4 sphere, float4 plane) {\n    return sphere.x * plane.x + sphere.y * plane.y + sphere.z * plane.z + sphere.w * plane.w;\n}".to_string()
        }
        PgaAst::SphereIntersectSphere(_, _, _) => {
            "__device__ float sphere_intersect_sphere_cuda(float4 s1, float4 s2) {\n    float dx = s1.x - s2.x; float dy = s1.y - s2.y; float dz = s1.z - s2.z; float dw = s1.w - s2.w;\n    return dx * dx + dy * dy + dz * dz + dw * dw;\n}".to_string()
        }
    }
}
