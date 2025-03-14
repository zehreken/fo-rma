use std::num::{NonZero, NonZeroU64};

use wgpu::{
    Color, Device, Operations, Queue, RenderPassColorAttachment, RenderPipeline, Surface,
    SurfaceConfiguration, SurfaceError,
};
use winit::window::Window;

use crate::{
    basics::{
        core::{GenericUniformData, Vertex},
        material::Material,
        primitive::Primitive,
        quad::Quad,
        uniforms::{ObjectUniform, ScreenQuadUniform, UniformTrait},
    },
    rendering_utils::{self, create_generic_uniform_data},
    utils,
};

pub struct Renderer<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    generic_uniform_data: GenericUniformData,
    screen_quad: Box<dyn Primitive>,
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

        let shader_utils = include_str!("shaders/utils.wgsl");
        let shader_main = include_str!("shaders/screen_quad.wgsl");
        let shader_combined = format!("{}\n{}", shader_main, shader_utils);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("screen_quad_shader"),
            source: wgpu::ShaderSource::Wgsl(shader_combined.into()),
        });

        let generic_uniform_data = create_generic_uniform_data(&device, &surface_config, 1);

        let material =
            create_screen_quad_material(&device, shader, &surface_config, &generic_uniform_data);
        let screen_quad = Box::new(Quad::new(&device, material));

        Self {
            surface,
            device,
            queue,
            generic_uniform_data,
            screen_quad,
        }
    }

    pub fn render(&mut self, window: &Window) -> Result<(), SurfaceError> {
        let output_frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(SurfaceError::Outdated) => return Ok(()),
            Err(e) => return Err(e),
        };

        let output_view = output_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render_encooder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &output_view,
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

        render_pass.set_pipeline(&self.screen_quad.material().render_pipeline);
        let object_uniform = ObjectUniform {
            view_proj: [[0.0; 4]; 4], // not used in shader
            model: [[0.0; 4]; 4],     // not used in shader
            normal1: [0.0; 4],        // not used in shader
            normal2: [0.0; 4],        // not used in shader
            normal3: [0.0; 4],        // not used in shader
        };
        self.queue.write_buffer(
            &self.generic_uniform_data.uniform_buffer,
            0,
            bytemuck::cast_slice(&[object_uniform]),
        );
        self.queue.write_buffer(
            &self.screen_quad.material().uniform_buffer,
            0,
            self.screen_quad.material().uniform.as_bytes(),
        );
        render_pass.set_bind_group(0, &self.generic_uniform_data.uniform_bind_group, &[0]);
        render_pass.set_bind_group(1, &self.screen_quad.material().bind_group, &[]);
        self.screen_quad.draw(&mut render_pass);
        drop(render_pass);

        self.queue.submit(Some(encoder.finish()));
        output_frame.present();
        Ok(())
    }
}

fn create_screen_quad_material(
    device: &Device,
    shader: wgpu::ShaderModule,
    surface_config: &SurfaceConfiguration,
    generic_uniform_data: &GenericUniformData,
) -> Material {
    let uniform = Box::new(ScreenQuadUniform { signal: [0.0; 4] });

    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("screen_quad_uniform_buffer"),
        size: uniform.get_size() as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let material_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("screen_quad_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: Some(NonZeroU64::new(uniform.get_size() as u64).unwrap()),
                },
                count: None,
            }],
        });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("screen_quad_bind_group"),
        layout: &material_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("screen_quad_pipeline_layout"),
        bind_group_layouts: &[
            &generic_uniform_data.uniform_bind_group_layout,
            &material_bind_group_layout,
        ],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("render_pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
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
                    wgpu::VertexAttribute {
                        offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                        shader_location: 2,
                        format: wgpu::VertexFormat::Float32x3,
                    },
                    wgpu::VertexAttribute {
                        offset: std::mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
                        shader_location: 3,
                        format: wgpu::VertexFormat::Float32x2,
                    },
                ],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
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

    Material {
        uniform,
        uniform_buffer,
        bind_group,
        render_pipeline,
    }
}
