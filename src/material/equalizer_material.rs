use super::{MaterialTrait, MaterialType};
use crate::{
    basics::{
        core::Vertex,
        uniforms::{EqualizerUniform, LightUniform, ObjectUniform},
    },
    rendering_utils,
};
use std::mem;
use wgpu::{BindGroup, Buffer, Device, Queue, RenderPipeline, SurfaceConfiguration};

pub struct EqualizerUniforms {
    pub object: ObjectUniform,
    pub equalizer: EqualizerUniform,
    pub light: LightUniform,
}

pub struct EqualizerMaterial {
    render_pipeline: RenderPipeline,
    buffers: [Buffer; 3],
    bind_groups: [BindGroup; 3],
}

impl MaterialTrait for EqualizerMaterial {
    fn render_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
    }

    fn buffers(&self) -> &[Buffer] {
        &self.buffers
    }

    fn bind_groups(&self) -> &[BindGroup] {
        &self.bind_groups
    }

    fn update(&self, queue: &Queue, data: &dyn std::any::Any) {
        if let Some(data) = data.downcast_ref::<EqualizerUniforms>() {
            queue.write_buffer(&self.buffers[0], 0, bytemuck::cast_slice(&[data.object]));
            queue.write_buffer(&self.buffers[1], 0, bytemuck::cast_slice(&[data.equalizer]));
            queue.write_buffer(&self.buffers[2], 0, bytemuck::cast_slice(&[data.light]));
        }
    }

    fn get_id(&self) -> MaterialType {
        MaterialType::Equalizer
    }
}

impl EqualizerMaterial {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let shader = rendering_utils::create_shader_module(device, MaterialType::Equalizer);

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

        // Equalizer uniform, bind group
        let equalizer_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("equalizer_uniform_buffer"),
            size: mem::size_of::<EqualizerUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let equalizer_uniform_bgl =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("equalizer_uniform_bind_group_layout"),
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
        let equalizer_uniform_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("equalizer_uniform_bind_group"),
            layout: &equalizer_uniform_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: equalizer_uniform_buffer.as_entire_binding(),
            }],
        });
        // =========================

        let light_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("light_uniform_buffer"),
            size: mem::size_of::<LightUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let light_uniform_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("light_uniform_bind_group_layout"),
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
        let light_uniform_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("light_uniform_bind_group"),
            layout: &light_uniform_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_uniform_buffer.as_entire_binding(),
            }],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("diffuse_color_render_pipeline_layout"),
                bind_group_layouts: &[
                    &object_uniform_bgl,
                    &equalizer_uniform_bgl,
                    &light_uniform_bgl,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("diffuse_color_render_pipeline"),
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

        let buffers = [
            object_uniform_buffer,
            equalizer_uniform_buffer,
            light_uniform_buffer,
        ];
        let bind_groups = [object_uniform_bg, equalizer_uniform_bg, light_uniform_bg];

        Self {
            render_pipeline,
            buffers,
            bind_groups,
        }
    }
}
