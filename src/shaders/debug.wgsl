struct Object {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    normal: mat3x3<f32>,
};
@group(0) @binding(0) var<uniform> object: Object;

struct Material {
    color: vec4<f32>
}
@group(1) @binding(0)
var<uniform> material: Material;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = object.view_proj * object.model * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    var color = material.color.rgb;
    color = srgb_to_linear(color);
    return vec4<f32>(color.rgb, 1.0);
}
