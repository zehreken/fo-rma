use super::Settings;
use crate::app::UiEvent;
use egui::{Color32, RichText};
use egui_winit::egui::{self, Context};

pub fn draw(ctx: &Context, settings: &mut Settings, ui_events: &mut Vec<UiEvent>, fps: f32) {
    egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.label(RichText::new(format!("FPS: {0:.2}", fps)).color(Color32::GREEN));
            ui.menu_button("file", |ui| {
                if ui.button("oscillator").clicked() {
                    settings.show_oscillator_inspector = true;
                    ui.close_menu();
                }
                if ui.button("sequencers").clicked() {
                    settings.show_sequencers = true;
                    ui.close_menu();
                }
                if ui.button("VFX").clicked() {
                    settings.show_vfx = true;
                    ui.close_menu();
                }
            });
            ui.menu_button("song", |ui| {
                if ui.button("save").clicked() {
                    ui_events.push(UiEvent::SaveSong);
                    ui.close_menu();
                }
                if ui.button("load").clicked() {
                    ui_events.push(UiEvent::LoadSong);
                    ui.close_menu();
                }
                if ui.button("clear").clicked() {
                    ui_events.push(UiEvent::ClearSong);
                    ui.close_menu();
                }
            })
        });
    });
}
