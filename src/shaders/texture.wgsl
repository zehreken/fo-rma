struct Object {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    normal: mat3x3<f32>,
};
@group(0) @binding(0) var<uniform> object: Object;

// Vertex shader
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) world_normal: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = vec2<f32>(model.uv.x, 1.0 - model.uv.y);
    let world_position = (object.model * vec4<f32>(model.position, 1.0)).xyz;
    out.world_position = world_position;
    out.world_normal = object.normal * model.normal;
    out.clip_position = object.view_proj * vec4<f32>(world_position, 1.0);
    return out;
}

// Fragment shader
@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
