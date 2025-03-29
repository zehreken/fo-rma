use wgpu::{BindGroup, BindGroupLayout, ComputePipeline, Device, Queue, Texture, TextureView};

pub struct PostProcessor {
    compute_pipeline: ComputePipeline,
    pub bind_group: BindGroup,
}

impl PostProcessor {
    pub fn new(device: &Device, write_view: &TextureView, read_view: &TextureView) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compute_shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../shaders/compute/noise.comp.wgsl").into(),
            ),
        });
        let (layout, bind_group) = create_bind_group(device, write_view, read_view);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("post_process_pipeline_layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("post_process_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "cs_main",
        });

        Self {
            compute_pipeline,
            bind_group,
        }
    }

    pub fn run(&self, device: &Device, queue: &Queue, width: u32, height: u32) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("post_process_encoder"),
        });

        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("post_process_compute"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(&self.compute_pipeline);
        compute_pass.set_bind_group(0, &self.bind_group, &[]);
        compute_pass.dispatch_workgroups((width + 7) / 8, (height + 7) / 8, 1);

        drop(compute_pass);

        queue.submit(Some(encoder.finish()));
    }

    pub fn resize(&mut self, device: &Device, write_view: &TextureView, read_view: &TextureView) {
        self.bind_group = create_bind_group(device, write_view, read_view).1;
    }
}

fn create_bind_group(
    device: &Device,
    write_view: &TextureView,
    read_view: &TextureView,
) -> (BindGroupLayout, BindGroup) {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("post_process_bind_group_layout"),
        entries: &[
            // writable image
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::WriteOnly,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            },
            // sampled image (optional)
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("post_process_bind_group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(write_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(read_view),
            },
        ],
    });

    (bind_group_layout, bind_group)
}
