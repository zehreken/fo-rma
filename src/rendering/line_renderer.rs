use std::mem;

use crate::{
    basics::{core::GenericUniformData, scene::Scene, uniforms::ObjectUniform},
    renderer, rendering_utils,
};
use wgpu::{
    CommandEncoderDescriptor, Device, LoadOp, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, StoreOp, SurfaceConfiguration, Texture, TextureView,
};

pub struct LineRenderer {
    render_pipeline: RenderPipeline,
    uniform_data: GenericUniformData,
}

impl LineRenderer {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let (uniform_data, render_pipeline) = rendering_utils::create_debug_uniform_data(
            device,
            surface_config,
            renderer::PRIMITIVE_COUNT,
        );

        Self {
            render_pipeline,
            uniform_data,
        }
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

        render_pass.set_pipeline(&self.render_pipeline);

        // Update debug uniforms
        for (i, primitive) in level.objects.iter().enumerate() {
            let debug_uniform = ObjectUniform {
                view_proj: level.camera.build_view_projection_matrix(),
                // debug_uniforms.model = self.light.debug_mesh.model_matrix();
                model: primitive.model_matrix(),
                normal1: primitive.normal_matrix().x_axis.extend(0.0).to_array(),
                normal2: primitive.normal_matrix().y_axis.extend(0.0).to_array(),
                normal3: primitive.normal_matrix().z_axis.extend(0.0).to_array(),
            };

            let uniform_alignment =
                device.limits().min_uniform_buffer_offset_alignment as wgpu::BufferAddress;
            let uniform_size = mem::size_of::<ObjectUniform>() as wgpu::BufferAddress;
            let aligned_uniform_size =
                (uniform_size + uniform_alignment - 1) & !(uniform_alignment - 1);
            let uniform_offset = (i as wgpu::BufferAddress) * aligned_uniform_size;

            queue.write_buffer(
                &self.uniform_data.uniform_buffer,
                uniform_offset,
                bytemuck::cast_slice(&[debug_uniform]),
            );

            render_pass.set_bind_group(
                0,
                &self.uniform_data.uniform_bind_group,
                &[uniform_offset as u32],
            );

            // Draw debug mesh (assuming you have a debug_mesh field in your Renderer)
            // self.light.debug_mesh.draw(&mut debug_render_pass);
            primitive.draw(&mut render_pass);
        }

        drop(render_pass);

        queue.submit(Some(encoder.finish()));
    }
}
