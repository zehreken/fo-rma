use crate::{
    basics::scene::Scene,
    color_utils::{self},
    material::MaterialType,
};
use wgpu::{
    Color, CommandEncoderDescriptor, Device, LoadOp, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, TextureView,
};

const BG_COLOR: [f32; 3] = color_utils::CCP.palette[0];

pub struct FillRenderer {}

impl FillRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        depth_texture: &TextureView,
        output_view: &TextureView,
        level: &Scene,
    ) {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("fill_render_encoder"),
        });

        let c_bg_color = color_utils::srgb_to_linear(BG_COLOR, color_utils::GAMMA);
        let bg_color = Color {
            r: c_bg_color[0] as f64,
            g: c_bg_color[1] as f64,
            b: c_bg_color[2] as f64,
            a: 1.0,
        };

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("fill_render_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(bg_color),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_texture,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        for (material_id, objects) in &level.material_object_map {
            if *material_id == MaterialType::EqualizerMaterial {
                for primitive in objects {
                    render_pass.set_pipeline(&primitive.material().render_pipeline());

                    render_pass.set_bind_group(0, &primitive.material().bind_groups()[0], &[]);
                    render_pass.set_bind_group(1, &primitive.material().bind_groups()[1], &[]);
                    render_pass.set_bind_group(2, &primitive.material().bind_groups()[2], &[]);

                    primitive.draw(&mut render_pass);
                }
            } else if *material_id == MaterialType::UnlitColorMaterial {
                for primitive in objects {
                    render_pass.set_pipeline(&primitive.material().render_pipeline());

                    render_pass.set_bind_group(0, &primitive.material().bind_groups()[0], &[]);
                    render_pass.set_bind_group(1, &primitive.material().bind_groups()[1], &[]);

                    primitive.draw(&mut render_pass);
                }
            } else if *material_id == MaterialType::DiffuseColorMaterial {
                for primitive in objects {
                    render_pass.set_pipeline(&primitive.material().render_pipeline());

                    render_pass.set_bind_group(0, &primitive.material().bind_groups()[0], &[]);
                    render_pass.set_bind_group(1, &primitive.material().bind_groups()[1], &[]);
                    render_pass.set_bind_group(2, &primitive.material().bind_groups()[2], &[]);

                    primitive.draw(&mut render_pass);
                }
            }
        }

        drop(render_pass); // also releases encoder

        queue.submit(Some(encoder.finish()));
    }
}
