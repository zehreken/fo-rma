struct Object {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    normal: mat3x3<f32>,
};
@group(0) @binding(0) var<uniform> object: Object;

struct Material {
    color: vec4<f32> // a is for signal
}
@group(1) @binding(0) var<uniform> material: Material;

struct Light {
    position: vec4<f32>,
    color: vec4<f32>,
};
@group(2) @binding(0) var<uniform> light: Light;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) world_normal: vec3<f32>,
};

// Vertex shader
@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // out.color = material.color * material.color.a;  // Apply signal to color
    out.color = material.color;
    let world_position = (object.model * vec4<f32>(model.position, 1.0)).xyz;
    out.world_position = world_position;
    out.world_normal = object.normal * model.normal;
    out.clip_position = object.view_proj * vec4<f32>(world_position, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.world_normal);
    let light_dir = normalize(light.position.xyz - in.world_position);

    // Ambient
    let ambient_strength = 0.1;
    let ambient = vec3(1.0, 1.0, 1.0) * ambient_strength;

    // Diffuse
    let diff = max(dot(normal, light_dir), 0.0);
    let diffuse = light.color.rgb * diff * light.color.a * clamp(in.color.a, 0.0, in.color.a) * 2.0;

    // Combine lighting with vertex color (which includes signal)
    let result = (ambient + diffuse) * in.color.rgb;

    // Debugging: Uncomment one of these to visualize different aspects
    // return vec4<f32>((light_dir + 1.0) / 2.0, 1.0);  // Visualize light direction
    // return vec4<f32>((normal + 1.0) / 2.0, 1.0);     // Visualize normals
    // return vec4<f32>(vec3<f32>(diff), 1.0);          // Visualize diffuse term

    return vec4<f32>(result, 1.0);
}