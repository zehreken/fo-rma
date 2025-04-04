struct Object {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    normal: mat3x3<f32>,
};
@group(0) @binding(0)
var<uniform> object: Object;

struct Light {
    position: vec4<f32>,
    color: vec4<f32>,
};
@group(1) @binding(0)
var<uniform> light: Light;

struct Material {
    color1: vec4<f32>,
    color2: vec4<f32>,
    color3: vec4<f32>,
    signal: f32,
};
@group(2) @binding(0)
var<uniform> material: Material;

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
    @location(5) uv: vec2<f32>,
    @location(4) signal: f32,
};

// Vertex shader
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
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

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(1080.0, 1080.0);
    let uv = in.uv;

    let top = srgb_to_linear(in.color1.rgb);
    let middle = srgb_to_linear(in.color2.rgb);
    let bottom = srgb_to_linear(in.color3.rgb);
    var color = mix(top, bottom, step(0.5, uv.y));

    let pi = 3.1415;
    let freq = 10.0;
    let amp = 1.0 / 10.0;

    // Calculate y-pos out of x-pos
    var y = sin((uv.x + in.signal / 10.0) * pi * freq) * amp + 0.5;
    // var y = uv.y + 0.5;

    // Define a uniform thickness threshold
    let thickness = 10.0;
    let thicknessThreshold = thickness / resolution.y; // Scale thickness to screen space

    // Calculate the perpendicular distance to the sine wave
    let distToWave = abs(uv.y - y) / sqrt(1.0 + pow(cos(uv.x * pi * freq) * pi * freq * amp, 2.0));

    // Blend factor based on the distance
    let blendFactor = clamp(distToWave / thicknessThreshold, 0.0, 1.0);

    // Sharp edges: If within the threshold, set the color directly
    if (distToWave < thicknessThreshold) {
        color = middle;
    } else {
        color = color; // Keep the original background color
    }

    // if (uv.y > 0.5) {
    //     color = top;
    // } else {
    //     color = middle;
    // }

    // color = mix(white, color, blendFactor);

    return vec4<f32>(color, 1.0);
}