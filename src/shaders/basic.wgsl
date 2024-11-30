struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    _padding: vec3<f32>,
    signal: f32,
};
@group(0) @binding(0) var<uniform> uniforms: Uniforms;


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

// Vertex shader
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color * uniforms.signal;
    out.clip_position = uniforms.view_proj * uniforms.model * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let corrected_color = inverse_gamma_correction(in.color);
    return vec4<f32>(corrected_color, 1.0);
}

fn gamma_correction(color: vec3<f32>) -> vec3<f32> {
    let gamma = 2.2;
    let gamma_corrected_color = vec3<f32>(
        pow(color.x, 1.0 / gamma),
        pow(color.y, 1.0 / gamma),
        pow(color.z, 1.0 / gamma)
    );
    return gamma_corrected_color;
}

fn inverse_gamma_correction(color: vec3<f32>) -> vec3<f32> {
    let gamma = 2.2;
    let expanded_color = vec3<f32>(
        pow(color.x, gamma),
        pow(color.y, gamma),
        pow(color.z, gamma)
    );
    return expanded_color;
}

// Convert from Linear to sRGB (Gamma correction)
fn linear_to_srgb(c: vec3<f32>) -> vec3<f32> {
    return pow(c, vec3<f32>(1.0 / 2.2));  // Linear to sRGB (gamma correction)
}