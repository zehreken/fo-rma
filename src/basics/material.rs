use super::{core::Vertex, uniforms::UniformTrait};
use image::{ImageBuffer, Rgba};
use std::num::NonZeroU64;
use wgpu::{
    BindGroup, BindGroupLayout, Buffer, Device, Extent3d, RenderPipeline, Sampler,
    SurfaceConfiguration, Texture, TextureView,
};

pub struct Material {
    pub uniform: Box<dyn UniformTrait>,
    pub uniform_buffer: Buffer,
    pub bind_group: BindGroup,
    pub render_pipeline: RenderPipeline,
    pub texture: Option<TextureStuff>,
}

pub struct TextureStuff {
    pub texture: Texture,
    pub rgba: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub size: Extent3d,
    pub texture_view: TextureView,
    pub sampler: Sampler,
    pub bind_group_layout: BindGroupLayout,
    pub bind_group: BindGroup,
}

impl Material {
    pub fn new(
        device: &Device,
        surface_config: &SurfaceConfiguration,
        generic_bind_group_layout: &BindGroupLayout,
        light_bind_group_layout: &BindGroupLayout,
        shader_main: &str,
        shader_name: &str,
        uniform: Box<dyn UniformTrait>,
    ) -> Self {
        let shader_utils = include_str!("../shaders/utils.wgsl");
        let shader_combined = format!("{}\n{}", shader_main, shader_utils);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(shader_name),
            source: wgpu::ShaderSource::Wgsl(shader_combined.into()),
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("material_uniform_buffer"),
            size: uniform.get_size() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let material_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("material_bind_group_layout"),
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

        let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("material_bind_group"),
            layout: &material_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render_pipeline_layout"),
                bind_group_layouts: &[
                    generic_bind_group_layout,
                    light_bind_group_layout,
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

        Self {
            uniform,
            uniform_buffer,
            bind_group: material_bind_group,
            render_pipeline,
            texture: None,
        }
    }
}
