@group(0) @binding(0)
var img: texture_storage_2d<rgba8unorm, write>;

@group(0) @binding(1)
var src: texture_2d<f32>;

fn hash(p: vec2<f32>) -> f32 {
    let p3 = fract(vec3(p.xyx) * 0.1031);
    return fract((p3.x + p3.y) * dot(p3, vec3(p3.yzx) + 33.333));
}

@compute @workgroup_size(8, 8)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let dims = textureDimensions(img);
    if (id.x >= dims.x || id.y >= dims.y) {
        return;
    }
    
    // Current color of the pixel
    var color = textureLoad(src, vec2<i32>(id.xy), 0);
    // Current pixel position
    let pos = vec2<f32>(f32(id.x), f32(id.y));

    for (var i = 0; i < 50; i++) {
        let fi = f32(i);

        // Use different seeds for each circle's properties
        let center_seed = vec2<f32>(fi * 123.45, 67.89);
        let radius_seed = vec2<f32>(fi * 234.56, 78.9);
        let color_seed = vec2<f32>(fi * 345.67, 89.01);

        // Random circle center (0 to texture dimensions)
        let quarter_width = f32(dims.x) / 4.0;
        let quarter_height = f32(dims.y) / 4.0;
        let center_x = quarter_width + hash(center_seed) * quarter_width * 2.0;
        let center_y = quarter_height + hash(center_seed + vec2<f32>(1.0, 0.0)) * quarter_height * 2.0;
        let center = vec2<f32>(center_x, center_y);

        // Random radius (10 to 80 pixels)
        let radius = 10.0 + hash(radius_seed) * 200.0;

        // Random color
        let r = hash(color_seed);
        let g = hash(color_seed + vec2<f32>(2.0, 0.0));
        let b = hash(color_seed + vec2<f32>(3.0, 0.0));
        let circle_color = vec3<f32>(r, 0.0, 0.0);

        let distance = length(pos - center);
        if (distance <= radius) {
            color += vec4<f32>(circle_color, 1.0) * 0.3;
        }
    }

    
    textureStore(img, vec2<i32>(id.xy), color);
}