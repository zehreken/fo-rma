use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use ringbuf::{HeapConsumer, HeapRb};
use winit::{
    dpi::{PhysicalSize, Size},
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

use crate::{
    audio::audio_model::AudioModel,
    basics::{cube::Cube, primitive::Primitive, quad::Quad, triangle::Triangle},
    renderer,
};

pub struct App<'a> {
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: &'a Window, // this stays here but above goes to renderer
    renderer: renderer::Renderer<'a>,
    primitives: Vec<Box<dyn Primitive>>,
    audio_model: AudioModel,
    view_consumer: HeapConsumer<f32>,
}

impl<'a> App<'a> {
    async fn new(window: &'a Window) -> App<'a> {
        let size = window.inner_size();
        let renderer = renderer::Renderer::new(window).await;
        // let init = vec![0.0; 60];

        let primitives: Vec<Box<dyn Primitive>> = vec![
            // Box::new(Triangle::new(&renderer.device)),
            // Box::new(Quad::new(&renderer.device)),
            Box::new(Cube::new(&renderer.device)),
        ];

        let view_ring = HeapRb::new(100000);
        let (view_producer, view_consumer) = view_ring.split();
        let audio_model = AudioModel::new(view_producer).unwrap();

        Self {
            size,
            window: &window,
            renderer,
            primitives,
            audio_model,
            view_consumer,
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.size = size;
            self.renderer.resize(size, self.window.scale_factor());
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {}
}

pub async fn start() {
    let size = Size::Physical(PhysicalSize {
        width: 1080,
        height: 1080,
    });
    let event_loop = EventLoop::new().unwrap();
    let window = create_window(size, &event_loop);
    let app = App::new(&window).await;

    run_event_loop(event_loop, app);
}

fn run_event_loop(event_loop: EventLoop<()>, mut app: App) {
    let init = [0.0; 60];
    let mut rolling_frame_times = VecDeque::from(init);
    let mut earlier = Instant::now();
    let mut elapsed: f32 = 0.0;

    const TARGET_FPS: u64 = 60;
    const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / TARGET_FPS);
    let mut last_frame_time = Instant::now();

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
            for primitive in &mut app.primitives {
                primitive.update(delta_time);
            }
            let signal = (app.view_consumer.pop().unwrap_or(0.0) + 1.0) / 2.0;
            // app.view_consumer.clear();
            let _ = app.renderer.render(
                &app.window,
                &app.primitives,
                elapsed,
                delta_time,
                fps,
                signal,
            );

            let frame_duration = last_frame_time.elapsed();
            if frame_duration < FRAME_TIME {
                std::thread::sleep(FRAME_TIME - frame_duration);
            }
            last_frame_time = Instant::now();

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
