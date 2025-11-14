use crate::{
    basics::uniforms::ColorUniform,
    rendering_utils::create_post_process_texture,
    shader_utils::{self, effect_to_name, Effect},
};
use std::{mem, time::Instant};
use wgpu::{
    naga::FastIndexMap, BindGroup, BindGroupLayout, Buffer, ComputePipeline, Device, Queue,
    ShaderModule, TextureView,
};
use winit::dpi::PhysicalSize;

struct EffectConfig {
    effect: Effect,
    bind_group: BindGroup,
}

impl EffectConfig {
    fn new(
        device: &Device,
        write_view: &TextureView,
        read_view: &TextureView,
        effect: Effect,
    ) -> Self {
        let (_layout, bind_group) = create_bind_group(device, write_view, read_view);

        Self { effect, bind_group }
    }
}

pub struct PostProcessor {
    pub control_uniform_buffer: Buffer,
    pub control_bg: BindGroup,
    pub instant: Instant,
    effects: Vec<EffectConfig>,
    intermediate_texture_view_1: TextureView,
    intermediate_texture_view_2: TextureView,
    compiled_shaders: FastIndexMap<Effect, ShaderModule>,
    compiled_pipelines: FastIndexMap<Effect, (BindGroupLayout, ComputePipeline)>,
}

