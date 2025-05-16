use crate::{color_utils, rendering::post_processor::Effect};

pub fn draw(
    ctx: &egui::Context,
    is_open: &mut bool,
    effect: &mut Effect,
    color_palette: &mut usize,
) {
    egui::Window::new("post process")
        .open(is_open)
        .show(ctx, |ui| {
            ctx.request_repaint();

            ui.horizontal(|ui| {
                ui.label("effect: ");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", effect))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(effect, Effect::None, "none");
                        ui.selectable_value(effect, Effect::Noise, "noise");
                        ui.selectable_value(effect, Effect::Pixelate, "pixelate");
                        ui.selectable_value(effect, Effect::InvertColor, "invertColor");
                        ui.selectable_value(effect, Effect::Wave, "wave");
                        ui.selectable_value(effect, Effect::Interlace, "interlace");
                        ui.selectable_value(effect, Effect::FlipAxis, "flip axis");
                        ui.selectable_value(effect, Effect::Grayscale, "grayscale");
                        ui.selectable_value(effect, Effect::Step, "step");
                    });
            });
            ui.horizontal(|ui| {
                ui.label("color palette: ");
                if ui.button("<").clicked() {
                    *color_palette -= 1;
                    *color_palette = (*color_palette).clamp(0, color_utils::COLORS.len() - 1);
                }
                ui.label(format!("{}", color_palette));
                if ui.button(">").clicked() {
                    *color_palette += 1;
                    *color_palette = (*color_palette).clamp(0, color_utils::COLORS.len() - 1);
                }
            });
        });
}
