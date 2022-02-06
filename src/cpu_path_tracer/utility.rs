use super::primitives::vec3::*;
use rand::Rng;

pub fn random_in_unit_circle() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3 =
        2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    while Vec3::dot(p, p) >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    }

    return p;
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3 = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
        - Vec3::new(1.0, 1.0, 1.0);
    while Vec3::dot(p, p) >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vec3::new(1.0, 1.0, 1.0);
    }

    return p;
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    // n = unit vector
    return v - 2.0 * Vec3::dot(v, n) * n;
}

pub fn get_random() -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen::<f32>();
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let uv: Vec3 = v.unit_vector();
    let dt: f32 = Vec3::dot(uv, n);
    let discriminant: f32 = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n;
        return true;
    } else {
        return false;
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0: f32 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
