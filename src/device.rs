use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;

#[napi]
pub struct GpuDevice {
    pub(crate) device: Arc<wgpu::Device>,
    pub(crate) queue: Arc<wgpu::Queue>,
}

impl GpuDevice {
    pub(crate) fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self {
            device: Arc::new(device),
            queue: Arc::new(queue),
        }
    }
}

#[napi]
impl GpuDevice {
    /// Create a GPU buffer
    #[napi]
    pub fn create_buffer(&self, size: u32, usage: u32, mapped_at_creation: Option<bool>) -> crate::GpuBuffer {
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: size as u64,
            usage: wgpu::BufferUsages::from_bits_truncate(usage),
            mapped_at_creation: mapped_at_creation.unwrap_or(false),
        });

        crate::GpuBuffer::new(buffer, self.device.clone())
    }

    /// Create a shader module
    #[napi]
    pub fn create_shader_module(&self, code: String) -> Result<GpuShaderModule> {
        let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(code.into()),
        });

        Ok(GpuShaderModule { shader })
    }

    /// Create a command encoder
    #[napi]
    pub fn create_command_encoder(&self) -> GpuCommandEncoder {
        let encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        GpuCommandEncoder {
            encoder: Some(encoder),
        }
    }

    /// Submit commands to the queue
    /// Note: This consumes the command buffer
    #[napi]
    pub fn queue_submit(&self, command_buffer: &mut GpuCommandBuffer) {
        if let Some(buffer) = command_buffer.buffer.take() {
            self.queue.submit(std::iter::once(buffer));
        }
    }

    /// Poll the device
    #[napi]
    pub fn poll(&self, force_wait: Option<bool>) {
        self.device.poll(if force_wait.unwrap_or(false) {
            wgpu::Maintain::Wait
        } else {
            wgpu::Maintain::Poll
        });
    }

    /// Write data to a buffer using the queue
    #[napi]
    pub fn queue_write_buffer(&self, buffer: &crate::GpuBuffer, offset: i64, data: Buffer) {
        self.queue.write_buffer(&buffer.buffer, offset as u64, &data);
    }

    /// Copy data from one buffer to another
    #[napi]
    pub fn copy_buffer_to_buffer(
        &self,
        encoder: &mut GpuCommandEncoder,
        source: &crate::GpuBuffer,
        source_offset: i64,
        destination: &crate::GpuBuffer,
        destination_offset: i64,
        size: i64,
    ) -> Result<()> {
        if let Some(ref mut enc) = encoder.encoder {
            enc.copy_buffer_to_buffer(
                &source.buffer,
                source_offset as u64,
                &destination.buffer,
                destination_offset as u64,
                size as u64,
            );
            Ok(())
        } else {
            Err(Error::from_reason("Command encoder already finished"))
        }
    }

    /// Create a texture
    #[napi]
    pub fn create_texture(&self, descriptor: crate::TextureDescriptor) -> crate::GpuTexture {
        let format = crate::pipeline::parse_texture_format(&descriptor.format);
        let dimension = match descriptor.dimension.as_deref() {
            Some("1d") => wgpu::TextureDimension::D1,
            Some("3d") => wgpu::TextureDimension::D3,
            _ => wgpu::TextureDimension::D2,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: descriptor.label.as_deref(),
            size: wgpu::Extent3d {
                width: descriptor.width,
                height: descriptor.height,
                depth_or_array_layers: descriptor.depth.unwrap_or(1),
            },
            mip_level_count: descriptor.mip_level_count.unwrap_or(1),
            sample_count: descriptor.sample_count.unwrap_or(1),
            dimension,
            format,
            usage: wgpu::TextureUsages::from_bits_truncate(descriptor.usage),
            view_formats: &[],
        });

        crate::GpuTexture::new(texture)
    }

    /// Create a sampler
    #[napi]
    pub fn create_sampler(&self, descriptor: crate::SamplerDescriptor) -> crate::GpuSampler {
        let sampler = crate::sampler::create_sampler(&self.device, &descriptor);
        crate::GpuSampler::new(sampler)
    }

    /// Create a bind group layout
    #[napi]
    pub fn create_bind_group_layout(&self, descriptor: crate::BindGroupLayoutDescriptor) -> Result<crate::GpuBindGroupLayout> {
        let entries: Vec<_> = descriptor.entries
            .iter()
            .map(|e| crate::bind_group::convert_bind_group_layout_entry(e))
            .collect();

        let layout = self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: descriptor.label.as_deref(),
            entries: &entries,
        });

        Ok(crate::GpuBindGroupLayout::new(layout))
    }

    /// Create a bind group with buffer bindings
    /// Bindings are specified as: binding index, buffer, offset, size
    /// All buffers are bound sequentially starting from binding 0
    #[napi]
    pub fn create_bind_group_buffers(
        &self,
        label: Option<String>,
        layout: &crate::GpuBindGroupLayout,
        buffers: Vec<&crate::GpuBuffer>,
    ) -> Result<crate::GpuBindGroup> {
        let entries: Vec<_> = buffers
            .iter()
            .enumerate()
            .map(|(index, buffer)| {
                wgpu::BindGroupEntry {
                    binding: index as u32,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &buffer.buffer,
                        offset: 0,
                        size: None,
                    }),
                }
            })
            .collect();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: label.as_deref(),
            layout: &layout.layout,
            entries: &entries,
        });

        Ok(crate::GpuBindGroup::new(bind_group))
    }

    /// Create a pipeline layout
    #[napi]
    pub fn create_pipeline_layout(
        &self,
        label: Option<String>,
        bind_group_layouts: Vec<&crate::GpuBindGroupLayout>,
    ) -> crate::GpuPipelineLayout {
        let bind_group_layouts_refs: Vec<_> = bind_group_layouts
            .iter()
            .map(|l| l.layout.as_ref())
            .collect();

        let layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: label.as_deref(),
            bind_group_layouts: &bind_group_layouts_refs,
            push_constant_ranges: &[],
        });

        crate::GpuPipelineLayout {
            layout: std::sync::Arc::new(layout),
        }
    }

    /// Create a compute pipeline
    #[napi]
    pub fn create_compute_pipeline(
        &self,
        label: Option<String>,
        layout: Option<&crate::GpuPipelineLayout>,
        shader_module: &GpuShaderModule,
        entry_point: String,
    ) -> crate::GpuComputePipeline {
        let layout_ref = layout.map(|l| l.layout.as_ref());

        let pipeline = self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: label.as_deref(),
            layout: layout_ref,
            module: &shader_module.shader,
            entry_point: &entry_point,
        });

        crate::GpuComputePipeline {
            pipeline: std::sync::Arc::new(pipeline),
        }
    }

    /// Create a render pipeline (simplified)
    /// vertex_formats: array of format strings for vertex attributes
    /// fragment_targets: array of format strings for color targets
    #[napi]
    pub fn create_render_pipeline(
        &self,
        label: Option<String>,
        layout: Option<&crate::GpuPipelineLayout>,
        vertex_shader: &GpuShaderModule,
        vertex_entry_point: String,
        vertex_formats: Vec<String>,
        fragment_shader: Option<&GpuShaderModule>,
        fragment_entry_point: Option<String>,
        fragment_formats: Vec<String>,
    ) -> Result<crate::GpuRenderPipeline> {
        // Build vertex attributes from formats
        let attributes: Vec<wgpu::VertexAttribute> = vertex_formats
            .iter()
            .enumerate()
            .map(|(i, format)| wgpu::VertexAttribute {
                format: crate::pipeline::parse_vertex_format(format),
                offset: 0, // Will be calculated properly later
                shader_location: i as u32,
            })
            .collect();

        // Calculate stride
        let stride: u64 = attributes
            .iter()
            .map(|attr| match attr.format {
                wgpu::VertexFormat::Float32 => 4,
                wgpu::VertexFormat::Float32x2 => 8,
                wgpu::VertexFormat::Float32x3 => 12,
                wgpu::VertexFormat::Float32x4 => 16,
                _ => 4,
            })
            .sum();

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes,
        };

        // Build fragment targets
        let targets: Vec<Option<wgpu::ColorTargetState>> = fragment_formats
            .iter()
            .map(|format| {
                Some(wgpu::ColorTargetState {
                    format: crate::pipeline::parse_texture_format(format),
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })
            })
            .collect();

        let fragment = if let Some(shader) = fragment_shader {
            Some(wgpu::FragmentState {
                module: &shader.shader,
                entry_point: &fragment_entry_point.unwrap_or_else(|| "main".to_string()),
                targets: &targets,
            })
        } else {
            None
        };

        let pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: label.as_deref(),
            layout: layout.map(|l| l.layout.as_ref()),
            vertex: wgpu::VertexState {
                module: &vertex_shader.shader,
                entry_point: &vertex_entry_point,
                buffers: &[vertex_buffer_layout],
            },
            fragment,
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Ok(crate::GpuRenderPipeline {
            pipeline: std::sync::Arc::new(pipeline),
        })
    }

    /// Destroy the device
    #[napi]
    pub fn destroy(&self) {
        // wgpu devices are automatically cleaned up
    }
}

