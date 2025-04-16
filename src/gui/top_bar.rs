use egui::{Color32, RichText};
use egui_winit::egui::{self, Context};

use super::Settings;

pub struct TopBar {
    is_window_open: bool,
}

impl TopBar {
    pub fn new() -> Self {
        Self {
            is_window_open: false,
        }
    }

    pub fn draw(&mut self, ctx: &Context, settings: &mut Settings, fps: f32) {
        egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.label(RichText::new(format!("FPS: {0:.2}", fps)).color(Color32::GREEN));
                ui.menu_button("File", |ui| {
                    if ui.button("About...").clicked() {
                        self.is_window_open = true;
                        ui.close_menu();
                    }
                    if ui.button("oscillator").clicked() {
                        settings.show_oscillator_inspector = true;
                        ui.close_menu();
                    }
                    if ui.button("sequencer list").clicked() {
                        settings.show_sequencer_list = true;
                        ui.close_menu();
                    }
                    if ui.button("vfx").clicked() {
                        settings.show_vfx = true;
                        ui.close_menu();
                    }
                });
            });
        });

        egui::Window::new("Hello, winit-wgpu-egui")
            .open(&mut self.is_window_open)
            .show(ctx, |ui| {
                ui.label(
                    "This is the most basic example of how to use winit, wgpu and egui together.",
                );
                ui.label("Mandatory heart: ♥");

                ui.separator();

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x /= 2.0;
                    ui.label("Learn more about wgpu at");
                    ui.hyperlink("https://docs.rs/winit");
                });
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x /= 2.0;
                    ui.label("Learn more about winit at");
                    ui.hyperlink("https://docs.rs/wgpu");
                });
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x /= 2.0;
                    ui.label("Learn more about egui at");
                    ui.hyperlink("https://docs.rs/egui");
                });
            });
    }
}
