use wgpu::{Device, RenderPipeline, TextureFormat};

pub trait ShaderData {
    fn create_pipeline(
        &self,
        device: &Device,
        surface_format: TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
        light_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> RenderPipeline;
}
