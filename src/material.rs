use std::any::Any;
use wgpu::{BindGroup, Buffer, Queue, RenderPipeline};

pub mod debug_material;
pub mod diffuse_color_material;
pub mod equalizer_material;
pub mod post_process_material;
pub mod unlit_color_material;

pub trait MaterialTrait {
    fn render_pipeline(&self) -> &RenderPipeline;
    fn buffers(&self) -> &[Buffer];
    fn bind_groups(&self) -> &[BindGroup];
    fn update(&self, queue: &Queue, data: &dyn Any);
    fn get_id(&self) -> MaterialType;
}

#[derive(PartialEq, Eq, Hash)]
pub enum MaterialType {
    DebugMaterial,
    DiffuseColorMaterial,
    EqualizerMaterial,
    PostProcessMaterial,
    UnlitColorMaterial,
}
