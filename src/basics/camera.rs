use glam::{Mat4, Vec3};
use winit::dpi::PhysicalSize;

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    pub fn new(eye: Vec3, target: Vec3, aspect: f32, fov_y: f32, z_near: f32, z_far: f32) -> Self {
        let up = Vec3::Y;

        let camera = Camera {
            eye,
            target,
            up,
            aspect,
            fov_y,
            z_near,
            z_far,
        };

        camera
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.aspect = size.width as f32 / size.height as f32;
    }

    pub fn build_view_projection_matrix(&self) -> [[f32; 4]; 4] {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(
            self.fov_y.to_radians(),
            self.aspect,
            self.z_near,
            self.z_far,
        );

        return (proj * view).to_cols_array_2d();
    }

    pub fn update_position(&mut self, position: Vec3) {
        self.eye = position;
    }

    pub fn move_x(&mut self, pos: bool) {
        self.eye.x += if pos { 0.1 } else { -0.1 }
    }

    pub fn move_y(&mut self, pos: bool) {
        self.eye.y += if pos { 0.1 } else { -0.1 }
    }

    pub fn move_z(&mut self, pos: bool) {
        self.eye.z += if pos { 0.1 } else { -0.1 }
    }
}
