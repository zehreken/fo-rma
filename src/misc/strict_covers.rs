use super::super::cpu_path_tracer::primitives::vec3::*;
use super::super::cpu_path_tracer::sphere::*;

pub fn get_objects() -> Vec<Sphere> {
    get_cover_07()
}

fn get_cover_01() -> Vec<Sphere> {
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        0.5,
        1, // lambert ian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));

    objects
}

fn get_cover_02() -> Vec<Sphere> {
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(
        Vec3::new(0.5, 0.0, 0.0),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(-0.5, 0.0, 0.0),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));

    objects
}

fn get_cover_03() -> Vec<Sphere> {
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(
        Vec3::new(0.5, -0.43302, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(-0.5, -0.43302, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.0, 0.43302, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));

    objects
}

fn get_cover_04() -> Vec<Sphere> {
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(
        Vec3::new(0.5, 0.5, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(-0.5, 0.5, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.5, -0.5, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(-0.5, -0.5, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));

    objects
}

fn get_cover_05() -> Vec<Sphere> {
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(
        Vec3::new(0.5, -0.43302, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(-0.5, -0.43302, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.0, 0.43302, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(-2.0, 0.43302, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(-1.0, 0.43302, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));

    objects
}

fn get_cover_06() -> Vec<Sphere> {
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(
        Vec3::new(0.5, 1.0, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(-0.5, 1.0, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.5, 0.0, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(-0.5, 0.0, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.5, -1.0, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(-0.5, -1.0, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    ));

    objects
}

fn get_cover_07() -> Vec<Sphere> {
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(
        Vec3::new(0.5, -0.86604, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(-0.5, -0.86604, -0.8),
        0.5,
        1, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.0, 0.0, -0.5),
        0.5,
        2, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(1.0, 0.0, -0.8),
        0.5,
        2, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(-1.0, 0.0, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.5, 0.86604, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(-0.5, 0.86604, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.95, 0.25, 0.25),
        0.2,
    ));

    objects
}
