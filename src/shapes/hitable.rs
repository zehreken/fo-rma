use crate::cpu_ray_tracer::primitives::vec3::Vec3;
use crate::cpu_ray_tracer::ray::*;

pub trait Hitable: Send {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
    fn scatter(
        &self,
        ray: Ray,
        hit_record: &mut HitRecord,
        reflect_record: &mut ReflectRecord,
    ) -> bool;
    fn translate(&mut self, v: Vec3) {}
    fn rotate(&mut self, v: Vec3) {}
}
