pub mod cpu_ray_tracer;
mod frontend;
mod misc;
pub mod shapes;
use macroquad::prelude::Conf;
use misc::fps_utils::FpsCounter;

#[macroquad::main(get_config)]
async fn main() {
    let mut fps_counter = FpsCounter::new();
    let future = frontend::macroquad::run(&mut fps_counter);

    future.await;

    println!("Average FPS: {}", fps_counter.average_frames_per_second());
}

pub fn get_config() -> Conf {
    Conf {
        window_title: "fōrma".to_owned(),
        window_width: 400 + frontend::macroquad::SIDE_PANEL_WIDTH as i32,
        window_height: 400,
        fullscreen: false,
        ..Default::default()
    }
}
