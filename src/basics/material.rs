use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, ShaderModule};

pub struct Material {
    pub shader: ShaderModule,
    pub uniform_buffer: Buffer,
    pub bind_group: BindGroup,
}

impl Material {
    pub fn new(
        device: &Device,
        shader_main: &str,
        shader_name: &str,
        uniform_data: &[u8],
        bind_group_layout: &BindGroupLayout,
    ) -> Self {
        let shader_utils = include_str!("../shaders/utils.wgsl");
        let shader_combined = format!("{}\n{}", shader_main, shader_utils);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(shader_name),
            source: wgpu::ShaderSource::Wgsl(shader_combined.into()),
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniform_buffer"),
            size: 
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            shader,
            uniform_buffer,
            bind_group,
        }
    }
}
