use std::collections::VecDeque;

use glam::{vec3, Vec3};
use wgpu::{util::DeviceExt, Device, SurfaceCapabilities, SurfaceError, TextureFormat};
use winit::{
    dpi::{PhysicalSize, Size},
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

use crate::{
    basics::{
        camera::{Camera, CameraUniform},
        cube, triangle,
    },
    gui, renderer,
};

pub struct App<'a> {
    // surface: wgpu::Surface<'a>,
    // device: wgpu::Device,
    // queue: wgpu::Queue,
    // config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: &'a Window, // this stays here but above goes to renderer
    renderer: renderer::Renderer<'a>,
    // gui: gui::Gui,
    // camera: Camera,
    // cube: cube::State,
    triangle: triangle::State,
}

impl<'a> App<'a> {
    async fn new(window: &'a Window) -> App<'a> {
        let size = window.inner_size();
        // let (instance, surface) = create_instance_and_surface(window);
        // let adapter = create_adapter(instance, &surface).await;
        // let (device, queue) = create_device_and_queue(&adapter).await;

        // let surface_caps = surface.get_capabilities(&adapter);
        // let texture_format = surface_caps
        //     .formats
        //     .iter()
        //     .copied()
        //     .find(|f| f.is_srgb())
        //     .unwrap_or(surface_caps.formats[0]);
        // let surface_config = create_surface_config(size, texture_format, surface_caps);
        // surface.configure(&device, &surface_config);
        let renderer = renderer::Renderer::new(window).await;
        let init = vec![0.0; 60];
        // let gui = gui::Gui::new(window, &device, texture_format);
        // let camera = create_camera(size, &device);
        // let cube = cube::State::new(&device, &surface_config);
        let triangle = triangle::State::new(&renderer.device);
        Self {
            // surface,
            // device,
            // queue,
            // config: surface_config,
            size,
            window: &window,
            renderer,
            // gui,
            // camera,
            // cube,
            triangle,
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.size = size;
            // self.config.width = size.width;
            // self.config.height = size.height;
            // self.surface.configure(&self.device, &self.config);
            // self.cube.resize(size);
            // self.triangle.resize(size);
            // self.gui.resize(size, self.window.scale_factor());
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    // fn update(&mut self) {
    //     self.cube.update(&self.queue);
    // }

    fn render(&mut self, fps: f32) -> Result<(), SurfaceError> {
        /*
        let output_frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Outdated) => {
                // This error occurs when the app is minimized on Windows.
                // Silently return here to prevent spamming the console with:
                // "The underlying surface has changed, and therefore the swap chain must be updated"
                eprintln!("Outdated");
                return Ok(());
            }
            Err(e) => {
                eprintln!("Dropped frame with error: {}", e);
                return Err(e);
            }
        };
        let output_view = output_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("renderer_encoder"),
            });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.03,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        // cube
        self.cube.update(&self.queue);
        self.cube.render(&self.queue, &mut render_pass);
        // triangle
        self.triangle.render(&self.queue, &mut render_pass);

        drop(render_pass);
        self.gui.render(
            &self.window,
            &output_view,
            &self.device,
            &self.queue,
            &mut encoder,
            fps,
        );
        self.queue.submit(Some(encoder.finish()));
        output_frame.present();
        self.window.request_redraw();
        Ok(())
        */
        Ok(())
    }

    fn render_ui(&mut self) {}
}

pub async fn start() {
    let size = Size::Physical(PhysicalSize {
        width: 1600,
        height: 1200,
    });
    let event_loop = EventLoop::new().unwrap();
    let window = create_window(size, &event_loop);
    let app = App::new(&window).await;

    run_event_loop(event_loop, app);
}

fn run_event_loop(event_loop: EventLoop<()>, mut app: App) {
    let init = [0.0; 60];
    let mut rolling_frame_times = VecDeque::from(init);
    let mut earlier = std::time::Instant::now();

    let r = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if app.window.id() == window_id => elwt.exit(),
        Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            state: ElementState::Pressed,
                            repeat: false,
                            ..
                        },
                    ..
                },
            ..
        } => elwt.exit(),
        Event::WindowEvent {
            window_id,
            event: WindowEvent::Resized(size),
        } if app.window.id() == window_id => app.resize(size),
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            let frame_time = std::time::Instant::now().duration_since(earlier);
            earlier = std::time::Instant::now();
            rolling_frame_times.pop_front();
            rolling_frame_times.push_back(frame_time.as_secs_f32());
            let fps = calculate_fps(&rolling_frame_times);
            // app.update();
            // let _ = app.render(fps);
            let _ = app.renderer.render(&app.triangle);
        }
        Event::WindowEvent { event, .. } => {
            // app.gui.handle_event(&app.window, &event);
        }
        _ => {}
    });
}

fn create_surface_config(
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

async fn create_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::default(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .unwrap();
    (device, queue)
}

async fn create_adapter(instance: wgpu::Instance, surface: &wgpu::Surface<'_>) -> wgpu::Adapter {
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

fn create_instance_and_surface(
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

fn create_window(size: Size, event_loop: &EventLoop<()>) -> winit::window::Window {
    let window = WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("winit-wgpu-egui")
        .with_inner_size(size)
        .build(event_loop)
        .unwrap();
    window
}

fn create_camera(size: PhysicalSize<u32>, device: &Device) -> Camera {
    let camera = Camera::new(
        device,
        vec3(0.0, 1.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        size.width as f32 / size.height as f32,
        45.0,
        0.1,
        100.0,
    );

    camera
}

pub fn calculate_fps(times: &VecDeque<f32>) -> f32 {
    let sum: f32 = times.iter().sum();

    let average_time = sum / times.len() as f32;
    return 1.0 / average_time;
}
