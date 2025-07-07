use rand::Rng;

/// I want you to do some research about the Vitruvian man
/// https://en.wikipedia.org/wiki/Vitruvian_Man
/// It was a search for the proportions of ideal man
/// A Vitruvian bike can be a similar search
use crate::basics::scene_loader::{
    construct_scene_from_object_data, Object, Quat, SceneData, Vec3,
};

pub fn generate_bicycle_scene() -> SceneData {
    let circles = [random_circle(), random_circle(), random_circle()];

    let mut objects: Vec<Object> = Vec::new();

    for circle in circles {
        objects.push(Object {
            mesh: "sphere".to_owned(),
            material: "DiffuseColorMaterial".to_owned(),
            position: Vec3 {
                x: circle.x,
                y: circle.y,
                z: 0.0,
            },
            rotation: Quat {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            scale: Vec3 {
                x: circle.r,
                y: circle.r,
                z: circle.r,
            },
        });
    }

    construct_scene_from_object_data(objects)
}

fn random_circle() -> Circle {
    let mut rng = rand::rng();
    Circle::new(
        rng.random_range(-20.0..20.0),
        rng.random_range(-20.0..20.0),
        rng.random_range(2.0..5.0),
    )
}

struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

impl Circle {
    pub fn new(x: f32, y: f32, r: f32) -> Self {
        Self { x, y, r }
    }
}
