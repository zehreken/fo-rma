use glam::{Mat4, Vec3};

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fov_y: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub uniform: CameraUniform,
}

impl Camera {
    pub fn new(eye: Vec3, target: Vec3, aspect: f32, fov_y: f32, z_near: f32, z_far: f32) -> Self {
        let up = Vec3::Y;
        let uniform = CameraUniform::new();

        let mut camera = Camera {
            eye,
            target,
            up,
            aspect,
            fov_y,
            z_near,
            z_far,
            uniform,
        };
        camera.update_view_proj();

        camera
    }

    fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(
            self.fov_y.to_radians(),
            self.aspect,
            self.z_near,
            self.z_far,
        );

        return proj * view;
    }

    pub fn update_view_proj(&mut self) -> [[f32; 4]; 4] {
        self.uniform.view_proj = self.build_view_projection_matrix().to_cols_array_2d();
        self.uniform.view_proj
        // Mat4::IDENTITY.to_cols_array_2d()
    }

    pub fn update(&mut self, v: f32) {
        self.eye = Vec3 {
            x: 0.0,
            y: 1.0,
            z: v,
        };
        self.build_view_projection_matrix();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}
