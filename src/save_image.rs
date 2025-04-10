use rand::Rng;
use wgpu::{Device, Queue, SurfaceConfiguration, Texture, TextureView};

use crate::color_utils;

pub fn save_image(
    device: &Device,
    queue: &Queue,
    surface_config: &SurfaceConfiguration,
    texture: &Texture,
) {
    let width = surface_config.width;
    let height = surface_config.height;

    let bytes_per_pixel = 4; // For Rgba8Unorm (4 bytes per pixel)
    let aligned_bytes_per_row = ((width * bytes_per_pixel + 255) & !255) as u32;

    // Read the high res texture and save it to a file
    let buffer_size = (aligned_bytes_per_row * height) as wgpu::BufferAddress;
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("texture_buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let command_encoder_desc = wgpu::CommandEncoderDescriptor {
        label: Some("image_save_encoder"),
    };

    let mut encoder = device.create_command_encoder(&command_encoder_desc);
    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(aligned_bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    queue.submit(Some(encoder.finish()));
    let buffer_slice = buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |result| {
        if let Err(e) = result {
            eprintln!("Failed to map buffer: {:?}", e);
        }
    });

    device.poll(wgpu::Maintain::Wait);

    let data = buffer_slice.get_mapped_range();

    // Fix Bgra to Rgba conversion
    let bgra_to_rgba = surface_config.format == wgpu::TextureFormat::Bgra8Unorm
        || surface_config.format == wgpu::TextureFormat::Bgra8UnormSrgb;

    // Use an image library to save the data
    use image::{ImageBuffer, Rgba};

    let tightly_packed_data = unprocessed(
        &data,
        width,
        height,
        bytes_per_pixel,
        aligned_bytes_per_row,
        bgra_to_rgba,
    );

    // Create the image buffer with tightly packed pixel data
    let buffer: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(width, height, tightly_packed_data).unwrap();

    // Save the image
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let image_path = format!("out/basic-{timestamp}.png");
    buffer.save(&image_path).unwrap();

    // buffer.unmap(); // This is a later version of wgpu

    println!("Saving image {}", image_path);
}

fn unprocessed(
    data: &[u8],
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
    aligned_bytes_per_row: u32,
    bgra_to_rgba: bool,
) -> Vec<u8> {
    let mut tightly_packed_data = Vec::new();
    for y in 0..height {
        let start = (y * aligned_bytes_per_row) as usize;
        let end = start + (width * bytes_per_pixel) as usize; // Assuming 4 bytes per pixel (RGBA8)

        for x in 0..width {
            let pixel_start = start + (x * bytes_per_pixel) as usize;
            let pixel_end = pixel_start + bytes_per_pixel as usize;
            let mut pixel = data[pixel_start..pixel_end].to_vec();
            // if bgra_to_rgba {
            //     pixel.swap(0, 2);
            // }
            pixel = color_utils::srgb_encode(pixel).to_vec();

            tightly_packed_data.extend_from_slice(&pixel);
        }
    }

    tightly_packed_data
}

fn random_noise(
    data: &[u8],
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
    aligned_bytes_per_row: u32,
    bgra_to_rbga: bool,
) -> Vec<u8> {
    let mut tightly_packed_data = Vec::new();
    for y in 0..height {
        let start = (y * aligned_bytes_per_row) as usize;

        for x in 0..width {
            let pixel_start = start + (x * bytes_per_pixel) as usize;
            let pixel_end = pixel_start + bytes_per_pixel as usize;
            let mut pixel = data[pixel_start..pixel_end].to_vec();

            if bgra_to_rbga {
                pixel.swap(0, 2);
            }
            let mut rng = rand::thread_rng();
            for channel in 0..3 {
                // skip alpha channel
                pixel[channel] = pixel[channel].saturating_add(rng.gen_range(0..50));
            }

            tightly_packed_data.extend_from_slice(&pixel);
        }
    }

    tightly_packed_data
}

fn monochrome_noise(
    data: &[u8],
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
    aligned_bytes_per_row: u32,
    bgra_to_rbga: bool,
) -> Vec<u8> {
    let mut tightly_packed_data = Vec::new();
    for y in 0..height {
        let start = (y * aligned_bytes_per_row) as usize;

        for x in 0..width {
            let pixel_start = start + (x * bytes_per_pixel) as usize;
            let pixel_end = pixel_start + bytes_per_pixel as usize;
            let mut pixel = data[pixel_start..pixel_end].to_vec();

            if bgra_to_rbga {
                pixel.swap(0, 2);
            }

            let mut rng = rand::thread_rng();
            let rnd = rng.gen_range(0..20);
            for channel in 0..3 {
                pixel[channel] = pixel[channel].saturating_add(rnd);
            }

            tightly_packed_data.extend_from_slice(&pixel);
        }
    }

    tightly_packed_data
}

fn pixelated(
    data: &[u8],
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
    aligned_bytes_per_row: u32,
    bgra_to_rbga: bool,
) -> Vec<u8> {
    let mut tightly_packed_data = Vec::new();
    const PIXEL_SIZE: usize = 10;
    for y in (0..height).step_by(PIXEL_SIZE) {
        let start = (y * aligned_bytes_per_row) as usize;
        let mut pixelated_row = Vec::new();

        for x in (0..width).step_by(PIXEL_SIZE) {
            let pixel_start = start + (x * bytes_per_pixel) as usize;
            let pixel_end = pixel_start + bytes_per_pixel as usize;
            let mut pixel = data[pixel_start..pixel_end].to_vec();

            if bgra_to_rbga {
                pixel.swap(0, 2);
            }

            for _ in 0..PIXEL_SIZE {
                pixelated_row.extend_from_slice(&pixel);
            }
        }

        for _ in 0..PIXEL_SIZE {
            tightly_packed_data.extend_from_slice(&pixelated_row);
        }
    }

    tightly_packed_data
}
