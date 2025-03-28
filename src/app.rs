use crate::{audio::audio_model::AudioModel, basics::level::Level, renderer, save_image};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
use winit::{
    dpi::{PhysicalSize, Size},
    error::EventLoopError,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    keyboard::{KeyCode, ModifiersState, PhysicalKey},
    window::{Window, WindowBuilder},
};

const TARGET_FPS: u64 = 60;
const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / TARGET_FPS);

pub struct App<'a> {
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: &'a Window, // this stays here but above goes to renderer
    renderer: renderer::Renderer<'a>,
    pub level: Level,
    audio_model: AudioModel,
    signal_peak: f32,
    rolling_frame_times: VecDeque<f32>,
    earlier: Instant,
    elapsed: f32,
}

impl<'a> App<'a> {
    async fn new(window: &'a Window) -> App<'a> {
        let size = window.inner_size();
        // let renderer = renderer::Renderer::new(window).await;
        let renderer = renderer::Renderer::new(window).await;
        // let init = vec![0.0; 60];

        let level = Level::new(
            &renderer.device,
            &renderer.surface_config,
            &renderer.generic_uniform_data,
            &renderer.light_uniform_data,
            size,
        );

        let audio_model = AudioModel::new().unwrap();

        Self {
            size,
            window: &window,
            renderer,
            level,
            audio_model,
            signal_peak: 0.0,
            rolling_frame_times: VecDeque::from([0.0; 60]),
            earlier: Instant::now(),
            elapsed: 0.0,
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.size = size;
            self.renderer.resize(size, self.window.scale_factor());
            self.level.camera.resize(size);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        let delta_time = std::time::Instant::now()
            .duration_since(self.earlier)
            .as_secs_f32();
        self.elapsed += delta_time;
        self.earlier = std::time::Instant::now();
        self.rolling_frame_times.pop_front();
        self.rolling_frame_times.push_back(delta_time);
        let fps = calculate_fps(&self.rolling_frame_times);
        let signal = self.audio_model.get_signal();
        if signal > self.signal_peak {
            self.signal_peak = signal;
        }
        self.level
            .update(delta_time, 1.0, self.audio_model.show_beat());
        let _ = self.renderer.render(
            self.window,
            &self.level,
            &mut self.audio_model.get_sequencers()[0],
            fps,
        );
        self.signal_peak = (self.signal_peak - 0.05).max(0.0);

        self.audio_model.update();

        #[cfg(not(target_os = "macos"))]
        {
            let frame_duration = last_frame_time.elapsed();
            if frame_duration < FRAME_TIME {
                std::thread::sleep(FRAME_TIME - frame_duration);
            }
            last_frame_time = Instant::now();
        }

        self.window.request_redraw();
    }
}

pub async fn start() {
    let size = Size::Physical(PhysicalSize {
        width: 1080,
        height: 1080,
    });
    let event_loop = EventLoop::new().unwrap();
    let window = create_window(size, &event_loop);
    let app = App::new(&window).await;

    let r = run_event_loop(event_loop, app);
}

fn run_event_loop(event_loop: EventLoop<()>, mut app: App) -> Result<(), EventLoopError> {
    let mut modifiers = winit::keyboard::ModifiersState::empty();

    event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if app.window.id() == window_id => elwt.exit(),
        Event::WindowEvent {
            window_id: _,
            event: WindowEvent::ModifiersChanged(new_modifiers),
        } => modifiers = new_modifiers.state(),
        Event::WindowEvent {
            event: WindowEvent::KeyboardInput {
                event: key_event, ..
            },
            ..
        } => handle_key_event(&modifiers, &key_event, elwt, &mut app),
        Event::WindowEvent {
            window_id,
            event: WindowEvent::Resized(size),
        } if app.window.id() == window_id => app.resize(size),
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            app.update();
        }
        Event::WindowEvent { event, .. } => {
            app.renderer.gui.handle_event(&app.window, &event);
        }
        _ => {}
    })
}

fn create_window(size: Size, event_loop: &EventLoop<()>) -> winit::window::Window {
    let window = WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("fōrma")
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

fn handle_key_event(
    modifiers: &ModifiersState,
    key_event: &KeyEvent,
    elwt: &EventLoopWindowTarget<()>,
    app: &mut App,
) {
    if modifiers.shift_key() {
        match key_event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) => app.level.camera.orbit_z(true),
            PhysicalKey::Code(KeyCode::KeyA) => app.level.camera.orbit_x(false),
            PhysicalKey::Code(KeyCode::KeyS) => app.level.camera.orbit_z(false),
            PhysicalKey::Code(KeyCode::KeyD) => app.level.camera.orbit_x(true),
            PhysicalKey::Code(KeyCode::KeyQ) => app.level.camera.orbit_y(true),
            PhysicalKey::Code(KeyCode::KeyE) => app.level.camera.orbit_y(false),
            _ => {}
        }
    } else {
        match key_event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    elwt.exit();
                }
            }
            PhysicalKey::Code(KeyCode::KeyR) => {
                if key_event.state == ElementState::Pressed && !key_event.repeat {
                    save_image::save_image(
                        &app.renderer.device,
                        &app.renderer.queue,
                        &app.renderer.surface_config,
                        &app.renderer.post_process_texture,
                    );
                }
            }
            PhysicalKey::Code(KeyCode::KeyW) => app.level.camera.move_z(true),
            PhysicalKey::Code(KeyCode::KeyA) => app.level.camera.move_x(false),
            PhysicalKey::Code(KeyCode::KeyS) => app.level.camera.move_z(false),
            PhysicalKey::Code(KeyCode::KeyD) => app.level.camera.move_x(true),
            PhysicalKey::Code(KeyCode::KeyQ) => app.level.camera.move_y(true),
            PhysicalKey::Code(KeyCode::KeyE) => app.level.camera.move_y(false),
            _ => {}
        }
    }
}
