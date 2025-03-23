@group(0) @binding(0)
var img: texture_storage_2d<rgba8unorm, write>;

@group(0) @binding(1)
var src: texture_2d<f32>;

@compute @workgroup_size(8, 8)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let dims = textureDimensions(img);
    if (id.x >= dims.x || id.y >= dims.y) {
        return;
    }

    let color = textureLoad(src, vec2<i32>(id.xy), 0);
    let i = id.x;
    let j = id.y;
    let mixed = vec2(i ^ (i << 13), j ^ (j << 12)); // bit-mix
    let noise = vec3(hash2(id.xy) / 20.0);
    let new_color = vec4(color.rgb + noise, 1.0);
    textureStore(img, vec2<i32>(id.xy), new_color);
}

fn hash2(p: vec2<u32>) -> f32 {
    let x = f32(p.x);
    let y = f32(p.y);
    return fract(sin(dot(vec2(x, y), vec2(12.9898, 78.233))) * 43758.5453);
}
