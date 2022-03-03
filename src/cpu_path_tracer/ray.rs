use super::primitives::vec3::*;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    from: Vec3,
    to: Vec3,
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "from: {}\nto: {}", self.from, self.to)
    }
}

impl Ray {
    pub fn new(from: Vec3, to: Vec3) -> Ray {
        Ray { from, to }
    }

    pub fn origin(self) -> Vec3 {
        return self.from;
    }

    pub fn direction(self) -> Vec3 {
        return self.to;
    }

    // P(t) = A + tb
    pub fn point_at(self, t: f32) -> Vec3 {
        return self.from + t * self.to;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::zero(),
            normal: Vec3::zero(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ReflectRecord {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

impl ReflectRecord {
    pub fn new(scattered: Ray, attenuation: Vec3) -> ReflectRecord {
        ReflectRecord {
            scattered,
            attenuation,
        }
    }
}
