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

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureUniform {}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 4], // xyz + padding
    pub color: [f32; 4],    // rgb and light intensity
}

// This is based on the shader and can vary a lot
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorUniform {
    pub color: [f32; 4],
}

impl UniformTrait for ColorUniform {
    fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(std::slice::from_ref(self))
    }

    fn get_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }

    fn set_signal(&mut self, signal: f32) {
        self.color[3] = signal;
    }
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

impl UniformTrait for EqualizerUniform {
    fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(std::slice::from_ref(self))
    }

    fn get_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }

    fn set_signal(&mut self, signal: f32) {
        self.signal = signal;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WaveWorldUniform {
    pub color1: [f32; 4],
    pub color2: [f32; 4],
    pub color3: [f32; 4],
    pub signal: f32,
    pub _padding: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ScreenQuadUniform {
    pub signal: [f32; 4],
}

impl UniformTrait for ScreenQuadUniform {
    fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(std::slice::from_ref(self))
    }

    fn get_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }

    fn set_signal(&mut self, signal: f32) {
        self.signal[0] = signal;
    }
}

impl UniformTrait for WaveWorldUniform {
    fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(std::slice::from_ref(self))
    }

    fn get_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }

    fn set_signal(&mut self, signal: f32) {
        self.signal = signal;
    }
}

pub trait UniformTrait {
    fn as_bytes(&self) -> &[u8];

    fn get_size(&self) -> usize;

    fn set_signal(&mut self, signal: f32);
}
