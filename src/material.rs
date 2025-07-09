use std::any::Any;
use wgpu::{BindGroup, Buffer, Queue, RenderPipeline};

pub mod debug_line_material;
pub mod debug_material;
pub mod diffuse_color_material;
pub mod diffuse_texture_material;
pub mod equalizer_material;
pub mod post_process_material;
pub mod texture_material;
pub mod unlit_color_material;
pub mod wave_material;

pub trait MaterialTrait {
    fn render_pipeline(&self) -> &RenderPipeline;
    fn buffers(&self) -> &[Buffer];
    fn bind_groups(&self) -> &[BindGroup];
    fn update(&self, queue: &Queue, data: &dyn Any);
    fn get_id(&self) -> Material;
}

#[derive(PartialEq, Eq, Hash)]
pub enum Material {
    Debug,
    DiffuseColor,
    Equalizer,
    UnlitColor,
    Wave,
    Texture,
    DiffuseTexture,
}
