@vertex

struct VertexOutput {
    @builtin(position) vert_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOutput {
    var out: VertexOutput;

    if (i==0) {
      out.vert_pos = vec4<f32>(-1.0, -1.0, 0.0, 1.0);
      out.uv = vec2<f32>(0.0);
    }
    if (i==2) {
      out.vert_pos = vec4<f32>(3.0, -1.0, 0.0, 1.0);
      out.uv = vec2<f32>(2.0,0.0);
    }
    if (i==1) {
      out.vert_pos = vec4<f32>(-1.0, 3.0, 0.0, 1.0);
      out.uv = vec2<f32>(0.0,2.0);
    }

    return out;
}



fn sdf(p: vec3<f32>) -> f32 {
    return length(p) - 0.5;
}

fn trace(ro: vec3<f32>, rd: vec3<f32>) -> f32 {
    var d = f32(0.0);
    for (var i: i32 = 0; i < 200; i+=1) {
        let pos = ro + rd * d;
        let delta = sdf(pos);
        d += delta;

        if (delta < 0.001) {
            break;
        }

        if (d > 100.0) {
            d = -1.0;
            break;
        }
    }
    return d;
}

fn get_color(uv: vec2<f32>) -> vec3<f32> {
    let ro = vec3<f32>(0.0, 0.0, -3.0);
    let rd = normalize(vec3<f32>(uv, 1.2));

    let d = trace(ro, rd);
    if (d < 0.0) {
        return vec3<f32>(0.0);
    }

    return vec3<f32>(1.0,0.0,0.0);
}


@fragment
fn fs_main(@location(0) in_uv: vec2<f32>) -> @location(0) vec4<f32> {
    let uv = in_uv * 2.0 - 1.0;

    return vec4<f32>(get_color(uv), 1.0);
}
