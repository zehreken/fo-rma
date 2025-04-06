struct Object {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    normal: mat3x3<f32>,
};
@group(0) @binding(0)
var<uniform> object: Object;

struct Material {
    color1: vec4<f32>,
    color2: vec4<f32>,
    color3: vec4<f32>,
    signal: f32,
};
@group(1) @binding(0)
var<uniform> material: Material;

@group(2) @binding(0)
var my_texture: texture_1d<f32>;
@group(2) @binding(1)
var my_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput; 
    let world_position = object.model * vec4<f32>(model.position, 1.0);
    out.clip_position = object.view_proj * world_position;
    out.uv = model.uv;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let t = clamp(uv.x, 0.0, 1.0); // not really necessary
    let wave = textureSample(my_texture, my_sampler, t);
    let r = wave.r;
    let g = wave.g;
    let b = wave.b;
    let y_centered = r * 0.25 + 0.5; // from [-1..1] to [0.25..0.75]

    let line_thickness = 0.1;
    let distance = abs(uv.y - y_centered);
    // let alpha = smoothstep(line_thickness, 0.0, distance);
    let alpha = step(distance, line_thickness);

    let c = mix(material.color2, material.color3, alpha);
    return vec4<f32>(srgb_to_linear(c.rgb), 1.0);
    // return vec4<f32>(uv.x, uv.y, 0.0, 1.0);
}

// @fragment
// fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//     let resolution = vec2<f32>(1080.0, 1080.0);
//     let uv = in.uv;

//     let x = uv.x; // Assume coord.x is normalized (0.0 - 1.0)
//     let y = textureSample(my_texture, my_sampler, x);

//     return y;
// }