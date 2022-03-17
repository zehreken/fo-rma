use super::primitives::vec3::*;
use super::ray::*;
use super::utility::*;
use std::fmt;

pub trait Hitable: Send {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
    fn scatter(
        &self,
        ray: Ray,
        hit_record: &mut HitRecord,
        reflect_record: &mut ReflectRecord,
    ) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: u8,
    color: Vec3,
    fuzz: f32, // only used for metal
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sphere")
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let origin_to_center: Vec3 = ray.origin() - self.center;
        let a: f32 = Vec3::dot(ray.direction(), ray.direction());
        let b: f32 = Vec3::dot(origin_to_center, ray.direction());
        let c: f32 = Vec3::dot(origin_to_center, origin_to_center) - self.radius * self.radius;
        let discriminant: f32 = b * b - a * c;

        if discriminant > 0.0 {
            let root_one: f32 = (-b - discriminant.sqrt()) / a;
            if root_one > t_min && root_one < t_max {
                hit_record.t = root_one;
                hit_record.p = ray.point_at(root_one);
                hit_record.normal = (hit_record.p - self.center) / self.radius;

                return true;
            }

            let root_two = (-b + discriminant.sqrt()) / a;
            if root_two > t_min && root_two < t_max {
                hit_record.t = root_two;
                hit_record.p = ray.point_at(root_two);
                hit_record.normal = (hit_record.p - self.center) / self.radius;

                return true;
            }
        }

        return false;
    }

    fn scatter(
        &self,
        ray: Ray,
        hit_record: &mut HitRecord,
        reflect_record: &mut ReflectRecord,
    ) -> bool {
        if self.material == 0 {
            return self.lambertian(hit_record, reflect_record);
        } else if self.material == 1 {
            return self.metal(ray, hit_record, reflect_record);
        } else if self.material == 2 {
            return self.dielectric(ray, hit_record, reflect_record);
        } else {
            return self.lambertian(hit_record, reflect_record); // default is lambertian
        }
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: u8, color: Vec3, fuzz: f32) -> Sphere {
        Sphere {
            center,
            radius,
            material,
            color,
            fuzz,
        }
    }

    fn lambertian(
        self,
        // ray: Ray,
        hit_record: &mut HitRecord,
        reflect_record: &mut ReflectRecord,
    ) -> bool {
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

    fn dielectric(
        self,
        ray: Ray,
        hit_record: &mut HitRecord,
        reflect_record: &mut ReflectRecord,
    ) -> bool {
        let ref_idx = 1.3;
        let mut outward_normal: Vec3 = Vec3::zero();
        let reflected = reflect(ray.direction(), hit_record.normal);
        let ni_over_nt: f32;
        reflect_record.attenuation = self.color; // Vec3::new(1.0, 1.0, 1.0);
        let mut refracted: Vec3 = Vec3::zero();
        let reflect_prob: f32;
        let cosine: f32;
        if Vec3::dot(ray.direction(), hit_record.normal) > 0.0 {
            outward_normal = outward_normal - hit_record.normal;
            ni_over_nt = ref_idx;
            cosine =
                ref_idx * Vec3::dot(ray.direction(), hit_record.normal) / ray.direction().length();
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / ref_idx;
            cosine = -Vec3::dot(ray.direction(), hit_record.normal) / ray.direction().length();
        }

        if refract(ray.direction(), outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = schlick(cosine, ref_idx);
        } else {
            reflect_prob = 1.0;
        }

        if get_random() < reflect_prob {
            reflect_record.scattered = Ray::new(hit_record.p, reflected);
        } else {
            reflect_record.scattered = Ray::new(hit_record.p, refracted);
        }

        return true;
    }
}
