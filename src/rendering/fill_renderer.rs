use std::mem;

use crate::{
    basics::{
        camera::{self, Camera},
        core::GenericUniformData,
        level::Level,
        light::Light,
        uniforms::{LightUniform, ObjectUniform},
    },
    rendering_utils,
    utils::{self, ToVec4},
};
use glam::vec3;
use wgpu::{
    Color, CommandEncoderDescriptor, Device, LoadOp, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, SurfaceConfiguration, TextureView,
};
use winit::dpi::PhysicalSize;

const BG_COLOR: [f32; 3] = utils::CCP.palette[0];

pub struct FillRenderer {
    pub camera: Camera,
    pub light: Light,
    pub depth_texture: TextureView,
}

impl FillRenderer {
    pub fn new(
        device: &Device,
        surface_config: &SurfaceConfiguration,
        size: PhysicalSize<u32>,
    ) -> Self {
        // camera ============
        let camera = camera::Camera::new(
            vec3(0.0, 0.0, 2.0),
            vec3(0.0, 0.0, 0.0),
            size.width as f32 / size.height as f32,
            45.0,
            0.1,
            100.0,
        );
        // Light uniform
        // let mut light = Light::new([1.0, 0.678, 0.003]);
        let mut light = Light::new([1.0, 1.0, 1.0]);
        light.update_position(vec3(2.0, 0.0, 2.0));
        // I might need to pass this to create_render_pipeline function
        // =============
        let depth_texture = rendering_utils::create_depth_texture(&device, &surface_config);

        Self {
            camera,
            light,
            depth_texture,
        }
    }

    pub fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        output_view: &TextureView,
        elapsed: f32,
        level: &Level,
        generic_uniform_data: &GenericUniformData,
        light_uniform_data: &GenericUniformData,
    ) {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("fill_render_encoder"),
        });

        let c_bg_color = utils::srgb_to_linear(BG_COLOR, utils::GAMMA);
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
                view: &self.depth_texture,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        // render_pass.set_pipeline(&self.render_pipeline);

        let uniform_alignment =
            device.limits().min_uniform_buffer_offset_alignment as wgpu::BufferAddress;
        let uniform_size = mem::size_of::<ObjectUniform>() as wgpu::BufferAddress;
        let aligned_uniform_size =
            (uniform_size + uniform_alignment - 1) & !(uniform_alignment - 1);

        // self.camera
        //     .update_position(vec3(5.0 * elapsed.cos(), 0.0, 5.0 * elapsed.sin()));
        let el = elapsed * 0.5;
        self.light
            .update_position(vec3(2.0 * el.cos(), 0.0, 2.0 * el.sin()));

        let light_data = light_uniform_data;
        let light_uniform = LightUniform {
            position: self.light.transform.position.extend(0.0).to_array(),
            color: self.light.color.to_vec4(1.0),
        };

        for (i, primitive) in level.objects.iter().enumerate() {
            let object_uniform = ObjectUniform {
                view_proj: self.camera.build_view_projection_matrix(),
                model: primitive.model_matrix(),
                normal1: primitive.normal_matrix().x_axis.extend(0.0).to_array(),
                normal2: primitive.normal_matrix().y_axis.extend(0.0).to_array(),
                normal3: primitive.normal_matrix().z_axis.extend(0.0).to_array(),
            };
            render_pass.set_pipeline(&primitive.material().render_pipeline);
            let uniform_offset = (i as wgpu::BufferAddress) * aligned_uniform_size;

            queue.write_buffer(
                &generic_uniform_data.uniform_buffer,
                uniform_offset,
                bytemuck::cast_slice(&[object_uniform]),
            );
            queue.write_buffer(
                &light_data.uniform_buffer,
                0,
                bytemuck::cast_slice(&[light_uniform]),
            );
            queue.write_buffer(
                &primitive.material().uniform_buffer,
                0,
                &primitive.material().uniform.as_bytes(),
            );
            render_pass.set_bind_group(
                0,
                &generic_uniform_data.uniform_bind_group,
                &[uniform_offset as u32],
            );
            render_pass.set_bind_group(1, &light_data.uniform_bind_group, &[]);
            render_pass.set_bind_group(2, &primitive.material().bind_group, &[]);
            primitive.draw(&mut render_pass);
        }

        drop(render_pass); // also releases encoder

        queue.submit(Some(encoder.finish()));
    }
}
