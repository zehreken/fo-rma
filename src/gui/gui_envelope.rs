// OBSOLETE Delete later
pub fn draw(ctx: &egui::Context, /* probably envelope will be passed */ is_open: &mut bool) {
    egui::Window::new("Envelope").open(is_open).show(ctx, |ui| {
        ctx.request_repaint();

        ui.horizontal(|ui| {
            ui.label("attack: ");
            ui.add(egui::Slider::new(&mut 1.0, 0.0..=1.0));
        });
        ui.horizontal(|ui| {
            ui.label("decay: ");
            ui.add(egui::Slider::new(&mut 1.0, 0.0..=1.0));
        });
        ui.horizontal(|ui| {
            ui.label("sustain: ");
            ui.add(egui::Slider::new(&mut 1.0, 0.0..=1.0));
        });
        ui.horizontal(|ui| {
            ui.label("release: ");
            ui.add(egui::Slider::new(&mut 1.0, 0.0..=1.0));
        });
        if ui.button("play").clicked() {
            println!("try envelope");
        }
    });
}
