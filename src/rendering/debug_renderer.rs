use crate::{
    basics::{
        scene::Scene,
        uniforms::{ColorUniform, ObjectUniform},
    },
    material::{debug_line_material::DebugLineMaterial, MaterialTrait},
};
use wgpu::{
    CommandEncoderDescriptor, Device, LoadOp, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, SurfaceConfiguration, TextureView,
};

pub struct DebugRenderer {
    debug_material: DebugLineMaterial,
}

impl DebugRenderer {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let debug_material = DebugLineMaterial::new(device, surface_config);

        Self { debug_material }
    }

    pub fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        depth_texture: &TextureView,
        output_view: &TextureView,
        scene: &Scene,
    ) {
        let color = ColorUniform {
            color: [0.0, 1.0, 0.0, 1.0],
        };
        for primitive in &scene.debug_objects {
            let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
                label: Some("line_renderer_encoder"),
            });

            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("line_render_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &output_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load,
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

            render_pass.set_pipeline(&self.debug_material.render_pipeline()); // Set pipeline once
            let object = ObjectUniform {
                view_proj: scene.camera.build_view_projection_matrix(),
                model: primitive.model_matrix(),
                normal1: primitive.normal_matrix().x_axis.extend(0.0).to_array(),
                normal2: primitive.normal_matrix().y_axis.extend(0.0).to_array(),
                normal3: primitive.normal_matrix().z_axis.extend(0.0).to_array(),
            };

            queue.write_buffer(
                &self.debug_material.buffers()[0],
                0,
                bytemuck::cast_slice(&[object]),
            );
            queue.write_buffer(
                &self.debug_material.buffers()[1],
                0,
                bytemuck::cast_slice(&[color]),
            );

            render_pass.set_bind_group(0, &self.debug_material.bind_groups()[0], &[]);
            render_pass.set_bind_group(1, &self.debug_material.bind_groups()[1], &[]);

            primitive.draw(&mut render_pass);
            drop(render_pass);

            queue.submit(Some(encoder.finish()));
        }
    }
}
