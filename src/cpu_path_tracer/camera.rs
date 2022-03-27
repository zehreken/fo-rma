use super::primitives::vec3::*;
use super::ray::*;
use super::utility::*;

const PI: f32 = 3.14159265359;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    aspect: f32,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        lower_left_corner: Vec3,
        aspect: f32,
        horizontal: Vec3,
        vertical: Vec3,
        u: Vec3,
        v: Vec3,
        w: Vec3,
        lens_radius: f32,
    ) -> Camera {
        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            aspect,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
        }
    }

    pub fn get_camera(width: u32, height: u32) -> Camera {
        let look_from: Vec3 = Vec3::new(0.0, 0.5, 1.0);
        let look_at: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let v_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let v_fov: f32 = 60.0;
        let aspect: f32 = width as f32 / height as f32;
        let aperture: f32 = 0.1;
        let focus_dist: f32 = (look_from - look_at).length();

        let lens_radius: f32 = aperture / 2.0;
        let theta: f32 = v_fov * PI / 180.0;
        let half_height: f32 = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).unit_vector();
        let u = Vec3::cross(v_up, w).unit_vector();
        let v = Vec3::cross(w, u);
        let lower_left_corner: Vec3 =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        return Camera::new(
            origin,
            lower_left_corner,
            aspect,
            horizontal,
            vertical,
            v,
            u,
            w,
            lens_radius,
        );
    }

    pub fn get_ray(self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_circle();
        let offset = rd.x * self.u + rd.y * self.v;

        return Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        );
    }

    pub fn translate(&mut self, delta: Vec3) {
        self.origin = self.origin + delta;
        let look_at: Vec3 = Vec3::new(0.0, 0.0, -1.0);
        let v_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        let v_fov: f32 = 60.0;
        let aperture: f32 = 0.1;
        let focus_dist: f32 = (self.origin - look_at).length();

        self.lens_radius = aperture / 2.0;
        let theta: f32 = v_fov * PI / 180.0;
        let half_height: f32 = (theta / 2.0).tan();
        let half_width = self.aspect * half_height;
        self.w = (self.origin - look_at).unit_vector();
        self.u = Vec3::cross(v_up, self.w).unit_vector();
        self.v = Vec3::cross(self.w, self.u);
        self.lower_left_corner = self.origin
            - half_width * focus_dist * self.u
            - half_height * focus_dist * self.v
            - focus_dist * self.w;
        self.horizontal = 2.0 * half_width * focus_dist * self.u;
        self.vertical = 2.0 * half_height * focus_dist * self.v;
    }
}
