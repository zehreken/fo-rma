mod app;
mod audio;
mod basics;
mod color_utils;
mod gui;
mod renderer;
mod rendering;
mod rendering_utils;
mod save_image;

fn main() {
    pollster::block_on(app::start());
}
