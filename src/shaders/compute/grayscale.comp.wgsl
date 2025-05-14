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
    // Based on NTSC formula
    let y = color.r * 0.299 + color.g * 0.587 + color.b * 0.114;
    let gray = vec4(y, y, y, color.a);
    textureStore(img, vec2<i32>(id.xy), gray);
}