pub mod cpu_path_tracer;
mod frontend;
mod misc;
use misc::fps_utils::FpsCounter;

fn main() {
    // thread_test::test_thread();
    // return;

    let mut fps_counter = FpsCounter::new();

    // cpu_path_tracer::save_image(512, 512, 50);
    // cpu_path_tracer::save_image_mt(&mut cpu_path_tracer::create_scene(1920, 1080, 3), 5);
    frontend::minifb::run(600, 600, &mut fps_counter);
    // let stage = frontend::miniquad::run();

    println!("Average fps: {}", fps_counter.average_frames_per_second());
}
