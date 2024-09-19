use glam::vec3;
use wgpu::{
    util::DeviceExt, BindGroup, Buffer, Color, CommandEncoderDescriptor, Device, LoadOp,
    Operations, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, StoreOp,
    Surface, SurfaceCapabilities, SurfaceConfiguration, SurfaceError, TextureFormat,
    TextureViewDescriptor,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    basics::{
        camera::{self, Camera},
        core::{Uniforms, Vertex},
        cube,
        primitive::Primitive,
        quad, triangle,
    },
    gui::Gui,
};

pub struct Renderer<'a> {
    surface: Surface<'a>,
    pub device: Device,
    queue: Queue,
    surface_config: SurfaceConfiguration,
    pub gui: Gui,
    camera: Camera,
    uniforms: Uniforms,
    uniform_buffer: Buffer,
    uniform_bind_group: BindGroup,
    render_pipeline: RenderPipeline,
}

impl<'a> Renderer<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let size = window.inner_size();
        let (instance, surface) = create_instance_and_surface(window);
        let adapter = create_adapter(instance, &surface).await;
        let (device, queue) = create_device_and_queue(&adapter).await;
        let surface_caps = surface.get_capabilities(&adapter);
        let texture_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let surface_config = create_surface_config(size, texture_format, surface_caps);
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
        let uniforms = Uniforms::new();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniform_buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });
        // ===================
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/basic.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render_pipeline_layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        Self {
            surface,
            device,
            queue,
            surface_config,
            gui,
            camera,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            render_pipeline,
        }
    }

    pub fn render(
        &mut self,
        window: &Window,
        primitives: &Vec<Box<dyn Primitive>>,
        elapsed: f32,
        delta_time: f32,
        fps: f32,
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

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("render_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.3,
                        g: 0.3,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.camera
            .update_position(vec3(2.0 * elapsed.cos(), 0.0, 2.0 * elapsed.sin()));
        self.uniforms.view_proj = self.camera.build_view_projection_matrix();
        self.uniforms.model = primitives[0].model_matrix();
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        // render some meshes
        // cube.draw(&mut render_pass);
        for primitive in primitives {
            primitive.draw(&mut render_pass);
        }

        drop(render_pass); // also releases encoder

        // =================

        // render gui on top
        self.gui.render(
            &window,
            &output_view,
            &self.device,
            &self.queue,
            &mut encoder,
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
        self.gui.resize(size, scale_factor)
    }
}

fn create_instance_and_surface(
    window: &winit::window::Window,
) -> (wgpu::Instance, wgpu::Surface<'static>) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });
    let surface = unsafe {
        instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(window).unwrap())
    }
    .unwrap();
    (instance, surface)
}

async fn create_adapter(instance: wgpu::Instance, surface: &wgpu::Surface<'_>) -> wgpu::Adapter {
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();
    adapter
}

async fn create_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::default(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .unwrap();
    (device, queue)
}

fn create_surface_config(
    size: PhysicalSize<u32>,
    texture_format: TextureFormat,
    surface_caps: SurfaceCapabilities,
) -> wgpu::SurfaceConfiguration {
    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: texture_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface_config
}
