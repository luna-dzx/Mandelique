@vertex

struct VertexOutput {
    @builtin(position) vert_pos: vec4<f32>,
    @location(0) vert_color: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;

    let i = i32(in_vertex_index);

    let x = f32(i - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);

    if (i == 0) {
        out.vert_color = vec3<f32>(1.0,0.0,0.0);
    }
    if (i == 1) {
        out.vert_color = vec3<f32>(0.0,1.0,0.0);
    }
    if (i == 2) {
        out.vert_color = vec3<f32>(0.0,0.0,1.0);
    }


    out.vert_pos = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}


@fragment
fn fs_main(@location(0) in_vert_color: vec3<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(in_vert_color, 1.0);
}
