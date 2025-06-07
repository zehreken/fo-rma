use super::MaterialTrait;
use crate::{
    basics::{
        core::Vertex,
        uniforms::{ColorUniform, ObjectUniform},
    },
    rendering_utils,
};
use std::{mem, sync::Arc};
use wgpu::{
    BindGroup, Buffer, Device, Extent3d, RenderPipeline, SurfaceConfiguration, Texture, TextureView,
};

pub struct WaveUniforms {
    pub object: ObjectUniform,
    pub color1: ColorUniform,
    pub color2: ColorUniform,
    pub wave: Arc<Vec<f32>>,
}

pub struct WaveMaterial {
    render_pipeline: RenderPipeline,
    buffers: [Buffer; 3], // Don't need a buffer for texture
    bind_groups: [BindGroup; 4],
    wave_texture: (Texture, TextureView),
}

impl MaterialTrait for WaveMaterial {
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
        let size = Extent3d {
            width: 512,
            height: 1,
            depth_or_array_layers: 1,
        };
        if let Some(data) = data.downcast_ref::<WaveUniforms>() {
            queue.write_buffer(&self.buffers[0], 0, bytemuck::cast_slice(&[data.object]));
            queue.write_buffer(&self.buffers[1], 0, bytemuck::cast_slice(&[data.color1]));
            queue.write_buffer(&self.buffers[2], 0, bytemuck::cast_slice(&[data.color2]));
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &self.wave_texture.0,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                bytemuck::cast_slice(&data.wave),
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * 512),
                    rows_per_image: Some(1),
                },
                size,
            );
        }
    }

    fn get_id(&self) -> super::Material {
        super::Material::Wave
    }
}

impl WaveMaterial {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let shader = rendering_utils::create_shader_module(device, super::Material::Wave);

        // Object uniform, bind group
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

        // Color uniform, bind group
        let color1_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("color1_uniform_buffer"),
            size: mem::size_of::<ColorUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let color2_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("color1_uniform_buffer"),
            size: mem::size_of::<ColorUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let color_uniform_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("color_uniform_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let color1_uniform_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("color1_uniform_bind_group"),
            layout: &color_uniform_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: color1_uniform_buffer.as_entire_binding(),
            }],
        });

        let color2_uniform_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("color2_uniform_bind_group"),
            layout: &color_uniform_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: color2_uniform_buffer.as_entire_binding(),
            }],
        });
        // =========================

        let wave_texture: (Texture, TextureView) = rendering_utils::create_wave_texture(device);
        let wave_texture_bind_group =
            rendering_utils::create_wave_texture_bind_group(device, &wave_texture.1);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("wave_pipeline_layout"),
                bind_group_layouts: &[
                    &object_uniform_bgl,
                    &color_uniform_bgl,
                    &color_uniform_bgl,
                    &wave_texture_bind_group.0,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("wave_pipeline"),
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
                format: wgpu::TextureFormat::Depth32Float, // must match your render pass
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let buffers = [
            object_uniform_buffer,
            color1_uniform_buffer,
            color2_uniform_buffer,
        ];
        let bind_groups = [
            object_uniform_bg,
            color1_uniform_bg,
            color2_uniform_bg,
            wave_texture_bind_group.1,
        ];

        Self {
            render_pipeline,
            buffers,
            bind_groups,
            wave_texture,
        }
    }
}
