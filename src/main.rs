mod app;
mod audio;
mod basics;
mod gui;
mod renderer;
mod rendering;
mod rendering_utils;
mod save_image;
mod utils;

fn main() {
    pollster::block_on(app::start());
}
