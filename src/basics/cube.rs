use super::core::Vertex;
use glam::{Mat4, Vec3};
use wgpu::{util::DeviceExt, Device, Queue, RenderPass, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    // Front face
    Vertex { position: [-0.5, -0.5,  0.5], color: [ 0.0,  0.0,  1.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [ 0.0,  0.0,  1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [ 0.0,  0.0,  1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [ 0.0,  0.0,  1.0] },
    // Back face
    Vertex { position: [-0.5, -0.5, -0.5], color: [ 0.0,  0.0, -1.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [ 0.0,  0.0, -1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [ 0.0,  0.0, -1.0] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [ 0.0,  0.0, -1.0] },
    // Right face
    Vertex { position: [ 0.5, -0.5, -0.5], color: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [ 1.0,  0.0,  0.0] },
    // Left face
    Vertex { position: [-0.5, -0.5, -0.5], color: [-1.0,  0.0,  0.0] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [-1.0,  0.0,  0.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [-1.0,  0.0,  0.0] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [-1.0,  0.0,  0.0] },
    // Top face
    Vertex { position: [-0.5,  0.5, -0.5], color: [ 0.0,  1.0,  0.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [ 0.0,  1.0,  0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [ 0.0,  1.0,  0.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [ 0.0,  1.0,  0.0] },
    // Bottom face
    Vertex { position: [-0.5, -0.5, -0.5], color: [ 0.0, -1.0,  0.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [ 0.0, -1.0,  0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [ 0.0, -1.0,  0.0] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [ 0.0, -1.0,  0.0] },
];

const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, // front
    4, 5, 6, 6, 7, 4, // back
    8, 9, 10, 10, 11, 8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // top
    20, 21, 22, 22, 23, 20, // bottom
];

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    model_view_proj: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            model_view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update(&mut self, width: u32, height: u32) {
        let aspect = width as f32 / height as f32;
        let proj = Mat4::orthographic_rh(-aspect, aspect, -1.0, 1.0, 0.1, 100.0);
        let view = Mat4::look_at_rh(Vec3::new(10.0, 10.0, 10.0), Vec3::ZERO, Vec3::Y);
        let model = Mat4::from_rotation_y(std::f32::consts::FRAC_PI_4);
        self.model_view_proj = (proj * view * model).to_cols_array_2d();
    }
}

pub struct State {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniforms: Uniforms,
}

impl State {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/cube.wgsl").into()),
        });

        let mut uniforms = Uniforms::new();
        uniforms.update(1600, 1200);

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

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            uniform_buffer,
            uniform_bind_group,
            uniforms,
        }
    }

    pub fn update(&mut self, queue: &Queue) {
        // self.rotation += 0.01;
        // println!("{}", self.rotation);
        // // let aspect_ratio = self.config.width as f32 / self.config.height as f32;
        // let aspect_ratio = 1.0;

        // // Projection matrix
        // let proj = Mat4::perspective_rh(45.0f32.to_radians(), aspect_ratio, 0.1, 100.0);

        // // View matrix
        // let view = Mat4::look_at_rh(
        //     Vec3::new(2.0, 2.0, 2.0), // Eye position
        //     Vec3::ZERO,               // Look at point
        //     Vec3::Y,                  // Up direction
        // );

        // // Model matrix (rotation around Y-axis)
        // let model = Mat4::from_rotation_y(self.rotation);

        // // Combine matrices
        // let mvp = proj * view * model;

        // // Update uniform buffer
        // queue.write_buffer(
        //     &self.uniform_buffer,
        //     0,
        //     bytemuck::cast_slice(&mvp.to_cols_array_2d()),
        // );
    }

    pub fn render<'a>(&'a mut self, queue: &Queue, render_pass: &mut RenderPass<'a>) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.uniforms.update(size.width, size.height)
    }
}
