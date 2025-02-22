struct Object {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    normal: mat3x3<f32>,
};
@group(0) @binding(0) var<uniform> object: Object;

struct Light {
    position: vec4<f32>,
    color: vec4<f32>,
};
@group(1) @binding(0) var<uniform> ligth: Light;

struct Material {
    color1: vec4<f32>,
    color2: vec4<f32>,
    color3: vec4<f32>,
    signal: f32,
};
@group(2) @binding(0) var<uniform> material: Material;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) color1: vec4<f32>,
    @location(2) color2: vec4<f32>,
    @location(3) color3: vec4<f32>,
    @location(4) uv: vec2<f32>,
    @location(5) signal: f32,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_position = object.model * vec4<f32>(model.position, 1.0);
    out.clip_position = object.view_proj * world_position;
    out.world_position = world_position.xyz / world_position.w;
    out.color1 = material.color1;
    out.color2 = material.color2;
    out.color3 = material.color3;
    out.uv = model.uv;
    out.signal = material.signal;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let bottom = srgb_to_linear(in.color1.rgb);
    let middle = srgb_to_linear(in.color2.rgb);
    let top = srgb_to_linear(in.color3.rgb);

    var base = vec3(0.1, 0.1, 0.1);
    var color = mix(middle, bottom, step(uv.y, 0.5));
    color = mix(top, color, step(uv.y, 0.75));

    let step_size = 0.125;
    let quantized_signal = floor(in.signal / step_size + 0.5) * step_size;
    color = mix(base, color, step(uv.y, quantized_signal));

    return vec4<f32>(color, 1.0);
}