impl PostProcessor {
    pub fn new(
        device: &Device,
        size: PhysicalSize<u32>,
        write_view: &TextureView,
        read_view: &TextureView,
    ) -> Self {
        let (control_uniform_buffer, control_bgl, control_bg) = create_control_bind_group(device);

        // Compiling shaders at start
        let mut compiled_shaders = FastIndexMap::default();
        for (effect, source) in shader_utils::EFFECTS.iter() {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(effect_to_name(*effect)),
                source: source.clone(),
            });
            compiled_shaders.insert(*effect, shader);
        }

        let (_intermediate_texture_1, intermediate_texture_view_1) =
            create_post_process_texture(device, size);
        let (_intermediate_texture_2, intermediate_texture_view_2) =
            create_post_process_texture(device, size);

        let mut compiled_pipelines = FastIndexMap::default();
        for (effect, shader) in compiled_shaders.iter() {
            let (layout, _bind_group) = create_bind_group(device, write_view, read_view);

            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&format!(
                    "post_process_pipeline_layout_{}",
                    effect_to_name(*effect)
                )),
                bind_group_layouts: &[&layout, &control_bgl],
                push_constant_ranges: &[],
            });

            let compute_pipeline =
                device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("post_process_pipeline"),
                    layout: Some(&pipeline_layout),
                    module: &shader,
                    entry_point: "cs_main",
                });
            compiled_pipelines.insert(*effect, (layout, compute_pipeline));
        }

        Self {
            control_uniform_buffer,
            control_bg,
            instant: Instant::now(),
            effects: vec![],
            intermediate_texture_view_1,
            intermediate_texture_view_2,
            compiled_shaders,
            compiled_pipelines,
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

        for effect in &self.effects {
            compute_pass.set_pipeline(&self.compiled_pipelines[&effect.effect].1);
            compute_pass.set_bind_group(0, &effect.bind_group, &[]);
            let control_uniform = ColorUniform {
                color: [self.instant.elapsed().as_secs_f32(), 0.0, 0.0, 0.0],
            };
            queue.write_buffer(
                &self.control_uniform_buffer,
                0,
                bytemuck::cast_slice(&[control_uniform]),
            );
            compute_pass.set_bind_group(1, &self.control_bg, &[]);
            compute_pass.dispatch_workgroups((width + 7) / 8, (height + 7) / 8, 1);
        }

        drop(compute_pass);

        queue.submit(Some(encoder.finish()));
    }

    pub fn update_effects(
        &mut self,
        device: &Device,
        write_view: &TextureView,
        read_view: &TextureView,
        effect_to_active: &FastIndexMap<Effect, bool>,
    ) {
        let mut active_effects: Vec<(&Effect, &ShaderModule)> = vec![];
        for (effect, active) in effect_to_active {
            if *active {
                active_effects.push((effect, &self.compiled_shaders[effect]));
            }
        }

        let mut effects_final = vec![];
        for (index, effect_data) in active_effects.iter().enumerate() {
            let effect = if index == 0 {
                EffectConfig::new(
                    device,
                    if active_effects.len() == 1 {
                        write_view
                    } else {
                        &self.intermediate_texture_view_1
                    },
                    read_view,
                    effect_data.0.to_owned(),
                )
            } else if index == active_effects.len() - 1 {
                if index % 2 == 1 {
                    EffectConfig::new(
                        device,
                        write_view,
                        &self.intermediate_texture_view_1,
                        effect_data.0.to_owned(),
                    )
                } else {
                    EffectConfig::new(
                        device,
                        write_view,
                        &self.intermediate_texture_view_2,
                        effect_data.0.to_owned(),
                    )
                }
            } else {
                if index % 2 == 1 {
                    EffectConfig::new(
                        device,
                        &self.intermediate_texture_view_2,
                        &self.intermediate_texture_view_1,
                        effect_data.0.to_owned(),
                    )
                } else {
                    EffectConfig::new(
                        device,
                        &self.intermediate_texture_view_1,
                        &self.intermediate_texture_view_2,
                        effect_data.0.to_owned(),
                    )
                }
            };
            effects_final.push(effect);
        }

        self.effects = effects_final;
    }

    pub fn resize(
        &mut self,
        device: &Device,
        size: PhysicalSize<u32>,
        write_view: &TextureView,
        read_view: &TextureView,
        effect_to_active: &FastIndexMap<Effect, bool>,
    ) {
        (_, self.intermediate_texture_view_1) = create_post_process_texture(device, size);
        (_, self.intermediate_texture_view_2) = create_post_process_texture(device, size);

        let mut active_effects: Vec<(&Effect, &ShaderModule)> = vec![];
        for (effect, active) in effect_to_active {
            if *active {
                active_effects.push((effect, &self.compiled_shaders[effect]));
            }
        }

        let mut effects_final = vec![];
        for (index, effect_data) in active_effects.iter().enumerate() {
            let effect = if index == 0 {
                EffectConfig::new(
                    device,
                    if active_effects.len() == 1 {
                        write_view
                    } else {
                        &self.intermediate_texture_view_1
                    },
                    read_view,
                    effect_data.0.to_owned(),
                )
            } else if index == active_effects.len() - 1 {
                if index % 2 == 1 {
                    EffectConfig::new(
                        device,
                        write_view,
                        &self.intermediate_texture_view_1,
                        effect_data.0.to_owned(),
                    )
                } else {
                    EffectConfig::new(
                        device,
                        write_view,
                        &self.intermediate_texture_view_2,
                        effect_data.0.to_owned(),
                    )
                }
            } else {
                if index % 2 == 1 {
                    EffectConfig::new(
                        device,
                        &self.intermediate_texture_view_2,
                        &self.intermediate_texture_view_1,
                        effect_data.0.to_owned(),
                    )
                } else {
                    EffectConfig::new(
                        device,
                        &self.intermediate_texture_view_1,
                        &self.intermediate_texture_view_2,
                        effect_data.0.to_owned(),
                    )
                }
            };
            effects_final.push(effect);
        }

        self.effects = effects_final;
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

fn create_control_bind_group(device: &Device) -> (Buffer, BindGroupLayout, BindGroup) {
    let control_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("control_uniform_buffer"),
        size: mem::size_of::<ColorUniform>() as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let control_uniform_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("control_uniform_bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    let control_uniform_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("control_uniform_bind_group"),
        layout: &control_uniform_bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: control_uniform_buffer.as_entire_binding(),
        }],
    });

    (
        control_uniform_buffer,
        control_uniform_bgl,
        control_uniform_bg,
    )
}
