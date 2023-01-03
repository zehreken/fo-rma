use crate::cpu_ray_tracer::primitives::vec3::*;
use crate::shapes::hitable::Hitable;
use crate::shapes::sphere::Sphere;

pub fn get_objects() -> Vec<Box<dyn Hitable>> {
    get_cover_07()
}

fn get_cover_01() -> Vec<Box<dyn Hitable>> {
    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        0.5,
        1, // lambert ian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));

    objects
}

fn get_cover_02() -> Vec<Box<dyn Hitable>> {
    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.5, 0.0, 0.0),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-0.5, 0.0, 0.0),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));

    objects
}

fn get_cover_03() -> Vec<Box<dyn Hitable>> {
    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.5, -0.43302, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-0.5, -0.43302, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.43302, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));

    objects
}

fn get_cover_04() -> Vec<Box<dyn Hitable>> {
    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.5, 0.5, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-0.5, 0.5, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.5, -0.5, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-0.5, -0.5, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));

    objects
}

fn get_cover_05() -> Vec<Box<dyn Hitable>> {
    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.5, -0.43302, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-0.5, -0.43302, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.43302, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-2.0, 0.43302, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.43302, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));

    objects
}

fn get_cover_06() -> Vec<Box<dyn Hitable>> {
    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.5, 1.0, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-0.5, 1.0, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.5, 0.0, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-0.5, 0.0, -0.5),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.5, -1.0, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-0.5, -1.0, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.0,
    )));

    objects
}

fn get_cover_07() -> Vec<Box<dyn Hitable>> {
    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.5, -0.86604, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.2,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-0.5, -0.86604, -0.8),
        0.5,
        1, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.2,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -0.5),
        0.5,
        2, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.2,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -0.8),
        0.5,
        2, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.2,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.2,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.5, 0.86604, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.25, 0.25, 0.25),
        0.2,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-0.5, 0.86604, -0.8),
        0.5,
        0, // lambertian
        Vec3::new(0.95, 0.25, 0.25),
        0.2,
    )));

    objects
}
