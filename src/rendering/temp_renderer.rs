use crate::{
    basics::{
        core::{GenericUniformData, Vertex},
        material::Material,
        scene2::Scene,
        uniforms::{EqualizerUniform, LightUniform, ObjectUniform, UniformTrait},
    },
    color_utils::{self, ToVec4},
};
use std::{mem, num::NonZeroU64};
use wgpu::{
    BindGroupLayout, Color, CommandEncoderDescriptor, Device, Extent3d, LoadOp, Operations, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, StoreOp, SurfaceConfiguration, TextureView,
};

const BG_COLOR: [f32; 3] = color_utils::CCP.palette[0];

pub struct FillRenderer {}

impl FillRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        depth_texture: &TextureView,
        output_view: &TextureView,
        scene: &Scene,
        generic_uniform_data: &GenericUniformData,
        light_uniform_data: &GenericUniformData,
        wave: &Vec<f32>,
    ) {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("temp_render_encoder"),
        });

        let c_bg_color = color_utils::srgb_to_linear(BG_COLOR, color_utils::GAMMA);
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

        let uniform_alignment =
            device.limits().min_uniform_buffer_offset_alignment as wgpu::BufferAddress;
        let uniform_size = mem::size_of::<ObjectUniform>() as wgpu::BufferAddress;
        let aligned_uniform_size =
            (uniform_size + uniform_alignment - 1) & !(uniform_alignment - 1);

        let light_data = light_uniform_data;
        let light_uniform = LightUniform {
            position: scene.lights[0].transform.position.extend(0.0).to_array(),
            color: scene.lights[0].color.to_vec4(1.0),
        };

        let primitive = &scene.objects[0];
        let object_uniform = ObjectUniform {
            view_proj: scene.camera.build_view_projection_matrix(),
            model: primitive.model_matrix(),
            normal1: primitive.normal_matrix().x_axis.extend(0.0).to_array(),
            normal2: primitive.normal_matrix().y_axis.extend(0.0).to_array(),
            normal3: primitive.normal_matrix().z_axis.extend(0.0).to_array(),
        };
        render_pass.set_pipeline(&primitive.material().render_pipeline);
        let uniform_offset = (0 as wgpu::BufferAddress) * aligned_uniform_size;
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
        let size = Extent3d {
            width: 512,
            height: 1,
            depth_or_array_layers: 1,
        };

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &scene.wave_texture.0,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(&wave),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * 512),
                rows_per_image: Some(1),
            },
            size,
        );
        render_pass.set_bind_group(
            0,
            &generic_uniform_data.uniform_bind_group,
            &[uniform_offset as u32],
        );
        render_pass.set_bind_group(1, &primitive.material().bind_group, &[]);
        render_pass.set_bind_group(2, &scene.wave_texture_bind_group.1, &[]);
        primitive.draw(&mut render_pass);

        // second primitive
        let primitive = &scene.objects[1];
        let object_uniform = ObjectUniform {
            view_proj: scene.camera.build_view_projection_matrix(),
            model: primitive.model_matrix(),
            normal1: primitive.normal_matrix().x_axis.extend(0.0).to_array(),
            normal2: primitive.normal_matrix().y_axis.extend(0.0).to_array(),
            normal3: primitive.normal_matrix().z_axis.extend(0.0).to_array(),
        };
        render_pass.set_pipeline(&primitive.material().render_pipeline);
        let uniform_offset = (1 as wgpu::BufferAddress) * aligned_uniform_size;

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

        drop(render_pass); // also releases encoder

        queue.submit(Some(encoder.finish()));
    }
}

pub fn create_wave_material(
    device: &Device,
    surface_config: &SurfaceConfiguration,
    generic_uniform_data: &GenericUniformData,
    texture_bind_group_layout: &BindGroupLayout,
) -> Material {
    let shader_utils = include_str!("../shaders/utils.wgsl");
    let shader_main = include_str!("../shaders/wave.wgsl");
    let shader_combined = format!("{}\n{}", shader_main, shader_utils);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("screen_quad_shader"),
        source: wgpu::ShaderSource::Wgsl(shader_combined.into()),
    });
    let uniform = Box::new(EqualizerUniform {
        color1: color_utils::CCP.palette[0].to_vec4(1.0),
        color2: color_utils::CCP.palette[1].to_vec4(1.0),
        color3: color_utils::CCP.palette[2].to_vec4(1.0),
        signal: 0.7,
        _padding: [0.0, 0.0, 0.0],
    });

    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("wave_uniform_buffer"),
        size: uniform.get_size() as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let material_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("wave_bind_group_layout"),
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
        label: Some("wave_bind_group"),
        layout: &material_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("wave_pipeline_layout"),
        bind_group_layouts: &[
            &generic_uniform_data.uniform_bind_group_layout,
            &material_bind_group_layout,
            &texture_bind_group_layout,
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
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::Always,
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

    Material {
        uniform,
        uniform_buffer,
        bind_group,
        render_pipeline,
        texture: None,
    }
}
