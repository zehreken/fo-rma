use crate::basics::{
    core::{GenericUniformData, Vertex},
    uniforms::{LightUniform, ObjectUniform},
};
use std::{mem, num::NonZeroU64};
use wgpu::{
    BindGroup, BindGroupLayout, Device, RenderPipeline, SurfaceCapabilities, SurfaceConfiguration,
    Texture, TextureFormat, TextureView,
};
use winit::dpi::PhysicalSize;

pub fn create_instance_and_surface(
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

pub async fn create_adapter(
    instance: wgpu::Instance,
    surface: &wgpu::Surface<'_>,
) -> wgpu::Adapter {
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

pub fn create_surface_config(
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

pub fn create_depth_texture(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width: config.width,
        height: config.height,
        depth_or_array_layers: 1,
    };
    let desc = wgpu::TextureDescriptor {
        label: Some("depth_texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    };
    let texture = device.create_texture(&desc);
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}

pub async fn create_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::POLYGON_MODE_LINE,
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .unwrap();
    (device, queue)
}

pub fn create_render_texture(
    device: &Device,
    texture_format: &TextureFormat,
    size: PhysicalSize<u32>,
) -> (Texture, TextureView) {
    let size = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("test_texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: *texture_format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    (texture, texture_view)
}

pub fn create_post_process_texture(
    device: &Device,
    size: PhysicalSize<u32>,
) -> (Texture, TextureView) {
    let post_process_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("post_process_texture"),
        size: wgpu::Extent3d {
            width: size.width,
            height: size.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm, // not sRGB!
        usage: wgpu::TextureUsages::STORAGE_BINDING
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });

    let post_process_view = post_process_texture.create_view(&Default::default());

    (post_process_texture, post_process_view)
}

pub fn create_wave_texture(device: &Device) -> (Texture, TextureView) {
    let wave_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("wave_texture"),
        size: wgpu::Extent3d {
            width: 512,
            height: 1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::R32Float,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let wave_view = wave_texture.create_view(&Default::default());

    (wave_texture, wave_view)
}

pub fn create_light_uniform_data(device: &Device) -> GenericUniformData {
    let light_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("light_uniform_buffer"),
        size: mem::size_of::<LightUniform>() as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let light_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            label: None,
        });

    let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &light_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: light_uniform_buffer.as_entire_binding(),
        }],
        label: Some("light_bind_group"),
    });

    GenericUniformData {
        uniform_buffer: light_uniform_buffer,
        uniform_bind_group: light_bind_group,
        uniform_bind_group_layout: light_bind_group_layout,
    }
}

pub fn create_generic_uniform_data(
    device: &Device,
    surface_config: &SurfaceConfiguration, /* include shader variant */
    primitive_count: u64,
) -> GenericUniformData {
    let uniform_alignment =
        device.limits().min_uniform_buffer_offset_alignment as wgpu::BufferAddress;
    let uniform_size = mem::size_of::<ObjectUniform>() as wgpu::BufferAddress;
    let aligned_uniform_size = (uniform_size + uniform_alignment - 1) & !(uniform_alignment - 1);
    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("uniform_buffer"),
        size: aligned_uniform_size * primitive_count,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let uniform_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: Some(NonZeroU64::new(uniform_size as u64).unwrap()),
                },
                count: None,
            }],
            label: Some("uniform_bind_group_layout"),
        });
    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &uniform_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &uniform_buffer,
                offset: 0,
                size: Some(NonZeroU64::new(uniform_size as u64).unwrap()),
            }),
        }],
        label: Some("uniform_bind_group"),
    });

    GenericUniformData {
        uniform_buffer,
        uniform_bind_group,
        uniform_bind_group_layout,
    }
}

pub fn create_debug_uniform_data(
    device: &Device,
    surface_config: &SurfaceConfiguration,
    primitive_count: u64,
) -> (GenericUniformData, RenderPipeline) {
    let uniform_alignment =
        device.limits().min_uniform_buffer_offset_alignment as wgpu::BufferAddress;
    let uniform_size = mem::size_of::<ObjectUniform>() as wgpu::BufferAddress;
    let aligned_uniform_size = (uniform_size + uniform_alignment - 1) & !(uniform_alignment - 1);

    let shader_debug = include_str!("shaders/debug.wgsl");
    let shader_utils = include_str!("shaders/utils.wgsl");
    let shader_combined = format!("{}\n{}", shader_debug, shader_utils);
    let debug_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("debug_shader"),
        source: wgpu::ShaderSource::Wgsl(shader_combined.into()),
    });

    let debug_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("debug_uniform_buffer"),
        size: aligned_uniform_size * primitive_count,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let debug_uniform_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: Some(NonZeroU64::new(uniform_size as u64).unwrap()),
                },
                count: None,
            }],
            label: Some("debug_uniform_bind_group_layout"),
        });
    let debug_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("debug_uniform_bind_group"),
        layout: &debug_uniform_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &debug_uniform_buffer,
                offset: 0,
                size: Some(NonZeroU64::new(uniform_size as u64).unwrap()),
            }),
        }],
    });

    let debug_render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("debug_render_pipeline_layout"),
            bind_group_layouts: &[&debug_uniform_bind_group_layout],
            push_constant_ranges: &[],
        });
    let debug_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("debug_render_pipeline"),
        layout: Some(&debug_render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &debug_shader,
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
                ],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &debug_shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_config.format,
                // format: TextureFormat::Rgba8Unorm,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Line,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Always,
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

    (
        GenericUniformData {
            uniform_buffer: debug_uniform_buffer,
            uniform_bind_group: debug_uniform_bind_group,
            uniform_bind_group_layout: debug_uniform_bind_group_layout,
        },
        debug_render_pipeline,
    )
}

pub fn create_render_texture_bind_group(
    device: &Device,
    render_texture: &TextureView,
) -> (BindGroupLayout, BindGroup) {
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
        label: Some("texture_bind_group_layout"),
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
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("texture_bind_group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&render_texture),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    });

    (bind_group_layout, bind_group)
}
