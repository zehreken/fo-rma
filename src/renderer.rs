use crate::{
    app::Settings,
    audio::sequencer::Sequencer,
    basics::scene3::Scene,
    gui::Gui,
    material::post_process_material::PostProcessMaterial,
    rendering::{
        fill_renderer::FillRenderer, line_renderer::LineRenderer, post_processor::PostProcessor,
        screen_renderer::ScreenRenderer,
    },
    rendering_utils::{self},
};
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, SurfaceError, TextureView};
use winit::{dpi::PhysicalSize, window::Window};

pub const PRIMITIVE_COUNT: u64 = 31;

pub struct Renderer<'a> {
    pub surface: Surface<'a>,
    pub device: Device,
    pub surface_config: SurfaceConfiguration,
    pub queue: Queue,
    pub gui: Gui,
    depth_texture: TextureView,
    pub render_texture_material: PostProcessMaterial,
    fill_renderer: FillRenderer,
    line_renderer: LineRenderer,
    post_processor: PostProcessor,
    screen_renderer: ScreenRenderer,
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

        let render_texture_material = PostProcessMaterial::new(&device, &surface_config, size);

        let fill_renderer = FillRenderer::new();
        let line_renderer = LineRenderer::new(&device, &surface_config);
        let post_processor = PostProcessor::new(
            &device,
            &render_texture_material.post_process_texture_view,
            &render_texture_material.render_texture_view,
        );
        let screen_renderer = ScreenRenderer::new(&device);

        Self {
            surface,
            device,
            surface_config,
            queue,
            gui,
            depth_texture,
            render_texture_material,
            fill_renderer,
            line_renderer,
            post_processor,
            screen_renderer,
            size,
        }
    }

    pub fn render(
        &mut self,
        window: &Window,
        scene: &Scene,
        sequencers: &mut Vec<Sequencer>,
        fps: f32,
        settings: &Settings,
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
            &self.render_texture_material.render_texture_view,
            scene,
        );

        // self.line_renderer.render(
        //     &self.device,
        //     &self.queue,
        //     &self.depth_texture,
        //     &self.render_texture_material.render_texture_view,
        //     scene,
        // );

        self.post_processor
            .run(&self.device, &self.queue, self.size.width, self.size.height);

        if settings.draw_ui {
            self.gui.render(
                window,
                &self.render_texture_material.post_process_texture_view,
                &self.device,
                &self.queue,
                sequencers,
                fps,
            );
        }

        let output_view = output_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.screen_renderer.render(
            &self.device,
            &self.queue,
            &output_view,
            &self.render_texture_material,
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
        self.render_texture_material
            .resize(&self.device, &self.surface_config, size);
        self.post_processor.resize(
            &self.device,
            &self.render_texture_material.post_process_texture_view,
            &self.render_texture_material.render_texture_view,
        );
        self.gui.resize(size, scale_factor);
    }
}
