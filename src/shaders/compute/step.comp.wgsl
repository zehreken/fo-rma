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
    var y = color.r * 0.299 + color.g * 0.587 + color.b * 0.114;
    let step_size = 0.2;
    let b = floor(y / step_size) * step_size;
    let gray = vec4(b, b, b, color.a);
    textureStore(img, vec2<i32>(id.xy), gray);
}