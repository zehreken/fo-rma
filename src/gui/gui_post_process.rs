use crate::{
    app::UiEvent,
    color_utils,
    shader_utils::{effect_to_name, Effect},
};
use std::collections::HashMap;

pub fn draw(
    ctx: &egui::Context,
    is_open: &mut bool,
    effect_to_active: &mut HashMap<Effect, bool>,
    color_palette: &mut usize,
    ui_events: &mut Vec<UiEvent>,
) {
    let mut has_effects_changed = false;
    egui::Window::new("post process")
        .open(is_open)
        .show(ctx, |ui| {
            ctx.request_repaint();

            for (label, active) in effect_to_active.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(effect_to_name(*label));
                    if ui.checkbox(active, "").clicked() {
                        has_effects_changed = true;
                    };
                    // if ui.button("⏶").clicked() {
                    //     has_effects_changed = true;
                    // }
                    // if ui.button("⏷").clicked() {
                    //     has_effects_changed = true;
                    // }
                });
            }
            if has_effects_changed {
                ui_events.push(UiEvent::UpdateEffects);
            }
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("color palette: ");
                if ui.button("⏴").clicked() {
                    *color_palette -= 1;
                    *color_palette = (*color_palette).clamp(0, color_utils::COLORS.len() - 1);
                }
                ui.label(format!("{}", color_palette));
                if ui.button("⏵").clicked() {
                    *color_palette += 1;
                    *color_palette = (*color_palette).clamp(0, color_utils::COLORS.len() - 1);
                }
            });
        });
}
