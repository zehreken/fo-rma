@group(0) @binding(0)
var img: texture_storage_2d<rgba8unorm, write>;

@group(0) @binding(1)
var src: texture_2d<f32>;

// A uniform to pass arbitrary values to the shader
struct ControlUniform {
    values: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> control_uniform: ControlUniform;

@compute @workgroup_size(8, 8)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let dims = textureDimensions(img);
    if (id.x >= dims.x || id.y >= dims.y) {
        return;
    }

    let color = textureLoad(src, vec2<i32>(id.xy), 0);
    let i = id.x;
    let j = id.y;
    let noise = vec3(hash2(id.xy, control_uniform.values[0]) / 20.0);
    let new_color = vec4(color.rgb + noise, 1.0);
    textureStore(img, vec2<i32>(id.xy), new_color);
}

fn hash2(p: vec2<u32>, time: f32) -> f32 {
    let f = vec2<f32>(p) / 10.0;
    let k = vec2(0.3183099, 0.3678794); // 1/π and 1/e
    let v = f * k + time * 0.05;
    return fract(23.0 * fract(v.x * v.y * (v.x + v.y)));
}
