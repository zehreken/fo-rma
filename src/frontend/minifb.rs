use super::misc::fps_utils::FpsCounter;
use minifb::{Key, Window, WindowOptions};

pub fn trace_with_minifb(width: usize, height: usize, fps_counter: &mut FpsCounter) {
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

    let mut scene = super::cpu_path_tracer::create_scene(width as u32, height as u32, 3);

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
        super::cpu_path_tracer::update(&mut scene, keys);
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
