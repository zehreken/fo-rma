use std::any::Any;
use wgpu::{BindGroup, Buffer, RenderPipeline};

pub mod diffuse_color_material;
pub mod equalizer_material;
pub mod post_process_material;
pub mod unlit_color_material;

pub trait MaterialTrait {
    fn render_pipeline(&self) -> &RenderPipeline;
    fn buffers(&self) -> &[Buffer];
    fn bind_groups(&self) -> &[BindGroup];
    fn update(&self, data: &dyn Any);
}
