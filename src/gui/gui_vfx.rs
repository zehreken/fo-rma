use crate::rendering::post_processor::Effect;

pub fn draw(ctx: &egui::Context, is_open: &mut bool, effect: &mut Effect) {
    egui::Window::new("VFX").open(is_open).show(ctx, |ui| {
        ctx.request_repaint();

        ui.horizontal(|ui| {
            ui.label("effect: ");
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", effect))
                .show_ui(ui, |ui| {
                    ui.selectable_value(effect, Effect::None, "None");
                    ui.selectable_value(effect, Effect::Noise, "Noise");
                    ui.selectable_value(effect, Effect::Pixelate, "Pixelate");
                    ui.selectable_value(effect, Effect::InvertColor, "InvertColor")
                });
        });
    });
}
