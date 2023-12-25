// Vertex shader
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3< f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct DirectionalLight {
    direction: vec3<f32>,
    color: vec3<f32>,
};

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
};

fn sky_color(d: vec3<f32>) -> vec3<f32> {
    let sun_light: DirectionalLight = DirectionalLight(normalize(vec3(1., .5, .5)), vec3(1e3));
    let transition = pow(smoothstep(0.02, 0.5, d.y), 0.4);

    let sky: vec3<f32> = 2e2*mix(vec3(0.52, 0.77, 1.0), vec3(0.12, 0.43, 1.0), transition);
    let sun: vec3<f32> = sun_light.c * pow(abs(dot(d, sun_light.d)), 5000.);
    return sky + sun;
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.color = model.color;
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2(600.0, 400.0);
    let uv = 2.0 * in.clip_position.xy / resolution  - 1.0;
    let o1: f32 = 0.25;
    let o2: f32 = 0.75;
    var msaa: array<vec2<f32>, 4> = array<vec2<f32>, 4>();
    msaa[0] = vec2(o1, o2);
    msaa[1] = vec2(o2, -o1);
    msaa[2] = vec2(-o1, -o2);
    msaa[3] = vec2(-o2, o1);
    
    var color: vec3<f32> = vec3(0.0);
    for (var i = 0; i < 4; i++) {
        let p0 = vec3(0.0, 1.0, 4.0);
        let offset = vec3(msaa[i] / resolution.y, 0.0);
        let d = normalize(vec3(resolution.x / resolution.y * uv.x, uv.y, -1.5) + offset);
    }
    return vec4<f32>(0.0);
}

// void mainImage( out vec4 fragColor, in vec2 fragCoord )
// {
// 	vec2 uv = 2. * fragCoord.xy / iResolution.xy - 1.;

//     float o1 = 0.25;
//     float o2 = 0.75;
//     vec2 msaa[4];
//     msaa[0] = vec2( o1,  o2);
//     msaa[1] = vec2( o2, -o1);
//     msaa[2] = vec2(-o1, -o2);
//     msaa[3] = vec2(-o2,  o1);

//     vec3 color = vec3(0.);
//     for (int i = 0; i < 4; ++i)
//     {
//         vec3 p0 = vec3(0., 1.1, 4.);
//         vec3 p = p0;
//         vec3 offset = vec3(msaa[i] / iResolution.y, 0.);
//         vec3 d = normalize(vec3(iResolution.x/iResolution.y * uv.x, uv.y, -1.5) + offset);
//         Ray r = Ray(p, d);
//         color += radiance(r) / 4.;
//     }

// 	fragColor = vec4(Uncharted2ToneMapping(color),1.0);
// }