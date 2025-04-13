use crate::{
    basics::{primitive::Primitive, quad::Quad},
    material::{
        post_process_material::PostProcessMaterial, unlit_color_material::UnlitColorMaterial,
    },
};
use wgpu::{
    Color, Device, Operations, Queue, RenderPassColorAttachment, SurfaceConfiguration, TextureView,
};

pub struct ScreenRenderer {
    screen_quad: Box<dyn Primitive>,
}

impl ScreenRenderer {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let mock_material = UnlitColorMaterial::new(device, surface_config);
        let screen_quad = Box::new(Quad::new(&device, Box::new(mock_material)));

        Self { screen_quad }
    }

    pub fn render(
        &self,
        device: &Device,
        queue: &Queue,
        output_view: &TextureView,
        render_texture_material: &PostProcessMaterial,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("screen_render_encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("screen_render_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: output_view,
                resolve_target: None,
                ops: Operations {
                    load: wgpu::LoadOp::Clear(Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&render_texture_material.render_pipeline);

        render_pass.set_bind_group(0, &render_texture_material.texture_bind_group, &[]);
        self.screen_quad.draw(&mut render_pass);
        drop(render_pass);

        queue.submit(Some(encoder.finish()));
    }
}
