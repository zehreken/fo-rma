use crate::{
    audio::audio_model::AudioModel,
    basics::{scene3::Scene, scene_constructor},
    renderer, save_image,
};
use std::{
    collections::VecDeque,
    sync::Arc,
    time::{Duration, Instant},
};
use winit::{
    dpi::{PhysicalSize, Size},
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

const TARGET_FPS: u64 = 60;
const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / TARGET_FPS);

pub struct App<'a> {
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: &'a Window, // this stays here but above goes to renderer
    renderer: renderer::Renderer<'a>,
    pub scene: Scene,
    audio_model: AudioModel,
    rolling_frame_times: VecDeque<f32>,
    earlier: Instant,
    elapsed: f32,
    last_frame_time: Instant,
    settings: Settings,
}

pub struct Settings {
    pub draw_ui: bool,
}

impl Settings {
    pub fn new() -> Self {
        Settings { draw_ui: true }
    }
}

impl<'a> App<'a> {
    async fn new(window: &'a Window) -> App<'a> {
        let size = window.inner_size();
        let renderer = renderer::Renderer::new(window).await;

        let json = include_str!("../scenes/scene_01.json");
        let pseudo_scene = scene_constructor::construct_scene_from_json(json);
        dbg!(pseudo_scene);

        let scene = Scene::new(&renderer.device, &renderer.surface_config, size);

        let audio_model = AudioModel::new().unwrap();

        Self {
            size,
            window: &window,
            renderer,
            scene,
            audio_model,
            rolling_frame_times: VecDeque::from([0.0; 60]),
            earlier: Instant::now(),
            elapsed: 0.0,
            last_frame_time: Instant::now(),
            settings: Settings::new(),
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.size = size;
            self.renderer.resize(size, self.window.scale_factor());
            self.scene.camera.resize(size);
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
        let mut signal_peak = self.audio_model.get_signal();

        let rolling_wave: Vec<f32> = self.audio_model.rolling_wave.iter().map(|i| *i).collect();
        signal_peak = (signal_peak - 0.05).max(0.0);
        self.scene.update(
            &self.renderer.queue,
            delta_time,
            signal_peak,
            self.audio_model.show_beat(),
            Arc::new(rolling_wave),
        );
        let _ = self.renderer.render(
            self.window,
            &self.scene,
            &mut self.audio_model.get_sequencers()[0],
            fps,
            &self.settings,
        );

        self.audio_model.update();

        #[cfg(not(target_os = "macos"))]
        {
            let frame_duration = self.last_frame_time.elapsed();
            if frame_duration < FRAME_TIME {
                std::thread::sleep(FRAME_TIME - frame_duration);
            }
            self.last_frame_time = Instant::now();
        }

        self.scene.camera.update();

        self.window.request_redraw();
    }
}

pub async fn start() {
    let size = Size::Physical(PhysicalSize {
        width: 1080,
        height: 1080,
    });
    let input = WinitInputHelper::new();
    let event_loop = EventLoop::new().unwrap();
    let window = create_window(size, &event_loop);
    let app = App::new(&window).await;

    // let r = run_event_loop(event_loop, app);
    let r = run_event_loop(event_loop, app, input);
}

fn run_event_loop(
    event_loop: EventLoop<()>,
    mut app: App,
    mut input: WinitInputHelper,
) -> Result<(), EventLoopError> {
    event_loop.run(move |event, elwt| {
        if input.update(&event) {
            if input.key_released(KeyCode::Escape) || input.close_requested() || input.destroyed() {
                elwt.exit();
                return;
            }
            if input.key_held(KeyCode::KeyW) {
                app.scene.camera.move_z(false);
            }
            if input.key_held(KeyCode::KeyA) {
                app.scene.camera.move_x(false);
            }
            if input.key_held(KeyCode::KeyS) {
                app.scene.camera.move_z(true);
            }
            if input.key_held(KeyCode::KeyD) {
                app.scene.camera.move_x(true);
            }
            if input.key_held(KeyCode::KeyQ) {
                app.scene.camera.move_y(true);
            }
            if input.key_held(KeyCode::KeyE) {
                app.scene.camera.move_y(false);
            }
            if input.held_shift() {
                if input.key_held(KeyCode::KeyW) {
                    app.scene.camera.orbit_z(false);
                }
                if input.key_held(KeyCode::KeyA) {
                    app.scene.camera.orbit_x(false);
                }
                if input.key_held(KeyCode::KeyS) {
                    app.scene.camera.orbit_z(true);
                }
                if input.key_held(KeyCode::KeyD) {
                    app.scene.camera.orbit_x(true);
                }
                if input.key_held(KeyCode::KeyQ) {
                    app.scene.camera.orbit_y(true);
                }
                if input.key_held(KeyCode::KeyE) {
                    app.scene.camera.orbit_y(false);
                }
            }
            if input.key_pressed(KeyCode::KeyU) {
                app.settings.draw_ui = !app.settings.draw_ui;
            }
            if input.key_pressed(KeyCode::KeyR) {
                save_image::save_image(
                    &app.renderer.device,
                    &app.renderer.queue,
                    &app.renderer.surface_config,
                    &app.renderer.render_texture_material.post_process_texture,
                );
            }
            app.scene
                .camera
                .rotate(input.mouse_diff().0, input.mouse_diff().1);
        }

        match event {
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
        }
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
