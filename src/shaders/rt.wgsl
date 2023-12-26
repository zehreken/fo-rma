// Vertex shader
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3< f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct Material {
    color: vec3<f32>, // diffuse color
    f0: f32, // specular color (monochrome)
};

struct DirectionalLight {
    direction: vec3<f32>,
    color: vec3<f32>,
};

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
};

struct Hit {
    t: f32, // solution to p=o+t*d
    normal: vec3<f32>, // normal
    material: Material, // material
};

struct Plane {
    d: f32, // solution to dot(n,p)+d=0
    normal: vec3<f32>, // normal
    material: Material, // material
};

struct Sphere {
    radius: f32, // radius
    center: vec3<f32>, // center position
    material: Material, // material
};

fn intersect_plane(p: Plane, r: Ray) -> Hit{
    let no_hit: Hit = Hit(1e10, vec3(0.), Material(vec3(-1.), -1.));
    let dotnd = dot(p.normal, r.direction);
    if (dotnd > 0.) {
        return no_hit;
    }

    let t: f32 = -(dot(r.origin, p.normal) + p.d) / dotnd;
    return Hit(t, p.normal, p.material);
}

fn intersect_sphere(s: Sphere, r: Ray) -> Hit {
    let no_hit: Hit = Hit(1e10, vec3(0.), Material(vec3(-1.), -1.));
    let op: vec3<f32> = s.center - r.origin;
    let b: f32 = dot(op, r.direction);
    var det: f32 = b * b - dot(op, op) + s.radius * s.radius;
    if (det < 0.) {
        return no_hit;
    }

    det = sqrt(det);
    var t: f32 = b - det;
    if (t < 0.) {
        t = b + det;
    }
    if (t < 0.) {
        return no_hit;
    }

    return Hit(t, (r.origin + t * r.direction - s.center) / s.radius, s.material);
}

fn get_color(ray_direction: vec3<f32>) -> vec3<f32> {
    let a = 0.5 * (ray_direction.y + 1.0);
    let color0 = (1.0 - a) * vec3(1.0, 1.0, 1.0);
    let color1 = a * vec3(0.5, 0.7, 1.0);
    return color0 + color1;
}

fn get_sky_color(ray_direction: vec3<f32>) -> vec3<f32> {
    let sun_light: DirectionalLight = DirectionalLight(normalize(vec3(1.0, 0.5, 0.5)), vec3(1e3));
    let transition: f32 = pow(smoothstep(0.02, 0.5, ray_direction.y), 0.4);

    let sky: vec3<f32> = 2e0*mix(vec3(0.12, 0.43, 1.0), vec3(0.52, 0.77, 1.0), transition);
    let sun: vec3<f32> = sun_light.color * pow(abs(dot(ray_direction, sun_light.direction)), 5000.);
    return sky + sun;
}
/*
vec3 unit_direction = unit_vector(r.direction());
auto a = 0.5*(unit_direction.y() + 1.0);
return (1.0-a)*color(1.0, 1.0, 1.0) + a*color(0.5, 0.7, 1.0);
*/

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
    let resolution: vec2<f32> = vec2(1600.0, 1200.0);
    let aspect_ratio = 4.0 / 3.0;
    let uv = 2.0 * in.clip_position.xy / resolution.xy - 1.0; // Maps xy to [-1, 1]

    let origin = vec3(0., 1.0, 4.);
    let direction = normalize(vec3(aspect_ratio * uv.x, uv.y, -1.5));
    
    let ray: Ray = Ray(origin, direction);
    let color = get_sky_color(ray.direction) / 4.0;

    // if (uv.y < -0.9) {
    //     return vec4<f32>(0.0);
    // } else {
    //     return vec4<f32>(1.0);
    // }
        
    return vec4<f32>(color, 1.0);
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