use crate::{
    basics::{
        primitive::Primitive,
        scene::Scene,
        uniforms::{ColorUniform, ObjectUniform},
    },
    material::{debug_material::DebugMaterial, MaterialTrait},
    renderer::PRIMITIVE_COUNT,
};
use wgpu::{
    CommandEncoderDescriptor, Device, LoadOp, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, SurfaceConfiguration, TextureView,
};

pub struct LineRenderer {
    debug_materials: Vec<DebugMaterial>,
}

impl LineRenderer {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let mut debug_materials = vec![];
        // Create one debug material for each primitive
        for _ in 0..PRIMITIVE_COUNT {
            let debug_material = DebugMaterial::new(device, surface_config);
            debug_materials.push(debug_material);
        }

        Self { debug_materials }
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

        render_pass.set_pipeline(&self.debug_materials[0].render_pipeline()); // Set pipeline once
        let flat: Vec<&Box<dyn Primitive>> = level
            .material_object_map
            .values()
            .flat_map(|v| v.iter())
            .collect();
        let color = ColorUniform {
            color: [1.0, 0.0, 0.0, 1.0],
        };
        for (i, primitive) in flat.iter().enumerate() {
            let object = ObjectUniform {
                view_proj: level.camera.build_view_projection_matrix(),
                model: primitive.model_matrix(),
                normal1: primitive.normal_matrix().x_axis.extend(0.0).to_array(),
                normal2: primitive.normal_matrix().y_axis.extend(0.0).to_array(),
                normal3: primitive.normal_matrix().z_axis.extend(0.0).to_array(),
            };

            queue.write_buffer(
                &self.debug_materials[i].buffers()[0],
                0,
                bytemuck::cast_slice(&[object]),
            );
            queue.write_buffer(
                &self.debug_materials[i].buffers()[1],
                0,
                bytemuck::cast_slice(&[color]),
            );

            render_pass.set_bind_group(0, &self.debug_materials[i].bind_groups()[0], &[]);
            render_pass.set_bind_group(1, &self.debug_materials[i].bind_groups()[1], &[]);

            primitive.draw(&mut render_pass);
        }

        drop(render_pass);

        queue.submit(Some(encoder.finish()));
    }
}
