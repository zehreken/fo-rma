use crate::{
    audio::sequencer::Sequencer,
    basics::{
        camera::{self, Camera},
        core::GenericUniformData,
        level::Level,
        light::Light,
        uniforms::{LightUniform, ObjectUniform},
    },
    gui::Gui,
    rendering_utils,
    utils::{self, ToVec4},
};
use glam::vec3;
use std::mem;
use wgpu::{
    Color, CommandEncoderDescriptor, Device, LoadOp, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, StoreOp, Surface, SurfaceConfiguration, SurfaceError,
    TextureView, TextureViewDescriptor,
};
use winit::{dpi::PhysicalSize, window::Window};

const BG_COLOR: [f32; 3] = utils::CCP.palette[0];

pub struct Renderer<'a> {
    surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
    pub surface_config: SurfaceConfiguration,
    pub gui: Gui,
    pub camera: Camera,
    pub debug_render_pipeline: RenderPipeline,
    light: Light,
    pub light_uniform_data: GenericUniformData,
    pub depth_texture: TextureView,
    pub generic_uniform_data: GenericUniformData,
    pub debug_uniform_data: GenericUniformData,
}

impl<'a> Renderer<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let size = window.inner_size();
        let (instance, surface) = rendering_utils::create_instance_and_surface(window);
        let adapter = rendering_utils::create_adapter(instance, &surface).await;
        let (device, queue) = rendering_utils::create_device_and_queue(&adapter).await;
        let surface_caps = surface.get_capabilities(&adapter);
        let texture_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let surface_config =
            rendering_utils::create_surface_config(size, texture_format, surface_caps);
        surface.configure(&device, &surface_config);
        // camera ============
        let camera = camera::Camera::new(
            vec3(0.0, 0.0, 2.0),
            vec3(0.0, 0.0, 0.0),
            size.width as f32 / size.height as f32,
            45.0,
            0.1,
            100.0,
        );
        let gui = Gui::new(&window, &device, texture_format);
        // Light uniform
        // let mut light = Light::new([1.0, 0.678, 0.003]);
        let mut light = Light::new([1.0, 1.0, 1.0]);
        light.update_position(vec3(2.0, 0.0, 2.0));
        let light_uniform_data = rendering_utils::create_light_uniform_data(&device);
        // I might need to pass this to create_render_pipeline function

        // =============
        // Debug
        let primitive_count = 25;
        let (debug_uniform_data, debug_render_pipeline) =
            rendering_utils::create_debug_uniform_data(&device, &surface_config, primitive_count);

        // =============
        let generic_uniform_data =
            rendering_utils::create_generic_uniform_data(&device, &surface_config, primitive_count);
        // =============
        let depth_texture = rendering_utils::create_depth_texture(&device, &surface_config);
        // let render_pipeline = create_render_pipeline(
        //     &device,
        //     &surface_config,
        //     &generic_uniform_data.uniform_bind_group_layout,
        //     &light_uniform_data.uniform_bind_group_layout,
        // );

        println!("Surface format: {:?}", surface_config.format);

        Self {
            surface,
            device,
            queue,
            surface_config,
            gui,
            camera,
            // render_pipeline,
            debug_render_pipeline,
            light,
            light_uniform_data,
            depth_texture,
            generic_uniform_data,
            debug_uniform_data,
        }
    }

    pub fn render(
        &mut self,
        window: &Window,
        level: &Level,
        elapsed: f32,
        delta_time: f32,
        fps: f32,
        sequencer: &mut Sequencer,
    ) -> Result<(), SurfaceError> {
        let output_frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(SurfaceError::Outdated) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        };

        let output_view = output_frame
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("render_encoder"),
            });

        let c_bg_color = utils::srgb_to_linear(BG_COLOR, utils::GAMMA);
        let bg_color = Color {
            r: c_bg_color[0] as f64,
            g: c_bg_color[1] as f64,
            b: c_bg_color[2] as f64,
            a: 1.0,
        };

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("render_pass"),
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
            self.device.limits().min_uniform_buffer_offset_alignment as wgpu::BufferAddress;
        let uniform_size = mem::size_of::<ObjectUniform>() as wgpu::BufferAddress;
        let aligned_uniform_size =
            (uniform_size + uniform_alignment - 1) & !(uniform_alignment - 1);

        // self.camera
        //     .update_position(vec3(5.0 * elapsed.cos(), 0.0, 5.0 * elapsed.sin()));
        let el = elapsed * 0.5;
        self.light
            .update_position(vec3(2.0 * el.cos(), 0.0, 2.0 * el.sin()));

        let light_data = &mut self.light_uniform_data;
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

            self.queue.write_buffer(
                &self.generic_uniform_data.uniform_buffer,
                uniform_offset,
                bytemuck::cast_slice(&[object_uniform]),
            );
            self.queue.write_buffer(
                &light_data.uniform_buffer,
                0,
                bytemuck::cast_slice(&[light_uniform]),
            );
            self.queue.write_buffer(
                &primitive.material().uniform_buffer,
                0,
                &primitive.material().uniform.as_bytes(),
            );
            render_pass.set_bind_group(
                0,
                &self.generic_uniform_data.uniform_bind_group,
                &[uniform_offset as u32],
            );
            render_pass.set_bind_group(1, &light_data.uniform_bind_group, &[]);
            render_pass.set_bind_group(2, &primitive.material().bind_group, &[]);
            primitive.draw(&mut render_pass);
        }

        drop(render_pass); // also releases encoder

        // debug render pass
        const DEBUG: bool = false;
        if DEBUG {
            let mut debug_render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("debug_render_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &output_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load,
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

            debug_render_pass.set_pipeline(&self.debug_render_pipeline);

            // Update debug uniforms
            for (i, primitive) in level.objects.iter().enumerate() {
                let debug_uniform = ObjectUniform {
                    view_proj: self.camera.build_view_projection_matrix(),
                    // debug_uniforms.model = self.light.debug_mesh.model_matrix();
                    model: primitive.model_matrix(),
                    normal1: primitive.normal_matrix().x_axis.extend(0.0).to_array(),
                    normal2: primitive.normal_matrix().y_axis.extend(0.0).to_array(),
                    normal3: primitive.normal_matrix().z_axis.extend(0.0).to_array(),
                };
                let uniform_offset = (i as wgpu::BufferAddress) * aligned_uniform_size;

                self.queue.write_buffer(
                    &self.debug_uniform_data.uniform_buffer,
                    uniform_offset,
                    bytemuck::cast_slice(&[debug_uniform]),
                );

                debug_render_pass.set_bind_group(
                    0,
                    &self.debug_uniform_data.uniform_bind_group,
                    &[uniform_offset as u32],
                );

                // Draw debug mesh (assuming you have a debug_mesh field in your Renderer)
                // self.light.debug_mesh.draw(&mut debug_render_pass);
                primitive.draw(&mut debug_render_pass);
            }
        }
        // =================

        // =================

        // render gui on top
        self.gui.render(
            &window,
            &output_view,
            &self.device,
            &self.queue,
            &mut encoder,
            sequencer,
            fps,
        );
        // =====================

        self.queue.submit(Some(encoder.finish()));
        output_frame.present();
        Ok(())
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>, scale_factor: f64) {
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;
        self.surface.configure(&self.device, &self.surface_config);
        self.depth_texture =
            rendering_utils::create_depth_texture(&self.device, &self.surface_config);
        self.camera.resize(size);
        self.gui.resize(size, scale_factor)
    }
}
