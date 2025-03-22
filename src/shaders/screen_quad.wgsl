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
    // 2.0 makes it full screen quad
    out.clip_position = vec4<f32>(model.position * 2.0, 1.0);
    out.uv = vec2<f32>(model.uv.x, 1.0 - model.uv.y);
    return out;
}

@group(2) @binding(0)
var my_texture: texture_2d<f32>;
@group(2) @binding(1)
var my_sampler: sampler;
 
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(my_texture, my_sampler, in.uv);
    // return vec4<f32>(1.0, 0.0, 0.0, 1.0); // Red-Green debug visualization
}
