// All uniforms will be here
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ObjectUniform {
    pub view_proj: [[f32; 4]; 4],
    pub model: [[f32; 4]; 4],
    pub normal1: [f32; 4],
    pub normal2: [f32; 4],
    pub normal3: [f32; 4],
}

// This is based on the shader and can vary a lot
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialUniform {
    pub signal: f32,
    pub _padding: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 4], // xyz + padding
    pub color: [f32; 4],    // rgb and intensity
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EqualizerUniform {
    pub color1: [f32; 4],
    pub color2: [f32; 4],
    pub color3: [f32; 4],
    pub signal: f32,
    pub _padding: [f32; 3],
}
