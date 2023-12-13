use super::primitives::vec3::Vec3;
use crate::shapes::hitable::Hitable;
use crate::shapes::plane::Plane;
use crate::shapes::sphere::Sphere;

pub fn get_simple_scene() -> Vec<Box<dyn Hitable>> {
    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();
    objects.push(Box::new(Plane::new(
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::one() * 5.0,
        1,
        Vec3::new(1.0, 0.3, 0.3),
        0.05,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-0.5, 0.0, 0.0),
        0.5,
        0,
        Vec3::new(0.0, 0.66, 0.13),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.5, 0.0, 0.0),
        0.5,
        0,
        Vec3::new(0.7, 0.43, 0.0),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.5, 0.0),
        1000.0,
        0,
        Vec3::new(0.3, 0.3, 0.3),
        1.0,
    )));

    objects
}

pub fn get_plane_scene() -> Vec<Box<dyn Hitable>> {
    let mut objects: Vec<Box<dyn Hitable>> = vec![];
    // objects.push(Box::new(Plane::new(
    //     Vec3::new(-1.0, 0.0, 0.0),
    //     Vec3::new(-1.0, 0.0, 0.0),
    //     Vec3::one() * 1.0,
    //     1,
    //     Vec3::new(0.9, 0.1, 0.1),
    //     0.,
    // )));
    objects.push(Box::new(Plane::new(
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::one() * 100.0,
        1,
        Vec3::new(0.1, 0.9, 0.1),
        0.,
    )));
    // objects.push(Box::new(Plane::new(
    //     Vec3::new(1.0, 0.0, 0.0),
    //     Vec3::new(1.0, 0.0, 0.0),
    //     Vec3::one() * 1.0,
    //     1,
    //     Vec3::new(0.1, 0.1, 0.9),
    //     0.,
    // )));
    // objects.push(Box::new(Plane::new(
    //     Vec3::new(0.0, 0.0, 1.0),
    //     Vec3::new(0.0, 0.0, 1.0),
    //     Vec3::one() * 1.0,
    //     1,
    //     Vec3::new(0.9, 0.9, 0.9),
    //     0.9,
    // )));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(0.0, 1.0, 0.0),
    //     0.4,
    //     1,
    //     Vec3::new(0.83, 0.69, 0.21),
    //     0.3,
    // )));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::zero(),
    //     0.4,
    //     1,
    //     Vec3::new(0.83, 0.69, 0.21),
    //     0.3,
    // )));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(0.0, -1.0, 0.0),
    //     0.4,
    //     1,
    //     Vec3::new(0.83, 0.69, 1.0),
    //     0.3,
    // )));
    // for i in 0..3 {
    //     objects.push(Box::new(Plane::new(
    //         Vec3::new(1.0, 0.0, -2.0 + i as f32 * 2.0),
    //         Vec3::new(1.0, 0.0, 0.0),
    //         Vec3::one(),
    //         1,
    //         Vec3::new(1.0 - i as f32 * 0.2, 0.2 + i as f32 * 0.2, 0.9),
    //         0.0,
    //     )));
    // }

    objects
}

pub fn get_objects() -> Vec<Box<dyn Hitable>> {
    let mut objects: Vec<Box<dyn Hitable>> = Vec::new();
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        0, // lambertian
        Vec3::new(0.5, 0.1, 0.1),
        0.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        1, // metal
        Vec3::new(0.9, 0.9, 0.9),
        0.2,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -3.0),
        0.5,
        1, // metal
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-1.0, -0.0, -1.0),
        0.5,
        2, // dielectric
        Vec3::new(0.1, 0.5, 0.1).sqrt().sqrt().sqrt(),
        0.2,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 1.0),
        0.5,
        2,
        Vec3::new(0.5, 0.5, 0.3).sqrt().sqrt().sqrt(),
        0.2,
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        0,
        Vec3::new(0.1, 0.3, 0.9),
        0.0,
    )));

    objects
}
