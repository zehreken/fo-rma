use wgpu::{Color, Device, Texture, TextureFormat, TextureView};

use crate::{basics::primitive::Primitive, renderer::Renderer, utils};

const BG_COLOR: [f32; 3] = utils::CCP.palette[0];

pub fn save_image(renderer: &mut Renderer, primitives: &Vec<Box<dyn Primitive>>) {
    let width = 1080;
    let height = 1080;

    let bytes_per_pixel = 4; // For Rgba8Unorm (4 bytes per pixel)
                             // let unaligned_bytes_per_row = width * bytes_per_pixel;
                             // let aligned_bytes_per_row = ((unaligned_bytes_per_row + 255) / 256) * 256; // Align to 256
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

    let c_bg_color = utils::srgb_to_linear(BG_COLOR, utils::GAMMA);
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

    // Set your existing pipeline and render primitives
    render_pass.set_pipeline(&renderer.pipeline_data.render_pipeline);
    for primitive in primitives {
        render_pass.set_bind_group(0, &renderer.pipeline_data.uniform_bind_group, &[0]);
        render_pass.set_bind_group(
            1,
            &renderer
                .pipeline_data
                .light_data
                .as_ref()
                .unwrap()
                .bind_group,
            &[],
        );
        primitive.draw(&mut render_pass);
    }

    drop(render_pass);
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

    // Use an image library to save the data
    use image::{ImageBuffer, Rgba};

    let mut tightly_packed_data = Vec::new();

    for y in 0..height {
        let start = (y * aligned_bytes_per_row) as usize;
        let end = start + (width * 4) as usize; // Assuming 4 bytes per pixel (RGBA8)
        tightly_packed_data.extend_from_slice(&data[start..end]);
    }

    // Create the image buffer with tightly packed pixel data
    let buffer: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(width, height, tightly_packed_data).unwrap();

    // Save the image
    buffer.save("out/basic.png").unwrap();

    // buffer.unmap(); // This is a later version of wgpu

    println!("Saving image");
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
