pub mod cpu_path_tracer;
mod frontend;
mod misc;
pub mod shapes;
use macroquad::prelude::Conf;
use misc::fps_utils::FpsCounter;

fn main_() {
    // thread_test::test_thread();
    // return;

    let mut fps_counter = FpsCounter::new();

    // cpu_path_tracer::save_image(512, 512, 50);
    // cpu_path_tracer::save_image_mt(&mut cpu_path_tracer::create_scene(1920, 1080, 3), 5);
    // frontend::minifb::run(600, 600, &mut fps_counter);
    // frontend::miniquad::run();

    println!("Average fps: {}", fps_counter.average_frames_per_second());
}

#[macroquad::main(get_config)]
async fn main() {
    let fps_counter = FpsCounter::new();
    let future = frontend::macroquad::run(fps_counter);

    future.await
}

pub fn get_config() -> Conf {
    Conf {
        window_title: "fōrma".to_owned(),
        window_width: 600,
        window_height: 600,
        fullscreen: false,
        ..Default::default()
    }
}
