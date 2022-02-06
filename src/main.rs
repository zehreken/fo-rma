extern crate image;
extern crate rand;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::*;
use std::time::Duration;
mod fps_utils;
use crate::fps_utils::fps_utils::*;
mod cpu_tracer;
use minifb::{Key, Window, WindowOptions};
mod strict_covers;
mod thread_test;

fn main() {
    // thread_test::test_thread();
    // return;

    let mut fps_counter = FpsCounter::new();

    // tracer::save_image(800, 600, 500);
    // tracer::save_image_mt(512, 512, 50);
    trace_with_minifb(400, 300, &mut fps_counter);
    // trace_with_sdl(400, 300);

    println!("Average fps: {}", fps_counter.average_frames_per_second());
}

fn trace_with_sdl(width: u32, height: u32) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rt-rs", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut scene = cpu_tracer::create_scene(width, height, 3);

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

        cpu_tracer::update(&mut scene, keys);

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

fn trace_with_minifb(width: usize, height: usize, fps_counter: &mut FpsCounter) {
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "rt-rs - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut scene = cpu_tracer::create_scene(width as u32, height as u32, 3);

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut keys: u8 = 0; // 0000ADWS
        if window.is_key_down(Key::A) {
            keys += 1 << 3;
        }
        if window.is_key_down(Key::D) {
            keys += 1 << 2;
        }
        if window.is_key_down(Key::W) {
            keys += 1 << 1;
        }
        if window.is_key_down(Key::S) {
            keys += 1;
        }
        cpu_tracer::update(&mut scene, keys);
        let mut index = 0;
        for i in buffer.iter_mut() {
            let color: u32 = ((scene.pixels[index] as u32) << 16)
                + ((scene.pixels[index + 1] as u32) << 8)
                + (scene.pixels[index + 2] as u32);
            *i = color;
            index += 3;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, width, height).unwrap();

        fps_counter.tick();
    }
}
