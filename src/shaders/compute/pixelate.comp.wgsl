@group(0) @binding(0)
var img: texture_storage_2d<rgba8unorm, write>;

@group(0) @binding(1)
var src: texture_2d<f32>;

const PIXEL_SIZE: i32 = 8;

@compute @workgroup_size(8, 8)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let dims = textureDimensions(img);
    if (id.x >= dims.x || id.y >= dims.y) {
        return;
    }

    // Round down to nearest block origin
    let pixel_pos = vec2<i32>(id.xy);
    let block_pos = (pixel_pos / PIXEL_SIZE) * PIXEL_SIZE;

    // Sample only once per block
    let color = textureLoad(src, block_pos, 0);
    textureStore(img, pixel_pos, color);
}
