use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::*;
use std::time::Duration;

pub fn trace_with_sdl(width: u32, height: u32) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rt-rs", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut scene = super::cpu_path_tracer::create_scene(width, height, 3);

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut framebuffer = texture_creator
        .create_texture(PixelFormatEnum::RGB24, TextureAccess::Static, width, height)
        .unwrap();

    const CHANNEL_COUNT: usize = 3;
    framebuffer
        .update(None, &scene.pixels, width as usize * CHANNEL_COUNT)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        let mut keys: u8 = 0; // 0000ADWS
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => keys += 1 << 3,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => keys += 1 << 2,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => keys += 1 << 1,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => keys += 1,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        super::cpu_path_tracer::update(&mut scene, keys, 1.0);

        framebuffer
            .update(None, &scene.pixels, width as usize * CHANNEL_COUNT)
            .unwrap();

        canvas
            .copy(&framebuffer, None, Rect::new(0, 0, width, height))
            .unwrap();

        canvas.present();
        canvas.clear();

        std::thread::sleep(Duration::from_millis(20));
    }
}
