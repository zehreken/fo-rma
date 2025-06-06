use crate::{
    basics::uniforms::{ObjectUniform, TextureUniform},
    material::{MaterialTrait, MaterialType},
    rendering_utils,
};
use wgpu::{BindGroup, Buffer, Device, RenderPipeline, SurfaceConfiguration, Texture, TextureView};

pub struct TextureUniforms {
    pub object: ObjectUniform,
    pub texture: TextureUniform,
}

pub struct TextureMaterial {
    render_pipeline: RenderPipeline,
    buffers: [Buffer; 2],
    bind_groups: [BindGroup; 2],
}

impl MaterialTrait for TextureMaterial {
    fn render_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
    }

    fn buffers(&self) -> &[Buffer] {
        &self.buffers
    }

    fn bind_groups(&self) -> &[BindGroup] {
        &self.bind_groups
    }

    fn update(&self, queue: &wgpu::Queue, data: &dyn std::any::Any) {
        if let Some(data) = data.downcast_ref::<TextureUniforms>() {}
    }

    fn get_id(&self) -> MaterialType {
        MaterialType::Texture
    }
}

impl TextureMaterial {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let shader = rendering_utils::create_shader_module(device, MaterialType::Texture);

        todo!()
    }
}
