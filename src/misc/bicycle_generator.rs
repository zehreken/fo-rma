use rand::Rng;

/// I want you to do some research about the Vitruvian man
/// https://en.wikipedia.org/wiki/Vitruvian_Man
/// It was a search for the proportions of ideal man
/// A Vitruvian bike can be a similar search
use crate::basics::scene_loader::SceneData;

pub fn generate_bicycle_scene() -> SceneData {
    todo!()
}

fn three_random_circles() {
    let mut rng = rand::rng();
    let a: Circle = Circle::new(
        rng.random_range(-2.0..2.0),
        rng.random_range(-2.0..2.0),
        rng.random_range(2.0..5.0),
    );
    let b: Circle = Circle::new(
        rng.random_range(-2.0..2.0),
        rng.random_range(-2.0..2.0),
        rng.random_range(2.0..5.0),
    );
    let c: Circle = Circle::new(
        rng.random_range(-2.0..2.0),
        rng.random_range(-2.0..2.0),
        rng.random_range(2.0..5.0),
    );
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
