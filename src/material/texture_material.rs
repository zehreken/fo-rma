use crate::{
    basics::{core::Vertex, uniforms::ObjectUniform},
    material::{MaterialTrait, MaterialType},
    rendering_utils,
};
use image::{ImageBuffer, Rgba};
use std::mem;
use wgpu::{
    BindGroup, Buffer, Device, Extent3d, Queue, RenderPipeline, SurfaceConfiguration, Texture,
    TextureView,
};
use winit::dpi::PhysicalSize;

pub struct TextureUniforms {
    pub object: ObjectUniform,
}

pub struct TextureMaterial {
    render_pipeline: RenderPipeline,
    buffers: [Buffer; 1],
    bind_groups: [BindGroup; 2],
    // texture: (Texture, TextureView),
    // image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl MaterialTrait for TextureMaterial {
    fn render_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
    }

    fn buffers(&self) -> &[Buffer] {
        &self.buffers
    }

    fn bind_groups(&self) -> &[BindGroup] {
        &self.bind_groups
    }

    fn update(&self, queue: &wgpu::Queue, data: &dyn std::any::Any) {
        if let Some(data) = data.downcast_ref::<TextureUniforms>() {
            queue.write_buffer(&self.buffers[0], 0, bytemuck::cast_slice(&[data.object]));
        }
    }

    fn get_id(&self) -> MaterialType {
        MaterialType::Texture
    }
}

impl TextureMaterial {
    pub fn new(device: &Device, queue: &Queue, surface_config: &SurfaceConfiguration) -> Self {
        let diffuse_bytes = include_bytes!("../../textures/uv.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();
        use image::GenericImageView;
        let dimensions = diffuse_image.dimensions();

        let shader = rendering_utils::create_shader_module(device, MaterialType::Texture);

        let object_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("object_uniform_buffer"),
            size: mem::size_of::<ObjectUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let object_uniform_bgl =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("object_uniform_bind_group_layout"),
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
            });
        let object_uniform_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("object_uniform_bind_group"),
            layout: &object_uniform_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: object_uniform_buffer.as_entire_binding(),
            }],
        });

        // Texture
        let size = PhysicalSize::new(dimensions.0, dimensions.1);
        let (texture, texture_view) =
            rendering_utils::create_texture(device, &surface_config.format, size);

        let (texture_bind_group_layout, texture_bind_group) =
            rendering_utils::create_texture_bind_group(device, &texture_view);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("screen_quad_pipeline_layout"),
                bind_group_layouts: &[&object_uniform_bgl, &texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("texture_render_pipeline"),
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let size = Extent3d {
            width: 512,
            height: 512,
            depth_or_array_layers: 1,
        };
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * 512),
                rows_per_image: Some(512),
            },
            size,
        );

        let buffers = [object_uniform_buffer];
        let bind_groups = [object_uniform_bg, texture_bind_group];

        Self {
            render_pipeline,
            buffers,
            bind_groups,
            // texture: (texture, texture_view),
            // image_buffer: diffuse_rgba,
        }
    }
}
