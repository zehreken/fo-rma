use crate::material::Material;
use wgpu::{
    BindGroup, BindGroupLayout, Device, Extent3d, ShaderModule, SurfaceCapabilities, Texture,
    TextureFormat, TextureView,
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

pub fn create_shader_module(device: &Device, material_type: Material) -> ShaderModule {
    let (name, shader_main) = match material_type {
        Material::Debug => ("debug", include_str!("shaders/debug.wgsl")),
        Material::DiffuseColor => ("diffuse_color", include_str!("shaders/diffuse_color.wgsl")),
        Material::Equalizer => ("equalizer", include_str!("shaders/equalizer.wgsl")),
        Material::UnlitColor => ("unlit_color", include_str!("shaders/unlit_color.wgsl")),
        Material::Wave => ("wave", include_str!("shaders/wave.wgsl")),
        Material::Texture => ("texture", include_str!("shaders/texture.wgsl")),
        Material::DiffuseTexture => todo!(),
    };

    let shader_utils = include_str!("shaders/utils.wgsl");
    let shader_combined = format!("{}\n{}", shader_main, shader_utils);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(name),
        source: wgpu::ShaderSource::Wgsl(shader_combined.into()),
    });

    shader
}

pub fn create_texture(
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
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    (texture, texture_view)
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
    let size = Extent3d {
        width: 512,
        height: 1,
        depth_or_array_layers: 1,
    };
    let post_process_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("wave_texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D1,
        format: wgpu::TextureFormat::R32Float,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let post_process_view = post_process_texture.create_view(&Default::default());

    (post_process_texture, post_process_view)
}

pub fn create_texture_bind_group(
    device: &Device,
    post_process_texture_view: &TextureView,
) -> (BindGroupLayout, BindGroup) {
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        address_mode_w: wgpu::AddressMode::Repeat,
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
                resource: wgpu::BindingResource::TextureView(&post_process_texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    });

    (bind_group_layout, bind_group)
}

pub fn create_wave_texture_bind_group(
    device: &Device,
    wave_texture: &TextureView,
) -> (BindGroupLayout, BindGroup) {
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
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
                    view_dimension: wgpu::TextureViewDimension::D1,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                // This should match the filterable field of the
                // corresponding Texture entry above.
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("wave_texture_bind_group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&wave_texture),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    });

    (bind_group_layout, bind_group)
}