// Helper functions for parsing blend modes
fn parse_blend_factor(factor: &str) -> wgpu::BlendFactor {
    match factor {
        "zero" => wgpu::BlendFactor::Zero,
        "one" => wgpu::BlendFactor::One,
        "src" => wgpu::BlendFactor::Src,
        "one-minus-src" => wgpu::BlendFactor::OneMinusSrc,
        "src-alpha" => wgpu::BlendFactor::SrcAlpha,
        "one-minus-src-alpha" => wgpu::BlendFactor::OneMinusSrcAlpha,
        "dst" => wgpu::BlendFactor::Dst,
        "one-minus-dst" => wgpu::BlendFactor::OneMinusDst,
        "dst-alpha" => wgpu::BlendFactor::DstAlpha,
        "one-minus-dst-alpha" => wgpu::BlendFactor::OneMinusDstAlpha,
        _ => wgpu::BlendFactor::One,
    }
}

fn parse_blend_operation(op: &str) -> wgpu::BlendOperation {
    match op {
        "add" => wgpu::BlendOperation::Add,
        "subtract" => wgpu::BlendOperation::Subtract,
        "reverse-subtract" => wgpu::BlendOperation::ReverseSubtract,
        "min" => wgpu::BlendOperation::Min,
        "max" => wgpu::BlendOperation::Max,
        _ => wgpu::BlendOperation::Add,
    }
}

