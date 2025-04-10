use crate::{basics::core::Vertex, rendering_utils};
use wgpu::{
    BindGroup, BindGroupLayout, Device, PipelineLayout, RenderPipeline, SurfaceConfiguration,
    Texture, TextureView,
};
use winit::dpi::PhysicalSize;

pub struct RenderPostProcessTextureMaterial {
    pub render_texture: Texture,
    pub render_texture_view: TextureView,
    pub post_process_texture: Texture,
    pub post_process_texture_view: TextureView,
    pub texture_bind_group_layout: BindGroupLayout,
    pub texture_bind_group: BindGroup,
    pub render_pipeline_layout: PipelineLayout,
    pub render_pipeline: RenderPipeline,
}

impl RenderPostProcessTextureMaterial {
    pub fn new(
        device: &Device,
        surface_config: &SurfaceConfiguration,
        size: PhysicalSize<u32>,
    ) -> Self {
        let (render_texture, render_texture_view) =
            rendering_utils::create_render_texture(device, &surface_config.format, size);
        let (post_process_texture, post_process_texture_view) =
            rendering_utils::create_post_process_texture(device, size);
        let (texture_bind_group_layout, texture_bind_group) =
            rendering_utils::create_texture_bind_group(device, &post_process_texture_view);
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("screen_quad_pipeline_layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader_utils = include_str!("../shaders/utils.wgsl");
        let shader_main = include_str!("../shaders/screen_quad.wgsl");
        let shader_combined = format!("{}\n{}", shader_main, shader_utils);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("screen_quad_shader"),
            source: wgpu::ShaderSource::Wgsl(shader_combined.into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("screen_render_pipeline"),
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

        Self {
            render_texture,
            render_texture_view,
            post_process_texture,
            post_process_texture_view,
            texture_bind_group_layout,
            texture_bind_group,
            render_pipeline_layout,
            render_pipeline,
        }
    }

    pub fn resize(
        &mut self,
        device: &Device,
        surface_config: &SurfaceConfiguration,
        size: PhysicalSize<u32>,
    ) {
        (self.render_texture, self.render_texture_view) =
            rendering_utils::create_render_texture(device, &surface_config.format, size);
        (self.post_process_texture, self.post_process_texture_view) =
            rendering_utils::create_post_process_texture(device, size);
        (self.texture_bind_group_layout, self.texture_bind_group) =
            rendering_utils::create_texture_bind_group(device, &self.post_process_texture_view);
    }
}
