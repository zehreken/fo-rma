// pub mod cpu_ray_tracer;
// mod frontend;
// mod misc;
// pub mod shapes;
// use macroquad::prelude::Conf;

// #[macroquad::main(get_config)]
// async fn main() {
//     let future = frontend::macroquad::run();

//     future.await;
// }

// pub fn get_config() -> Conf {
//     Conf {
//         window_title: "fōrma".to_owned(),
//         window_width: 400 + frontend::macroquad::SIDE_PANEL_WIDTH as i32,
//         window_height: 400,
//         fullscreen: false,
//         ..Default::default()
//     }
// }

mod frontend;

fn main() {
    pollster::block_on(frontend::wnt::run());
}
