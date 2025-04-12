use crate::{
    basics::{
        primitive::Primitive,
        quad::Quad,
        uniforms::{ObjectUniform, ScreenQuadUniform, UniformTrait},
    },
    material::{
        post_process_material::PostProcessMaterial, unlit_color_material::UnlitColorMaterial,
    },
};
use wgpu::{
    BindGroup, BindGroupLayout, Color, Device, Extent3d, Operations, Queue,
    RenderPassColorAttachment, SurfaceConfiguration, Texture, TextureView,
};

pub struct ScreenRenderer {
    screen_quad: Box<dyn Primitive>,
}

impl ScreenRenderer {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let mock_material = UnlitColorMaterial::new(device, surface_config);
        let screen_quad = Box::new(Quad::new(&device, Box::new(mock_material)));

        Self { screen_quad }
    }

    pub fn render(
        &self,
        device: &Device,
        queue: &Queue,
        output_view: &TextureView,
        render_texture_material: &PostProcessMaterial,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("screen_render_encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("screen_render_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: output_view,
                resolve_target: None,
                ops: Operations {
                    load: wgpu::LoadOp::Clear(Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // render_pass.set_pipeline(&self.screen_quad.material().render_pipeline());
        render_pass.set_pipeline(&render_texture_material.render_pipeline);
        // let object_uniform = ObjectUniform {
        //     view_proj: [[0.0; 4]; 4], // not used in shader
        //     model: [[0.0; 4]; 4],     // not used in shader
        //     normal1: [0.0; 4],        // not used in shader
        //     normal2: [0.0; 4],        // not used in shader
        //     normal3: [0.0; 4],        // not used in shader
        // };
        // queue.write_buffer(
        //     &self.generic_uniform_data.uniform_buffer,
        //     0,
        //     bytemuck::cast_slice(&[object_uniform]),
        // );
        // queue.write_buffer(
        //     &self.screen_quad.material().uniform_buffer,
        //     0,
        //     self.screen_quad.material().uniform.as_bytes(),
        // );

        // render_pass.set_bind_group(0, &self.screen_quad.material().bind_groups()[0], &[0]);
        // render_pass.set_bind_group(1, &self.screen_quad.material().bind_group, &[]);
        render_pass.set_bind_group(0, &render_texture_material.texture_bind_group, &[]);
        self.screen_quad.draw(&mut render_pass);
        drop(render_pass);

        queue.submit(Some(encoder.finish()));
    }
}

pub struct DynamicTexture {
    pub texture: Texture,
    pub texture_view: TextureView,
    pub size: Extent3d,
    pub bind_group: BindGroup,
}

impl DynamicTexture {
    fn new(device: &Device, width: u32, height: u32, bind_group_layout: &BindGroupLayout) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("dynamic_texture"),
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("dynamic_texture_bind_group"),
        });
        Self {
            texture,
            texture_view,
            size,
            bind_group,
        }
    }

    fn update(&self, queue: &Queue, image_data: &[u8]) {
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            image_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.size.width),
                rows_per_image: Some(self.size.height),
            },
            self.size,
        );
    }
}
