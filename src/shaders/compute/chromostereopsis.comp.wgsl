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
    let diff = color.r - color.b;
    let r = saturate(sign(diff));
    let chromo = vec4(r, 0.0, 1.0 - r, 1.0);
    textureStore(img, vec2<i32>(id.xy), chromo);
}