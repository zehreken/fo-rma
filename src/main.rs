mod app;
mod audio;
mod basics;
mod color_utils;
mod gui;
mod material;
mod misc;
mod renderer;
mod rendering;
mod rendering_utils;
mod save_image;
mod shader_utils;

fn main() {
    pollster::block_on(app::start());
}
