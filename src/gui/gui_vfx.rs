pub fn draw(ctx: &egui::Context, is_open: &mut bool) {
    egui::Window::new("VFX").open(is_open).show(ctx, |ui| {
        ctx.request_repaint();

        ui.label("invert_color")
    });
}
