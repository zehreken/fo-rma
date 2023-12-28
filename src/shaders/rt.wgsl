// Vertex shader
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3< f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

const EPSILON = 4e-4;
const SAMPLES = 20;

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

// 'a: ptr<function, Hit>' is equal to 'out Hit a' in glsl
// (*a) to evaluate first *a.t does not work
fn compare(a: ptr<function, Hit>, b: Hit) -> bool {
    if (b.material.f0 >= 0. && b.t < (*a).t)
    {
        *a = b;
        return true;
    }
    return false;
}

// Can't get a reference to let variable, 'let hit' does not compile
fn intersect_scene(ray: Ray) -> Hit {
    let no_hit: Hit = Hit(1e10, vec3(0.), Material(vec3(-1.), -1.));

    let s: Sphere = Sphere(1., vec3(1., 1., 0.), Material(vec3(0.5), 0.04));
    let p: Plane = Plane(0., vec3(0., 1., 0.), Material(vec3(0.5, 0.4, 0.3), 0.04));

    var hit = no_hit;
    compare(&hit, intersect_plane(p, ray));
    compare(&hit, intersect_sphere(s, ray));
    return hit;
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

    let sky: vec3<f32> = 2e0*mix(vec3(0.52, 0.77, 1.0), vec3(0.12, 0.43, 1.0), transition);
    let sun: vec3<f32> = sun_light.color * pow(abs(dot(ray_direction, sun_light.direction)), 5000.);
    return sky + sun;
}

fn radiance(r: Ray) -> vec3<f32> {
    var ray = r;
    var accum = vec3(0.);
    var attenuation = vec3(1.);

    for (var i = 0; i < SAMPLES; i++)
    {
        // Hit hit = intersectScene(r);
        let hit = intersect_scene(ray);

        if (hit.material.f0 >= 0.) {
        //     float f = fresnel(hit.n, -r.d, hit.m.f0);

        //     vec3 hitPos = r.o + hit.t * r.d;
            let hit_pos = ray.origin + hit.t * ray.direction;

        // Diffuse
        //     vec3 incoming = vec3(0.);
        //     incoming += accountForDirectionalLight(hitPos, hit.n, sunLight);
            accum += hit.material.color;

        //     accum += (1. - f) * attenuation * hit.m.c * incoming;

        //     // Specular: next bounce
        //     attenuation *= f;
        //     vec3 d = reflect(r.d, hit.n);
            let d = reflect(ray.direction, hit.normal);
            ray = Ray(ray.origin + hit.t * ray.direction + EPSILON * d, d);
        } else {
            accum += get_sky_color(ray.direction);
            break;
        }
    }
    accum = accum / f32(SAMPLES);
    return accum;
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
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
    var uv = 2.0 * in.clip_position.xy / resolution.xy - 1.0; // Maps xy to [-1, 1]
    uv.y = -uv.y;

    let origin = vec3(0., 1.0, 4.);
    let direction = normalize(vec3(aspect_ratio * uv.x, uv.y, -1.5));
    
    let ray: Ray = Ray(origin, direction);
    var color: vec3<f32> = vec3(0.0);
    color = radiance(ray);
    // for (var i = 0; i < 3; i++)
    // {
    //     let hit = intersect_scene(ray);
    //     if (hit.material.f0 >= 0.) {

    //     } else {
    //         color += get_sky_color(ray.direction);
    //         break;
    //     }
    // }
    
    return vec4<f32>(color, 1.0);
}
