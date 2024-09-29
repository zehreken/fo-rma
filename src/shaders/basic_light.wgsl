struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    _padding: vec3<f32>,
    signal: f32,
    normal: mat3x3<f32>,
};
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct Light {
    position: vec3<f32>,
    _padding: f32,
    color: vec3<f32>,
    _padding2: f32,
};
@group(1) @binding(0) var<uniform> light: Light;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) world_normal: vec3<f32>,
};

// Vertex shader
@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color * uniforms.signal;  // Apply signal to color
    let world_position = (uniforms.model * vec4<f32>(model.position, 1.0)).xyz;
    out.world_position = world_position;
    out.world_normal = uniforms.normal * model.normal;
    out.clip_position = uniforms.view_proj * vec4<f32>(world_position, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.world_normal);
    let light_dir = normalize(light.position - in.world_position);

    // Ambient
    let ambient_strength = 0.1;
    let ambient = light.color * ambient_strength;

    // Diffuse
    let diff = max(dot(normal, light_dir), 0.0);
    let diffuse = light.color * diff;

    // Combine lighting with vertex color (which includes signal)
    let result = (ambient + diffuse) * in.color;

    return vec4<f32>(result, 1.0);
}