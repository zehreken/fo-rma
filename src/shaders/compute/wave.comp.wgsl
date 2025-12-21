// A new idea, a post processing effect that makes the screen move like a wave based on the audio signal.
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
    let y = color.r * 0.299 + color.g * 0.587 + color.b * 0.114;
    var saturated = vec4(y, y, y, 1.0);
    if (color.r > 0.4) {
        saturated = vec4(1.0, color.g, 0.1 * color.b, 1.0);
    }
    if (color.g > 0.4) {
        saturated = vec4(0.1, color.g, 0.1 * color.b, 1.0);
    }
    if (color.b > 0.4) {
        saturated = vec4(0.1 * color.r, color.g, 1.0, 1.0);
    }
    textureStore(img, vec2<i32>(id.xy), saturated);
}