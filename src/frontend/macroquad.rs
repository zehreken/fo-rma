use macroquad::prelude::*;

pub async fn run() {
    const WIDTH: usize = 600;
    const HEIGHT: usize = 600;
    const CHANNELS: usize = 3;
    let mut scene = super::cpu_path_tracer::create_scene(WIDTH as u32, HEIGHT as u32, 3);
    super::cpu_path_tracer::update(&mut scene, 0, 0.0);
    let mut buffer: [u8; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];
    let mut index = 0;
    for i in buffer.iter_mut() {
        if index < scene.pixels.len() {
            let color: u8 = ((scene.pixels[index] as u8) << 16)
                + ((scene.pixels[index + 1] as u8) << 8)
                + (scene.pixels[index + 2] as u8);
            *i = color;
            index += 3;
        }
    }

    let texture: Texture2D = Texture2D::from_rgba8(WIDTH as u16, HEIGHT as u16, &buffer);

    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                ui.label("Test");
            });
        });

        // Draw things before egui
        draw_texture(texture, 0.0, 0.0, WHITE);

        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}
