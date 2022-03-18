use super::primitives::vec3::*;
use super::ray::*;
use super::sphere::Hitable;
use super::utility::*;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    position: Vec3,
    normal: Vec3,
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
        let denom = Vec3::dot(self.normal, ray.direction());
        if denom > t_min && denom < t_max {
            let plane_to_ray = self.position - ray.origin();
            hit_record.t = Vec3::dot(plane_to_ray, self.normal) / denom;
            hit_record.p = ray.point_at(hit_record.t);
            hit_record.normal = self.normal;
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
        if self.material == 0 {
            return self.lambertian(hit_record, reflect_record);
        } else {
            return self.metal(ray, hit_record, reflect_record);
        }
    }
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3, material: u8, color: Vec3, fuzz: f32) -> Plane {
        Plane {
            position,
            normal,
            material,
            color,
            fuzz,
        }
    }

    pub fn default() -> Plane {
        Plane {
            position: Vec3::zero(),
            normal: Vec3::zero(),
            material: 0,
            color: Vec3::zero(),
            fuzz: 0.0,
        }
    }

    fn lambertian(self, hit_record: &mut HitRecord, reflect_record: &mut ReflectRecord) -> bool {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        reflect_record.scattered = Ray::new(hit_record.p, target - hit_record.p);
        reflect_record.attenuation = self.color;
        return true;
    }

    fn metal(
        self,
        ray: Ray,
        hit_record: &mut HitRecord,
        reflect_record: &mut ReflectRecord,
    ) -> bool {
        let reflected = reflect(ray.direction().unit_vector(), hit_record.normal);
        reflect_record.scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        reflect_record.attenuation = self.color;

        return Vec3::dot(reflect_record.scattered.direction(), hit_record.normal) > 0.0;
    }
}
