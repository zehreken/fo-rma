struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    color1: vec4<f32>,
    color2: vec4<f32>,
    color3: vec4<f32>,
    signal: f32,
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

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
    let world_position = uniforms.model * vec4<f32>(model.position, 1.0);
    out.clip_position = uniforms.view_proj * world_position;
    out.world_position = world_position.xyz / world_position.w;
    out.color1 = uniforms.color1;
    out.color2 = uniforms.color2;
    out.color3 = uniforms.color3;
    out.uv = model.uv;
    out.signal = uniforms.signal;

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
    color = mix(base, color, step(uv.y, in.signal));

    return vec4<f32>(color, 1.0);
}