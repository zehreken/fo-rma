use crate::{
    basics::{level::Level, uniforms::ObjectUniform},
    renderer::Renderer,
    utils,
};
use rand::Rng;
use std::mem;
use wgpu::{Color, Device, Texture, TextureFormat, TextureView};

const BG_COLOR: [f32; 3] = utils::CCP.palette[0];

pub fn save_image(renderer: &mut Renderer, level: &Level) {
    let width = renderer.surface_config.width;
    let height = renderer.surface_config.height;

    let bytes_per_pixel = 4; // For Rgba8Unorm (4 bytes per pixel)
    let aligned_bytes_per_row = ((width * bytes_per_pixel + 255) & !255) as u32;

    let (high_res_texture, high_res_view) = crate::save_image::create_high_res_texture(
        &renderer.device,
        width,
        height,
        renderer.surface_config.format,
    );

    let mut encoder = renderer
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("high_res_encoder"),
        });

    let c_bg_color = utils::force_srgb_to_linear(BG_COLOR, utils::GAMMA);
    let bg_color = Color {
        r: c_bg_color[0] as f64,
        g: c_bg_color[1] as f64,
        b: c_bg_color[2] as f64,
        a: 1.0,
    };

    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("high_res_render_pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &high_res_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(bg_color),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &renderer.depth_texture,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
    });

    let uniform_alignment =
        renderer.device.limits().min_uniform_buffer_offset_alignment as wgpu::BufferAddress;
    let uniform_size = mem::size_of::<ObjectUniform>() as wgpu::BufferAddress;
    let aligned_uniform_size = (uniform_size + uniform_alignment - 1) & !(uniform_alignment - 1);

    // Set your existing pipeline and render primitives
    for (i, object) in level.objects.iter().enumerate() {
        let uniform_offset = (i as wgpu::BufferAddress) * aligned_uniform_size;
        render_pass.set_pipeline(&object.material().render_pipeline);
        render_pass.set_bind_group(
            0,
            &renderer.generic_uniform_data.uniform_bind_group,
            &[uniform_offset as u32],
        );
        render_pass.set_bind_group(1, &renderer.light_uniform_data.uniform_bind_group, &[]);
        render_pass.set_bind_group(2, &object.material().bind_group, &[]);
        object.draw(&mut render_pass);
    }

    drop(render_pass);

    const DEBUG: bool = false;
    if DEBUG {
        let mut debug_render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("debug_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &high_res_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &renderer.depth_texture,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        debug_render_pass.set_pipeline(&renderer.debug_render_pipeline);
        for (i, primitive) in level.objects.iter().enumerate() {
            let uniform_offset = (i as wgpu::BufferAddress) * aligned_uniform_size;
            debug_render_pass.set_bind_group(
                0,
                &renderer.debug_uniform_data.uniform_bind_group,
                &[uniform_offset as u32],
            );
            primitive.draw(&mut debug_render_pass);
        }
        drop(debug_render_pass);
    }

    renderer.queue.submit(Some(encoder.finish()));

    // Read the high res texture and save it to a file
    let buffer_size = (aligned_bytes_per_row * height) as wgpu::BufferAddress;
    let buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("texture_buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let command_encoder_desc = wgpu::CommandEncoderDescriptor {
        label: Some("image_save_encoder"),
    };

    let mut encoder = renderer
        .device
        .create_command_encoder(&command_encoder_desc);
    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &high_res_texture,
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

    renderer.queue.submit(Some(encoder.finish()));
    let buffer_slice = buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |result| {
        if let Err(e) = result {
            eprintln!("Failed to map buffer: {:?}", e);
        }
    });

    renderer.device.poll(wgpu::Maintain::Wait);

    let data = buffer_slice.get_mapped_range();

    // Fix Bgra to Rgba conversion
    let bgra_to_rgba = renderer.surface_config.format == wgpu::TextureFormat::Bgra8Unorm
        || renderer.surface_config.format == wgpu::TextureFormat::Bgra8UnormSrgb;

    // Use an image library to save the data
    use image::{ImageBuffer, Rgba};

    let tightly_packed_data = pixelated(
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
            if bgra_to_rgba {
                pixel.swap(0, 2);
            }

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

pub fn create_high_res_texture(
    device: &Device,
    width: u32,
    height: u32,
    format: TextureFormat,
) -> (Texture, TextureView) {
    let size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let desc = wgpu::TextureDescriptor {
        label: Some("high_res_texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    };
    let texture = device.create_texture(&desc);
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}
