use crate::renderer::Renderer;

use super::{primitive::Primitive, quad::Quad, sphere::Sphere};

pub struct Level {
    pub primitives: Vec<Box<dyn Primitive>>,
}

impl Level {
    pub fn new(renderer: &Renderer) -> Self {
        let primitives: Vec<Box<dyn Primitive>> = vec![
            Box::new(Sphere::new(renderer)),
            Box::new(Quad::new(renderer)),
        ];

        Self { primitives }
    }
}
