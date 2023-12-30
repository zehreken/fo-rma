use crate::{back::Back, gui::AppUi};
use wgpu::{CommandEncoder, Device, Queue, Surface, SurfaceConfiguration, TextureFormat};
use winit::{
    dpi::{PhysicalSize, Size},
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct App {
    window: Window,
    surface: Surface,
    config: SurfaceConfiguration,
    pub device: Device,
    pub queue: Queue,
    surface_format: TextureFormat,
}

impl App {
    async fn new(window: Window) -> App {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // App owns the window so this should be safe
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // This is async but you can also use pollster::block_on without await
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(), // Can be HighPerformance
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ))
        .unwrap();

        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width as u32,
            height: size.height as u32,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);
        App {
            window,
            surface,
            config: surface_config,
            device,
            queue,
            surface_format,
        }
    }

    fn window(&self) -> &Window {
        &self.window
    }

    fn render(&self) {}

    fn render_ui(&self) {}
}

pub async fn start() {
    let size = Size::Physical(PhysicalSize {
        width: 1600,
        height: 1200,
    });
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(false)
        .with_transparent(false)
        .with_title("fo-rma")
        .with_inner_size(size)
        .build(&event_loop)
        .unwrap();

    let app = App::new(window).await;
    let back = Back::new(&app.device, &app.config);
    let mut app_ui = AppUi::new(&event_loop, 1600, 1200, 2., &app.device, app.surface_format);

    event_loop.run(move |event, _elwt, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == app.window().id() => control_flow.set_exit(),
        Event::WindowEvent { event, .. } => {
            app_ui.handle_event(&event);
        }
        Event::MainEventsCleared => app.window().request_redraw(),
        Event::RedrawRequested(_) => {
            let output_frame = match app.surface.get_current_texture() {
                Ok(frame) => frame,
                Err(wgpu::SurfaceError::Outdated) => {
                    // This error occurs when the app is minimized on Windows.
                    // Silently return here to prevent spamming the console with:
                    // "The underlying surface has changed, and therefore the swap chain must be updated"
                    return;
                }
                Err(e) => {
                    eprintln!("Dropped frame with error: {}", e);
                    return;
                }
            };
            let output_view = output_frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = app
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("encoder"),
                });
            back.render(&output_view, &app.device, &app.queue);
            app_ui.prepare(&app.window);
            app_ui.render(&mut encoder, &output_view, &app);
            app.queue.submit(Some(encoder.finish()));
            output_frame.present();
        }
        _ => {}
    });
}
