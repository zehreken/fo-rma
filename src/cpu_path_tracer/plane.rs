use super::primitives::vec3::*;
use super::ray::*;
use super::sphere::Hitable;
use super::utility::*;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    position: Vec3,
    orientation: Vec3,
    material: u8,
    color: Vec3,
    fuzz: f32,
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "plane")
    }
}

impl Hitable for Plane {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let denom = Vec3::dot(self.orientation, ray.direction());
        if denom > 0.001 {
            let plane_to_ray = self.position - ray.origin();
            hit_record.t = Vec3::dot(plane_to_ray, self.orientation) / denom;
            return true;
        }
        false
    }

    fn scatter(
        &self,
        ray: Ray,
        hit_record: &mut HitRecord,
        reflect_record: &mut ReflectRecord,
    ) -> bool {
        true
    }
}

impl Plane {
    pub fn new(position: Vec3, orientation: Vec3, material: u8, color: Vec3, fuzz: f32) -> Plane {
        Plane {
            position,
            orientation,
            material,
            color,
            fuzz,
        }
    }

    pub fn default() -> Plane {
        Plane {
            position: Vec3::zero(),
            orientation: Vec3::zero(),
            material: 0,
            color: Vec3::zero(),
            fuzz: 0.0,
        }
    }
}
