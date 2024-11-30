mod app;
mod audio;
mod basics;
mod gui;
mod renderer;
mod utils;

fn main() {
    pollster::block_on(app::start());
}
