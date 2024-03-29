use super::hitable::*;
use crate::cpu_ray_tracer::primitives::vec3::*;
use crate::cpu_ray_tracer::ray::*;
use crate::cpu_ray_tracer::utility::*;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    _position: Vec3,
    material: u8,
    color: Vec3,
    fuzz: f32,
}

impl fmt::Display for AABB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "aabb")
    }
}

impl Hitable for AABB {
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

impl AABB {
    pub fn new(material: u8, color: Vec3, fuzz: f32) -> AABB {
        AABB {
            _position: Vec3::zero(),
            material,
            color,
            fuzz,
        }
    }
}
