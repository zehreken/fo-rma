use crate::{
    app::UiEvent,
    color_utils,
    shader_utils::{effect_to_name, Effect},
};
use wgpu::naga::FastIndexMap;

pub fn draw(
    ctx: &egui::Context,
    is_open: &mut bool,
    effect_to_active: &mut FastIndexMap<Effect, bool>,
    color_palette: &mut usize,
    ui_events: &mut Vec<UiEvent>,
) {
    let mut has_effects_changed = false;
    egui::Window::new("post process")
        .open(is_open)
        .show(ctx, |ui| {
            ctx.request_repaint();

            let mut swap_pair: Option<(usize, usize)> = None;
            let length = effect_to_active.len();
            for (index, (effect, active)) in effect_to_active.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    if ui.button("⏶").clicked() {
                        if index > 0 {
                            swap_pair = Some((index, index - 1));
                            has_effects_changed = true;
                        }
                    }
                    if ui.button("⏷").clicked() {
                        if index < length - 1 {
                            swap_pair = Some((index, index + 1));
                            has_effects_changed = true;
                        }
                    }
                    if ui.checkbox(active, "").clicked() {
                        has_effects_changed = true;
                    };
                    ui.label(effect_to_name(*effect));
                });
            }
            if let Some(sp) = swap_pair {
                effect_to_active.swap_indices(sp.0, sp.1);
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
