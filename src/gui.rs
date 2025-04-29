use crate::audio::sequencer::Sequencer;
use egui::epaint::Shadow;
use egui::ViewportId;
use egui_wgpu::wgpu::TextureFormat;
use egui_wgpu::{Renderer, ScreenDescriptor};
use egui_winit::{
    egui::{self, ClippedPrimitive, Context, TexturesDelta},
    State,
};
use top_bar::TopBar;
use wgpu::{Device, Queue};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub mod gui_envelope;
pub mod gui_oscillator;
pub mod gui_sequencer;
pub mod gui_sequencer_list;
pub mod gui_vfx;
pub mod top_bar;

pub struct Gui {
    ctx: Context,
    state: State,
    renderer: Renderer,
    screen_descriptor: ScreenDescriptor,
    paint_jobs: Vec<ClippedPrimitive>,
    textures: TexturesDelta,
    top_bar: TopBar,
    settings: Settings,
}

pub struct Settings {
    pub show_sequencer_list: bool,
    pub show_sequencer: bool,
    pub show_oscillator_inspector: bool,
    pub show_vfx: bool,
    pub show_envelope: bool,
    pub selected: u8,
}

impl Gui {
    pub fn new(window: &Window, device: &wgpu::Device, texture_format: TextureFormat) -> Self {
        let scale_factor = window.scale_factor();
        let size = window.inner_size();
        let max_texture_size = device.limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            ViewportId::ROOT,
            window,
            Some(scale_factor as f32),
            Some(max_texture_size),
        );
        egui_ctx.style_mut(|style| {
            style.visuals.window_shadow = Shadow::NONE;
            style.visuals.popup_shadow = Shadow::NONE
        });

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [size.width, size.height],
            pixels_per_point: scale_factor as f32,
        };
        let renderer = Renderer::new(device, TextureFormat::Rgba8Unorm, None, 1);
        let textures = TexturesDelta::default();

        let top_bar = TopBar::new();

        Self {
            ctx: egui_ctx,
            state: egui_state,
            renderer,
            screen_descriptor,
            paint_jobs: vec![],
            textures,
            top_bar,
            settings: Settings {
                show_sequencer_list: true,
                show_sequencer: true,
                show_oscillator_inspector: true,
                show_vfx: false,
                show_envelope: false,
                selected: 0,
            },
        }
    }

    pub fn handle_event(&mut self, window: &Window, event: &winit::event::WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>, scale_factor: f64) {
        self.screen_descriptor = ScreenDescriptor {
            size_in_pixels: [size.width, size.height],
            pixels_per_point: scale_factor as f32,
        };
    }

    // update scale factor

    pub fn render(
        &mut self,
        window: &Window,
        render_target: &wgpu::TextureView,
        device: &Device,
        queue: &Queue,
        sequencers: &mut Vec<Sequencer>,
        fps: f32,
    ) {
        let raw_input = self.state.take_egui_input(window);
        let output = self.ctx.run(raw_input, |egui_ctx| {
            self.top_bar.draw(egui_ctx, &mut self.settings, fps);
            if self.settings.show_oscillator_inspector {
                gui_oscillator::draw(
                    egui_ctx,
                    &mut sequencers[self.settings.selected as usize],
                    &mut self.settings.show_oscillator_inspector,
                );
            }
            if self.settings.show_vfx {
                gui_vfx::draw(egui_ctx, &mut self.settings.show_vfx);
            }
            if self.settings.show_sequencer_list {
                gui_sequencer_list::draw(
                    egui_ctx,
                    &mut self.settings.selected,
                    &mut self.settings.show_sequencer_list,
                );
            }
            if self.settings.show_sequencer {
                gui_sequencer::draw(
                    egui_ctx,
                    &mut sequencers[self.settings.selected as usize],
                    &mut self.settings.show_sequencer,
                );
            }
            if self.settings.show_envelope {
                gui_envelope::draw(egui_ctx, &mut self.settings.show_envelope);
            }
        });

        self.textures.append(output.textures_delta);
        self.state
            .handle_platform_output(window, output.platform_output);
        self.paint_jobs = self
            .ctx
            .tessellate(output.shapes, window.scale_factor() as f32);

        // Upload all resources to the GPU.
        for (id, image_delta) in &self.textures.set {
            self.renderer
                .update_texture(device, queue, *id, image_delta);
        }

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("gui_renderer_encoder"),
        });
        self.renderer.update_buffers(
            device,
            queue,
            &mut encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Render egui with WGPU
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("egui"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: render_target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            ..Default::default()
        });

        self.renderer
            .render(&mut render_pass, &self.paint_jobs, &self.screen_descriptor);

        // dropping render_pass here
        drop(render_pass);

        queue.submit(Some(encoder.finish()));

        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.renderer.free_texture(id);
        }
    }
}
