use crate::{
    rendering::{fill_renderer::FillRenderer, screen_renderer::ScreenRenderer},
    rendering_utils::{self},
};
use wgpu::{
    BindGroup, BindGroupLayout, Device, Extent3d, Queue, Sampler, Surface, SurfaceError, Texture,
    TextureView,
};
use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    offscreen_texture: TextureStuff,
    fill_renderer: FillRenderer,
    screen_renderer: ScreenRenderer,
    // generic_uniform_data: GenericUniformData,
    // screen_quad: Box<dyn Primitive>,
}

impl<'a> Renderer<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let size = window.inner_size();
        let (instance, surface) = rendering_utils::create_instance_and_surface(window);
        let adapter = rendering_utils::create_adapter(instance, &surface).await;
        let (device, queue) = rendering_utils::create_device_and_queue(&adapter).await;
        let surface_caps = surface.get_capabilities(&adapter);
        let texture_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let surface_config =
            rendering_utils::create_surface_config(size, texture_format, surface_caps);
        surface.configure(&device, &surface_config);

        let offscreen_texture = create_test_texture(&device, &queue, size);

        let fill_renderer = FillRenderer::new(&device, &surface_config, size);
        let screen_renderer = ScreenRenderer::new(
            &device,
            &queue,
            &surface_config,
            &offscreen_texture.bind_group_layout,
        );

        Self {
            surface,
            device,
            queue,
            offscreen_texture,
            fill_renderer,
            screen_renderer,
        }
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output_frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(SurfaceError::Outdated) => return Ok(()),
            Err(e) => return Err(e),
        };

        self.fill_renderer.render(
            &self.device,
            &self.queue,
            &self.offscreen_texture.texture_view,
            0.0,
        );
        // post_processor::test(&output_view);

        let output_view = output_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.screen_renderer.render(
            &self.device,
            &self.queue,
            &output_view,
            &self.offscreen_texture,
        );

        output_frame.present();
        Ok(())
    }
}

fn create_test_texture(device: &Device, queue: &Queue, size: PhysicalSize<u32>) -> TextureStuff {
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
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
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
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    });

    TextureStuff {
        texture,
        size,
        texture_view,
        sampler,
        bind_group_layout,
        bind_group,
    }
}

pub struct TextureStuff {
    pub texture: Texture,
    pub size: Extent3d,
    pub texture_view: TextureView,
    pub sampler: Sampler,
    pub bind_group_layout: BindGroupLayout,
    pub bind_group: BindGroup,
}
