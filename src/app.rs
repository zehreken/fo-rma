use std::{collections::VecDeque, time};

use winit::{
    dpi::{PhysicalSize, Size},
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

use crate::{
    basics::{cube, quad, triangle},
    renderer,
};

pub struct App<'a> {
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: &'a Window, // this stays here but above goes to renderer
    renderer: renderer::Renderer<'a>,
    quad: quad::State,
    cube: cube::State,
    triangle: triangle::State,
}

impl<'a> App<'a> {
    async fn new(window: &'a Window) -> App<'a> {
        let size = window.inner_size();
        let renderer = renderer::Renderer::new(window).await;
        // let init = vec![0.0; 60];

        let quad = quad::State::new(&renderer.device);
        let cube = cube::State::new(&renderer.device);
        let triangle = triangle::State::new(&renderer.device);
        Self {
            size,
            window: &window,
            renderer,
            quad,
            cube,
            triangle,
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.size = size;
            self.renderer.resize(size, self.window.scale_factor());
            self.quad.resize(size);
            self.cube.resize(size);
            self.triangle.resize(size);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        // self.camera.update();
    }
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
    let mut elapsed: f32 = 0.0;

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
            let delta_time = std::time::Instant::now()
                .duration_since(earlier)
                .as_secs_f32();
            elapsed += delta_time;
            earlier = std::time::Instant::now();
            rolling_frame_times.pop_front();
            rolling_frame_times.push_back(delta_time);
            let fps = calculate_fps(&rolling_frame_times);
            app.quad.update();
            app.triangle.update(delta_time);
            let _ = app.renderer.render(
                &app.window,
                &app.quad,
                &app.cube,
                &app.triangle,
                elapsed,
                delta_time,
                fps,
            );
            app.window.request_redraw();
        }
        Event::WindowEvent { event, .. } => {
            app.renderer.gui.handle_event(&app.window, &event);
        }
        _ => {}
    });
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

pub fn calculate_fps(times: &VecDeque<f32>) -> f32 {
    let sum: f32 = times.iter().sum();

    let average_time = sum / times.len() as f32;
    return 1.0 / average_time;
}
