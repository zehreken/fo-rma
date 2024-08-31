use std::collections::VecDeque;

use winit::{
    dpi::{PhysicalSize, Size},
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

use crate::{basics::cube, gui, renderer};

pub struct App {
    rolling_frame_time: VecDeque<f32>,
}

impl App {
    async fn new() -> App {
        let init = vec![0.0; 60];
        Self {
            rolling_frame_time: VecDeque::from(init),
        }
    }
}

pub async fn start() {
    let size = Size::Physical(PhysicalSize {
        width: 1600,
        height: 1200,
    });
    let event_loop = EventLoop::new().unwrap();
    let window = create_window(size, &event_loop);
    let app = App::new().await;
    let (instance, surface) = create_instance_and_surface(&window);
    // Async is fine but you can also use pollster::block_on without await
    let adapter = create_adapter(instance, &surface).await;
    // Same with this one, pollster::block_on(adapter_request(...)).unwrap(); is another way
    let (device, queue) = create_device_and_queue(&adapter).await;

    let size = window.inner_size();
    let surface_caps = surface.get_capabilities(&adapter);
    let texture_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    let surface_config = create_surface_config(texture_format, size, surface_caps);
    surface.configure(&device, &surface_config);
    // create renderer
    // let mut renderer = renderer::Renderer::new(&device, &surface_config);
    let cube_renderer = cube::State::new(&device, &surface_config);
    // create gui
    let gui = gui::Gui::new(&window, &device, texture_format);

    run_event_loop(
        event_loop,
        window,
        surface,
        cube_renderer,
        queue,
        device,
        gui,
    );
}

fn run_event_loop(
    event_loop: EventLoop<()>,
    window: winit::window::Window,
    surface: wgpu::Surface<'_>,
    mut cube_renderer: cube::State,
    queue: wgpu::Queue,
    device: wgpu::Device,
    mut gui: gui::Gui,
) {
    let init = [0.0; 60];
    let mut rolling_frame_times = VecDeque::from(init);
    let mut earlier = std::time::Instant::now();
    let mut elapsed_time = 0.0;

    let r = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window.id() == window_id => elwt.exit(),
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
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            let frame_time = std::time::Instant::now().duration_since(earlier);
            elapsed_time += frame_time.as_secs_f32();
            earlier = std::time::Instant::now();
            rolling_frame_times.pop_front();
            rolling_frame_times.push_back(frame_time.as_secs_f32());
            let fps = calculate_fps(&rolling_frame_times);
            let output_frame = match surface.get_current_texture() {
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

            // renderer.render(&device, &queue, &output_view, elapsed_time);
            cube_renderer.update(&queue);
            cube_renderer.render(&device, &queue, &output_view);
            gui.render(&window, &output_view, &device, &queue, fps);
            output_frame.present();
            window.request_redraw();
        }
        Event::WindowEvent { event, .. } => {
            gui.handle_event(&window, &event);
        }
        _ => {}
    });
}

fn create_surface_config(
    texture_format: wgpu::TextureFormat,
    size: PhysicalSize<u32>,
    surface_caps: wgpu::SurfaceCapabilities,
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
        .with_resizable(false)
        .with_transparent(false)
        .with_title("winit-wgpu-egui")
        .with_inner_size(size)
        .build(event_loop)
        .unwrap();
    window
}

pub fn calculate_fps(times: &VecDeque<f32>) -> f32 {
    let sum: f32 = times.iter().sum();

    let average_time = sum / times.len() as f32;
    return 1.0 / average_time;
}
