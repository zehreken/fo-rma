use super::hitable::*;
use crate::cpu_path_tracer::primitives::vec3::*;
use crate::cpu_path_tracer::ray::*;
use crate::cpu_path_tracer::utility::*;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    position: Vec3,
    material: u8,
    color: Vec3,
    fuzz: f32,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rectangle")
    }
}

impl Hitable for Rectangle {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        false
    }

    fn scatter(
        &self,
        ray: Ray,
        hit_record: &mut HitRecord,
        reflect_record: &mut ReflectRecord,
    ) -> bool {
        false
    }
}

impl Rectangle {
    pub fn new(material: u8, color: Vec3, fuzz: f32) -> Rectangle {
        Rectangle {
            position: Vec3::zero(),
            material,
            color,
            fuzz,
        }
    }
}
