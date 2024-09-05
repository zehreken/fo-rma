mod app;
mod basics;
mod gui;
mod rend;
mod renderer;

fn main() {
    pollster::block_on(app::start());
}
