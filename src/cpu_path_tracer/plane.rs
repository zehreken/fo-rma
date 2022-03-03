use super::primitives::vec3::*;
use super::ray::*;
use super::sphere::Hitable;
use super::utility::*;
use std::fmt;

pub struct Plane {
    _position: Vec3,
    _rotation: Vec3,
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
    fn hit(self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        false
    }

    fn scatter(
        self,
        ray: Ray,
        hit_record: &mut HitRecord,
        reflect_record: &mut ReflectRecord,
    ) -> bool {
        false
    }
}

impl Plane {
    pub fn new(material: u8, color: Vec3, fuzz: f32) -> Plane {
        Plane {
            _position: Vec3::zero(),
            _rotation: Vec3::zero(),
            material,
            color,
            fuzz,
        }
    }
}
