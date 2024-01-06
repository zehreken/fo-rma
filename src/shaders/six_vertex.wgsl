struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    var x = 0.0;
    var y = 0.0;
    switch in_vertex_index {
        case 0u: {
            x = -0.45;
            y = 0.45;
        }
        case 1u, 3u: {
            x = -0.45;
            y = -0.45;
        }
        case 2u, 5u: {
            x = 0.45;
            y = 0.45;
        }
        case 4u: {
            x = 0.45;
            y = -0.45;
        }
        default: {}
    }

    out.position = vec4<f32>(x, y, 0.0, 1.0);
    out.coord = vec2<f32>(x + 0.5, y + 0.5);
    return out;
}