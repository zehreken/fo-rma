use glam::{Mat4, Vec3};

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
}
