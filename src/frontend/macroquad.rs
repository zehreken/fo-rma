use crate::misc::fps_utils::FpsCounter;
use macroquad::{prelude::*, window};

pub async fn run(fps_counter: &mut FpsCounter) {
    let width = window::screen_width() as u32;
    let height = window::screen_height() as u32;
    let mut model = super::cpu_ray_tracer::tracer::create_model(width, height);

    let mut image = Image::gen_image_color(width as u16, height as u16, WHITE);
    let texture: Texture2D = Texture2D::from_image(&image);

    let mut orientation = crate::cpu_ray_tracer::primitives::vec3::Vec3::new(-1.0, 0.0, 0.0);
    loop {
        clear_background(BLACK);

        // Process keys, mouse etc.
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("fōrma").show(egui_ctx, |ui| {
                ui.label("Test");
                ui.add(egui::Slider::new(&mut orientation.x, -1.0..=1.0).text("rotation_x"));
                ui.add(egui::Slider::new(&mut orientation.y, -1.0..=1.0).text("rotation_y"));
                ui.add(egui::Slider::new(&mut orientation.z, -1.0..=1.0).text("rotation_z"));
            });
        });

        // Draw things before egui
        let mut keys: u8 = 0; // 00EQADWS
        if is_key_down(KeyCode::E) {
            // Down
            keys += 1 << 5;
        }
        if is_key_down(KeyCode::Q) {
            // Up
            keys += 1 << 4;
        }
        if is_key_down(KeyCode::A) {
            keys += 1 << 3;
        }
        if is_key_down(KeyCode::D) {
            keys += 1 << 2;
        }
        if is_key_down(KeyCode::W) {
            keys += 1 << 1;
        }
        if is_key_down(KeyCode::S) {
            keys += 1;
        }
        if is_key_pressed(KeyCode::R) {
            super::cpu_ray_tracer::tracer::save_image(&mut model, 50);
        }
        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        model.scene.objects[0].rotate(orientation);
        super::cpu_ray_tracer::tracer::update(&mut model, keys, fps_counter.get_delta_time());
        let mut pixel_index: u32 = 0;
        for i in (0..model.pixels.len()).step_by(3) {
            let color = Color::new(
                model.pixels[i] as f32 / 255.0,
                model.pixels[i + 1] as f32 / 255.0,
                model.pixels[i + 2] as f32 / 255.0,
                1.0,
            );
            image.set_pixel(
                pixel_index % width as u32,
                pixel_index / width as u32,
                color,
            );
            pixel_index += 1;
        }
        texture.update(&image);
        draw_texture(texture, 0.0, 0.0, WHITE);

        egui_macroquad::draw();

        // Draw things after egui

        fps_counter.tick();
        std::thread::sleep(std::time::Duration::from_millis(50));
        next_frame().await;
    }
}
