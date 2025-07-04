use glam::{Mat4, Vec3};
use winit::dpi::PhysicalSize;

const SPEED: f32 = 0.05;
const FAST_SPEED: f32 = 1.0;

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub aspect: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub is_fast: bool,
}

impl Camera {
    pub fn new(eye: Vec3, target: Vec3, aspect: f32, fov_y: f32, z_near: f32, z_far: f32) -> Self {
        let up = Vec3::Y;

        let camera = Camera {
            eye,
            target,
            up,
            yaw: 0.0,
            pitch: 0.0,
            aspect,
            fov_y,
            z_near,
            z_far,
            is_fast: false,
        };

        camera
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.aspect = size.width as f32 / size.height as f32;
    }

    pub fn build_view_projection_matrix(&self) -> [[f32; 4]; 4] {
        let forward = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        );
        let target = self.eye + forward;
        let up = Vec3::Y;
        let view = Mat4::look_at_lh(self.eye, target, up);
        let proj = Mat4::perspective_lh(
            self.fov_y.to_radians(),
            self.aspect,
            self.z_near,
            self.z_far,
        );

        return (proj * view).to_cols_array_2d();
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.eye = position;
    }

    pub fn update(&mut self) {}

    pub fn reset(&mut self) {}

    pub fn rotate(&mut self, diff_x: f32, diff_y: f32) {
        let sensitivity = 0.0014;
        self.yaw -= diff_x * sensitivity;
        self.pitch -= diff_y * sensitivity;

        self.pitch = self.pitch.clamp(-1.0, 1.0);
    }

    pub fn set_fast(&mut self, is_fast: bool) {
        self.is_fast = is_fast;
    }

    pub fn move_x(&mut self, plus: bool) {
        let speed = if self.is_fast { FAST_SPEED } else { SPEED };
        let factor = if plus { -speed } else { speed };
        let forward = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        );
        let right = forward.cross(Vec3::Y).normalize();
        self.eye = self.eye + right * factor;
    }

    pub fn move_y(&mut self, plus: bool) {
        let speed = if self.is_fast { FAST_SPEED } else { SPEED };
        self.eye.y += if plus { speed } else { -speed };
        // self.target.y += if plus { DIST } else { -DIST };
    }

    pub fn move_z(&mut self, plus: bool) {
        let speed = if self.is_fast { FAST_SPEED } else { SPEED };
        let factor = if plus { -speed } else { speed };
        let forward = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            // 0.0,
            self.yaw.sin() * self.pitch.cos(),
        );
        self.eye = self.eye + forward * factor;
    }

    pub fn orbit_x(&mut self, plus: bool) {
        self.eye.x += if plus { SPEED } else { -SPEED };
    }

    pub fn orbit_y(&mut self, plus: bool) {
        self.eye.y += if plus { SPEED } else { -SPEED };
    }

    pub fn orbit_z(&mut self, plus: bool) {
        self.eye.z += if plus { SPEED } else { -SPEED };
    }
}
