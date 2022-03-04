pub mod cpu_path_tracer;
mod frontend;
mod misc;
use misc::fps_utils::FpsCounter;

fn main() {
    // thread_test::test_thread();
    // return;

    let mut fps_counter = FpsCounter::new();

    // cpu_path_tracer::save_image(512, 512, 50);
    cpu_path_tracer::save_image_mt(512, 512, 500);
    // frontend::minifb::trace_with_minifb(400, 300, &mut fps_counter);
    // frontend::sdl::trace_with_sdl(400, 300);

    println!("Average fps: {}", fps_counter.average_frames_per_second());
}
