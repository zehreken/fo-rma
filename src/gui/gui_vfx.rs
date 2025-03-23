pub fn draw(ctx: &egui::Context) {
    egui::Window::new("vfx").show(ctx, |ui| {
        ctx.request_repaint();

        ui.label("invert_color")
    });
}
