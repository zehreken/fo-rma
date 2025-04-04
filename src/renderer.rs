use crate::{
    audio::sequencer::Sequencer,
    basics::{core::GenericUniformData, scene::Scene},
    gui::Gui,
    rendering::{
        fill_renderer::FillRenderer, line_renderer::LineRenderer, post_processor::PostProcessor,
        screen_renderer::ScreenRenderer,
    },
    rendering_utils::{self},
};
use wgpu::{
    BindGroup, BindGroupLayout, Device, Queue, Surface, SurfaceConfiguration, SurfaceError,
    Texture, TextureFormat, TextureView,
};
use winit::{dpi::PhysicalSize, window::Window};
pub const PRIMITIVE_COUNT: u64 = 27;

pub struct Renderer<'a> {
    pub surface: Surface<'a>,
    pub device: Device,
    pub surface_config: SurfaceConfiguration,
    pub queue: Queue,
    pub gui: Gui,
    depth_texture: TextureView,
    pub render_texture: (Texture, TextureView),
    render_texture_bind_group: (BindGroupLayout, BindGroup),
    pub post_process_texture: (Texture, TextureView),
    fill_renderer: FillRenderer,
    line_renderer: LineRenderer,
    post_processor: PostProcessor,
    screen_renderer: ScreenRenderer,
    pub generic_uniform_data: GenericUniformData,
    pub light_uniform_data: GenericUniformData,
    texture_format: TextureFormat,
    size: PhysicalSize<u32>,
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

        let gui = Gui::new(window, &device, texture_format);

        let depth_texture = rendering_utils::create_depth_texture(&device, &surface_config);

        let render_texture = rendering_utils::create_render_texture(&device, &texture_format, size);
        let post_process_texture = rendering_utils::create_post_process_texture(&device, size);
        // Bind it to the post_processed texture, since that is the one we want to show
        let render_texture_bind_group =
            rendering_utils::create_render_texture_bind_group(&device, &post_process_texture.1);

        let fill_renderer = FillRenderer::new();
        let line_renderer = LineRenderer::new(&device, &surface_config);
        let post_processor =
            PostProcessor::new(&device, &post_process_texture.1, &render_texture.1);
        let screen_renderer = ScreenRenderer::new(
            &device,
            &queue,
            &surface_config,
            &render_texture_bind_group.0,
        );

        let generic_uniform_data =
            rendering_utils::create_generic_uniform_data(&device, &surface_config, PRIMITIVE_COUNT);
        let light_uniform_data = rendering_utils::create_light_uniform_data(&device);

        Self {
            surface,
            device,
            surface_config,
            queue,
            gui,
            depth_texture,
            render_texture,
            render_texture_bind_group,
            post_process_texture,
            fill_renderer,
            line_renderer,
            post_processor,
            screen_renderer,
            generic_uniform_data,
            light_uniform_data,
            texture_format,
            size,
        }
    }

    pub fn render(
        &mut self,
        window: &Window,
        level: &Scene,
        sequencer: &mut Sequencer,
        fps: f32,
    ) -> Result<(), SurfaceError> {
        let output_frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(SurfaceError::Outdated) => return Ok(()),
            Err(e) => return Err(e),
        };

        self.fill_renderer.render(
            &self.device,
            &self.queue,
            &self.depth_texture,
            &self.render_texture.1,
            level,
            &self.generic_uniform_data,
            &self.light_uniform_data,
        );

        // self.line_renderer.render(
        //     &self.device,
        //     &self.queue,
        //     &self.depth_texture,
        //     &self.render_texture.1,
        //     level,
        // );

        self.post_processor
            .run(&self.device, &self.queue, self.size.width, self.size.height);

        self.gui.render(
            window,
            &self.post_process_texture.1,
            &self.device,
            &self.queue,
            sequencer,
            fps,
        );

        let output_view = output_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.screen_renderer.render(
            &self.device,
            &self.queue,
            &output_view,
            &self.render_texture_bind_group.1,
        );
        output_frame.present();

        Ok(())
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>, scale_factor: f64) {
        self.size = size;
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;
        self.surface.configure(&self.device, &self.surface_config);
        self.depth_texture =
            rendering_utils::create_depth_texture(&self.device, &self.surface_config);
        self.render_texture =
            rendering_utils::create_render_texture(&self.device, &self.texture_format, size);
        self.post_process_texture =
            rendering_utils::create_post_process_texture(&self.device, size);
        self.render_texture_bind_group = rendering_utils::create_render_texture_bind_group(
            &self.device,
            &self.post_process_texture.1,
        );
        self.post_processor.resize(
            &self.device,
            &self.post_process_texture.1,
            &self.render_texture.1,
        );
        self.gui.resize(size, scale_factor);
    }
}
