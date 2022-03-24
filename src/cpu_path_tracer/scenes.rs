use super::{hitable::Hitable, plane::Plane, primitives::vec3::Vec3, sphere::Sphere};

pub fn get_simple_scene() -> Vec<Sphere> {
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(
        Vec3::new(-0.5, 0.0, 0.0),
        0.5,
        0,
        Vec3::new(0.5, 0.5, 0.5),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.5, 0.0, 0.0),
        0.5,
        0,
        Vec3::new(0.7, 0.7, 0.7),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.0, -100.5, 0.0),
        100.0,
        0,
        Vec3::new(0.7, 0.1, 0.7),
        0.0,
    ));

    objects
}

pub fn get_plane_scene() -> Vec<Box<dyn Hitable>> {
    let mut objects: Vec<Box<dyn Hitable>> = vec![];
    objects.push(Box::new(Plane::new(
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::one() * 2.0,
        1,
        Vec3::new(0.8, 0.8, 0.8),
        0.,
    )));
    for i in 0..3 {
        objects.push(Box::new(Plane::new(
            Vec3::new(1.0, 0.0, -2.0 + i as f32 * 2.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::one(),
            1,
            Vec3::new(0.009, 0.2 + i as f32 * 0.1, 0.9),
            0.0,
        )));
    }
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(0.0, -1000.5, -1.0),
    //     1000.0,
    //     1,
    //     Vec3::new(0.4, 0.5, 0.5),
    //     0.1,
    // )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        0.5,
        1,
        Vec3::new(0.3, 0.3, 0.98),
        0.1,
    )));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     0,
    //     Vec3::new(0.9, 0.9, 0.9),
    //     0.2,
    // )));
    // objects.push(Box::new(Sphere::new(
    //     Vec3::new(-1.0, -0.0, -1.0),
    //     0.5,
    //     2,
    //     Vec3::new(1.0, 1.0, 1.0),
    //     0.2,
    // )));

    objects
}

fn get_objects() -> Vec<Sphere> {
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        0, // lambertian
        Vec3::new(0.5, 0.1, 0.1),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        1, // metal
        Vec3::new(0.9, 0.9, 0.9),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(1.0, 0.0, -3.0),
        0.5,
        1, // metal
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(-1.0, -0.0, -1.0),
        0.5,
        2, // dielectric
        Vec3::new(0.1, 0.5, 0.1).sqrt().sqrt().sqrt(),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.0, 0.0, 1.0),
        0.5,
        2,
        Vec3::new(0.5, 0.5, 0.3).sqrt().sqrt().sqrt(),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        0,
        Vec3::new(0.1, 0.3, 0.9),
        0.0,
    ));

    objects
}