#[napi]
pub struct GpuShaderModule {
    pub(crate) shader: wgpu::ShaderModule,
}

#[napi]
pub struct GpuCommandEncoder {
    pub(crate) encoder: Option<wgpu::CommandEncoder>,
}

#[napi]
impl GpuCommandEncoder {
    /// Begin a compute pass and execute it with the given pipeline and bind groups
    #[napi]
    pub fn compute_pass(
        &mut self,
        pipeline: &crate::GpuComputePipeline,
        bind_groups: Vec<&crate::GpuBindGroup>,
        workgroups_x: u32,
        workgroups_y: Option<u32>,
        workgroups_z: Option<u32>,
    ) -> Result<()> {
        if let Some(ref mut enc) = self.encoder {
            let mut pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });

            pass.set_pipeline(&pipeline.pipeline);

            for (index, bind_group) in bind_groups.iter().enumerate() {
                pass.set_bind_group(index as u32, &bind_group.bind_group, &[]);
            }

            pass.dispatch_workgroups(
                workgroups_x,
                workgroups_y.unwrap_or(1),
                workgroups_z.unwrap_or(1),
            );

            drop(pass);
            Ok(())
        } else {
            Err(Error::from_reason("Command encoder already finished"))
        }
    }

    /// Execute a render pass (simplified inline execution)
    /// color_attachments: array of texture views to render to
    /// clear_colors: optional array of [r, g, b, a] values for clearing
    #[napi]
    pub fn render_pass(
        &mut self,
        pipeline: &crate::GpuRenderPipeline,
        vertex_buffers: Vec<&crate::GpuBuffer>,
        vertex_count: u32,
        color_attachments: Vec<&crate::GpuTextureView>,
        clear_colors: Option<Vec<Vec<f64>>>,
    ) -> Result<()> {
        if let Some(ref mut enc) = self.encoder {
            // Build color attachments
            let attachments: Vec<_> = color_attachments
                .iter()
                .enumerate()
                .map(|(i, view)| {
                    let clear_color = if let Some(ref colors) = clear_colors {
                        if i < colors.len() && colors[i].len() >= 4 {
                            wgpu::Color {
                                r: colors[i][0],
                                g: colors[i][1],
                                b: colors[i][2],
                                a: colors[i][3],
                            }
                        } else {
                            wgpu::Color::BLACK
                        }
                    } else {
                        wgpu::Color::BLACK
                    };

                    Some(wgpu::RenderPassColorAttachment {
                        view: &view.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: wgpu::StoreOp::Store,
                        },
                    })
                })
                .collect();

            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &attachments,
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&pipeline.pipeline);

            // Set vertex buffers
            for (index, buffer) in vertex_buffers.iter().enumerate() {
                pass.set_vertex_buffer(index as u32, buffer.buffer.slice(..));
            }

            pass.draw(0..vertex_count, 0..1);

            drop(pass);
            Ok(())
        } else {
            Err(Error::from_reason("Command encoder already finished"))
        }
    }

    /// Finish encoding and return a command buffer
    #[napi]
    pub fn finish(&mut self) -> GpuCommandBuffer {
        let buffer = self.encoder.take().map(|e| e.finish());
        GpuCommandBuffer { buffer }
    }
}

#[napi]
pub struct GpuCommandBuffer {
    buffer: Option<wgpu::CommandBuffer>,
}
