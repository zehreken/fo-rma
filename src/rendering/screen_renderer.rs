use std::num::NonZeroU64;

use image::GenericImageView;
use wgpu::{
    Color, Device, Operations, Queue, RenderPassColorAttachment, SurfaceConfiguration, TextureView,
};

use crate::{
    basics::{
        core::{GenericUniformData, Vertex},
        material::{Material, TextureStuff},
        primitive::Primitive,
        quad::Quad,
        uniforms::{ObjectUniform, ScreenQuadUniform, UniformTrait},
    },
    rendering_utils::create_generic_uniform_data,
};

pub struct ScreenRenderer {
    generic_uniform_data: GenericUniformData,
    screen_quad: Box<dyn Primitive>,
}

impl ScreenRenderer {
    pub fn new(device: &Device, queue: &Queue, surface_config: &SurfaceConfiguration) -> Self {
        let shader_utils = include_str!("../shaders/utils.wgsl");
        let shader_main = include_str!("../shaders/screen_quad.wgsl");
        let shader_combined = format!("{}\n{}", shader_main, shader_utils);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("screen_quad_shader"),
            source: wgpu::ShaderSource::Wgsl(shader_combined.into()),
        });

        let generic_uniform_data = create_generic_uniform_data(&device, &surface_config, 1);

        let material = create_screen_quad_material(
            &device,
            &queue,
            shader,
            &surface_config,
            &generic_uniform_data,
        );
        let screen_quad = Box::new(Quad::new(&device, material));

        Self {
            generic_uniform_data,
            screen_quad,
        }
    }

    pub fn render(&self, device: &Device, queue: &Queue, output_view: &TextureView) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render_encooder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render_pass"),
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

        render_pass.set_pipeline(&self.screen_quad.material().render_pipeline);
        let object_uniform = ObjectUniform {
            view_proj: [[0.0; 4]; 4], // not used in shader
            model: [[0.0; 4]; 4],     // not used in shader
            normal1: [0.0; 4],        // not used in shader
            normal2: [0.0; 4],        // not used in shader
            normal3: [0.0; 4],        // not used in shader
        };
        queue.write_buffer(
            &self.generic_uniform_data.uniform_buffer,
            0,
            bytemuck::cast_slice(&[object_uniform]),
        );
        queue.write_buffer(
            &self.screen_quad.material().uniform_buffer,
            0,
            self.screen_quad.material().uniform.as_bytes(),
        );

        render_pass.set_bind_group(0, &self.generic_uniform_data.uniform_bind_group, &[0]);
        render_pass.set_bind_group(1, &self.screen_quad.material().bind_group, &[]);
        render_pass.set_bind_group(
            2,
            &self
                .screen_quad
                .material()
                .texture
                .as_ref()
                .unwrap()
                .bind_group,
            &[],
        );
        self.screen_quad.draw(&mut render_pass);
        drop(render_pass);

        queue.submit(Some(encoder.finish()));
    }
}

fn create_screen_quad_material(
    device: &Device,
    queue: &Queue,
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

    let texture = create_test_texture(device, queue);

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("screen_quad_pipeline_layout"),
        bind_group_layouts: &[
            &generic_uniform_data.uniform_bind_group_layout,
            &material_bind_group_layout,
            &texture.bind_group_layout,
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
        texture: Some(texture),
    }
}

fn create_test_texture(device: &Device, queue: &Queue) -> TextureStuff {
    let bytes = include_bytes!("../../basic.png");
    let image = image::load_from_memory(bytes).unwrap();
    let rgba = image.to_rgba8();

    let size = wgpu::Extent3d {
        width: image.dimensions().0,
        height: image.dimensions().1,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: Some("test_texture"),
        view_formats: &[],
    });

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                // This should match the filterable field of the
                // corresponding Texture entry above.
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: Some("texture_bind_group_layout"),
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
        label: Some("texture_bind_group"),
    });

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * size.width),
            rows_per_image: Some(size.height),
        },
        size,
    );

    TextureStuff {
        texture,
        rgba,
        size,
        texture_view,
        sampler,
        bind_group_layout,
        bind_group,
    }
}
