use miniquad::Texture;
use {egui_miniquad as egui_mq, miniquad as mq};

struct Stage {
    egui_mq: egui_mq::EguiMq,
    texture: Texture,
}

impl Stage {
    fn new(ctx: &mut mq::Context) -> Self {
        const WIDTH: usize = 600;
        const HEIGHT: usize = 600;
        let mut scene = super::cpu_ray_tracer::create_scene(WIDTH as u32, HEIGHT as u32);
        super::cpu_ray_tracer::update(&mut scene, 0, 0.0);
        let mut buffer: [u8; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
        let mut index = 0;
        for i in buffer.iter_mut() {
            let color: u8 = ((scene.pixels[index] as u8) << 16)
                + ((scene.pixels[index + 1] as u8) << 8)
                + (scene.pixels[index + 2] as u8);
            *i = color;
            index += 3;
        }
        Self {
            egui_mq: egui_mq::EguiMq::new(ctx),
            texture: Texture::from_rgba8(ctx, WIDTH as u16, HEIGHT as u16, &buffer),
        }
    }
}

impl mq::EventHandler for Stage {
    fn update(&mut self, _ctx: &mut mq::Context) {}

    fn draw(&mut self, ctx: &mut mq::Context) {
        ctx.clear(Some((1., 1., 1., 1.)), None, None);
        ctx.begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        ctx.end_render_pass();

        // Run the UI code:
        self.egui_mq.run(ctx, |_mq_ctx, egui_ctx| {
            egui::Window::new("egui ❤ miniquad").show(egui_ctx, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.checkbox(&mut false, "Show egui demo windows");

                #[cfg(not(target_arch = "wasm32"))]
                {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                }
            });
        });

        // Draw things behind egui here

        self.egui_mq.draw(ctx);

        // Draw things in front of egui here

        ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, ctx: &mut mq::Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut mq::Context, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        _ctx: &mut mq::Context,
        character: char,
        _keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut mq::Context,
        keycode: mq::KeyCode,
        keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut mq::Context, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}

pub fn run() {
    let conf = mq::conf::Conf {
        high_dpi: true,
        ..Default::default()
    };

    mq::start(conf, |mut ctx| Box::new(Stage::new(&mut ctx)));
}
