pub fn draw(ctx: &egui::Context, selected: &mut u8, is_open: &mut bool) {
    egui::Window::new("Sequencers")
        .open(is_open)
        .show(ctx, |ui| {
            for i in 0..2 {
                if ui
                    .add(egui::RadioButton::new(
                        i == *selected,
                        format!("{}", (i + 1)),
                    ))
                    .clicked()
                {
                    *selected = i;
                }
            }
        });
}
