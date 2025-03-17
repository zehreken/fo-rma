use rand::Fill;
use wgpu::{Device, Queue, Surface, SurfaceError};
use winit::window::Window;

use crate::{
    basics::level::Level,
    rendering::{fill_renderer::FillRenderer, screen_renderer::ScreenRenderer},
    rendering_utils::{self},
};

pub struct Renderer<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
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

        let fill_renderer = FillRenderer::new(&device, &surface_config, size);
        let screen_renderer = ScreenRenderer::new(&device, &queue, &surface_config);

        Self {
            surface,
            device,
            queue,
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

        let output_view = output_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.fill_renderer
            .render(&self.device, &self.queue, &output_view, 0.0);
        // self.screen_renderer
        //     .render(&self.device, &self.queue, &output_view);

        output_frame.present();
        Ok(())
    }
}
