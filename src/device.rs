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

    /// Copy data from buffer to texture
    #[napi]
    pub fn copy_buffer_to_texture(
        &self,
        encoder: &mut GpuCommandEncoder,
        source: &crate::GpuBuffer,
        source_offset: i64,
        bytes_per_row: u32,
        rows_per_image: Option<u32>,
        destination: &crate::GpuTexture,
        mip_level: Option<u32>,
        origin_x: Option<u32>,
        origin_y: Option<u32>,
        origin_z: Option<u32>,
        width: u32,
        height: u32,
        depth: Option<u32>,
    ) -> Result<()> {
        if let Some(ref mut enc) = encoder.encoder {
            enc.copy_buffer_to_texture(
                wgpu::ImageCopyBuffer {
                    buffer: &source.buffer,
                    layout: wgpu::ImageDataLayout {
                        offset: source_offset as u64,
                        bytes_per_row: Some(bytes_per_row),
                        rows_per_image,
                    },
                },
                wgpu::ImageCopyTexture {
                    texture: &destination.texture,
                    mip_level: mip_level.unwrap_or(0),
                    origin: wgpu::Origin3d {
                        x: origin_x.unwrap_or(0),
                        y: origin_y.unwrap_or(0),
                        z: origin_z.unwrap_or(0),
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: depth.unwrap_or(1),
                },
            );
            Ok(())
        } else {
            Err(Error::from_reason("Command encoder already finished"))
        }
    }

    /// Copy data from texture to buffer
    #[napi]
    pub fn copy_texture_to_buffer(
        &self,
        encoder: &mut GpuCommandEncoder,
        source: &crate::GpuTexture,
        mip_level: Option<u32>,
        origin_x: Option<u32>,
        origin_y: Option<u32>,
        origin_z: Option<u32>,
        destination: &crate::GpuBuffer,
        destination_offset: i64,
        bytes_per_row: u32,
        rows_per_image: Option<u32>,
        width: u32,
        height: u32,
        depth: Option<u32>,
    ) -> Result<()> {
        if let Some(ref mut enc) = encoder.encoder {
            enc.copy_texture_to_buffer(
                wgpu::ImageCopyTexture {
                    texture: &source.texture,
                    mip_level: mip_level.unwrap_or(0),
                    origin: wgpu::Origin3d {
                        x: origin_x.unwrap_or(0),
                        y: origin_y.unwrap_or(0),
                        z: origin_z.unwrap_or(0),
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::ImageCopyBuffer {
                    buffer: &destination.buffer,
                    layout: wgpu::ImageDataLayout {
                        offset: destination_offset as u64,
                        bytes_per_row: Some(bytes_per_row),
                        rows_per_image,
                    },
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: depth.unwrap_or(1),
                },
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

    /// Create a bind group with mixed resources (buffers, textures, samplers)
    #[napi]
    pub fn create_bind_group(
        &self,
        label: Option<String>,
        layout: &crate::GpuBindGroupLayout,
        entries: Vec<crate::BindGroupEntry>,
        buffers: Vec<&crate::GpuBuffer>,
        textures: Vec<&crate::GpuTextureView>,
        samplers: Vec<&crate::GpuSampler>,
    ) -> Result<crate::GpuBindGroup> {
        let bind_entries: Vec<_> = entries
            .iter()
            .map(|entry| {
                let resource = match entry.resource_type.as_str() {
                    "buffer" => {
                        let idx = entry.buffer_index.unwrap_or(0) as usize;
                        if idx >= buffers.len() {
                            return Err(Error::from_reason(format!("Buffer index {} out of range", idx)));
                        }
                        wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer: &buffers[idx].buffer,
                            offset: 0,
                            size: None,
                        })
                    }
                    "texture" => {
                        let idx = entry.texture_index.unwrap_or(0) as usize;
                        if idx >= textures.len() {
                            return Err(Error::from_reason(format!("Texture index {} out of range", idx)));
                        }
                        wgpu::BindingResource::TextureView(&textures[idx].view)
                    }
                    "sampler" => {
                        let idx = entry.sampler_index.unwrap_or(0) as usize;
                        if idx >= samplers.len() {
                            return Err(Error::from_reason(format!("Sampler index {} out of range", idx)));
                        }
                        wgpu::BindingResource::Sampler(&samplers[idx].sampler)
                    }
                    _ => {
                        return Err(Error::from_reason(format!("Unknown resource type: {}", entry.resource_type)));
                    }
                };

                Ok(wgpu::BindGroupEntry {
                    binding: entry.binding,
                    resource,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: label.as_deref(),
            layout: &layout.layout,
            entries: &bind_entries,
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
    /// depth_stencil_format: optional depth/stencil format (e.g., "depth24plus")
    /// blend_mode: optional blend mode ("replace", "alpha", "additive", "premultiplied")
    /// write_mask: optional color write mask (0-15, default 15 = all channels)
    /// sample_count: optional MSAA sample count (1, 2, 4, 8, default 1)
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
        depth_stencil_format: Option<String>,
        blend_mode: Option<String>,
        write_mask: Option<u32>,
        sample_count: Option<u32>,
    ) -> Result<crate::GpuRenderPipeline> {
        // Build vertex attributes from formats with proper offsets
        let mut current_offset: u64 = 0;
        let attributes: Vec<wgpu::VertexAttribute> = vertex_formats
            .iter()
            .enumerate()
            .map(|(i, format)| {
                let vertex_format = crate::pipeline::parse_vertex_format(format);
                let size = match vertex_format {
                    wgpu::VertexFormat::Float32 => 4,
                    wgpu::VertexFormat::Float32x2 => 8,
                    wgpu::VertexFormat::Float32x3 => 12,
                    wgpu::VertexFormat::Float32x4 => 16,
                    _ => 4,
                };

                let attr = wgpu::VertexAttribute {
                    format: vertex_format,
                    offset: current_offset,
                    shader_location: i as u32,
                };

                current_offset += size;
                attr
            })
            .collect();

        // Stride is the total size of all attributes
        let stride: u64 = current_offset;

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes,
        };

        // Parse blend mode and write mask
        let blend_state = blend_mode
            .as_ref()
            .map(|mode| parse_blend_mode(mode))
            .unwrap_or(wgpu::BlendState::REPLACE);

        let color_write_mask = write_mask
            .map(|mask| wgpu::ColorWrites::from_bits(mask).unwrap_or(wgpu::ColorWrites::ALL))
            .unwrap_or(wgpu::ColorWrites::ALL);

        // Build fragment targets
        let targets: Vec<Option<wgpu::ColorTargetState>> = fragment_formats
            .iter()
            .map(|format| {
                Some(wgpu::ColorTargetState {
                    format: crate::pipeline::parse_texture_format(format),
                    blend: Some(blend_state),
                    write_mask: color_write_mask,
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

        // Build depth/stencil state if format is provided
        let depth_stencil = depth_stencil_format.as_ref().map(|format_str| {
            wgpu::DepthStencilState {
                format: crate::pipeline::parse_texture_format(format_str),
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }
        });

        // Configure MSAA sample count
        let multisample_state = wgpu::MultisampleState {
            count: sample_count.unwrap_or(1),
            mask: !0,
            alpha_to_coverage_enabled: false,
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
            depth_stencil,
            multisample: multisample_state,
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

fn parse_blend_mode(mode: &str) -> wgpu::BlendState {
    match mode {
        "replace" => wgpu::BlendState::REPLACE,
        "alpha" => wgpu::BlendState::ALPHA_BLENDING,
        "additive" => wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        },
        "premultiplied" => wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING,
        _ => wgpu::BlendState::REPLACE,
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
    /// color_attachments: array of texture views to render to (MSAA textures if using MSAA)
    /// clear_colors: optional array of [r, g, b, a] values for clearing
    /// bind_groups: optional array of bind groups to set
    /// depth_stencil_attachment: optional depth/stencil texture view
    /// clear_depth: optional depth clear value (0.0 to 1.0)
    /// resolve_targets: optional array of texture views to resolve MSAA to (must match color_attachments length)
    #[napi]
    pub fn render_pass(
        &mut self,
        pipeline: &crate::GpuRenderPipeline,
        vertex_buffers: Vec<&crate::GpuBuffer>,
        vertex_count: u32,
        color_attachments: Vec<&crate::GpuTextureView>,
        clear_colors: Option<Vec<Vec<f64>>>,
        bind_groups: Option<Vec<&crate::GpuBindGroup>>,
        depth_stencil_attachment: Option<&crate::GpuTextureView>,
        clear_depth: Option<f64>,
        resolve_targets: Option<Vec<&crate::GpuTextureView>>,
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

                    // Get resolve target if MSAA is being used
                    let resolve_target = resolve_targets.as_ref().and_then(|targets| {
                        if i < targets.len() {
                            Some(&*targets[i].view)
                        } else {
                            None
                        }
                    });

                    Some(wgpu::RenderPassColorAttachment {
                        view: &view.view,
                        resolve_target,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: wgpu::StoreOp::Store,
                        },
                    })
                })
                .collect();

            // Build depth/stencil attachment if provided
            let depth_stencil = depth_stencil_attachment.map(|view| {
                wgpu::RenderPassDepthStencilAttachment {
                    view: &view.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_depth.unwrap_or(1.0) as f32),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }
            });

            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &attachments,
                depth_stencil_attachment: depth_stencil,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&pipeline.pipeline);

            // Set bind groups
            if let Some(groups) = bind_groups {
                for (index, group) in groups.iter().enumerate() {
                    pass.set_bind_group(index as u32, &group.bind_group, &[]);
                }
            }

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

    /// Execute a render pass with indexed drawing
    #[napi]
    pub fn render_pass_indexed(
        &mut self,
        pipeline: &crate::GpuRenderPipeline,
        vertex_buffers: Vec<&crate::GpuBuffer>,
        index_buffer: &crate::GpuBuffer,
        index_format: String,
        index_count: u32,
        color_attachments: Vec<&crate::GpuTextureView>,
        clear_colors: Option<Vec<Vec<f64>>>,
        bind_groups: Option<Vec<&crate::GpuBindGroup>>,
        depth_stencil_attachment: Option<&crate::GpuTextureView>,
        clear_depth: Option<f64>,
        resolve_targets: Option<Vec<&crate::GpuTextureView>>,
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

                    // Get resolve target if MSAA is being used
                    let resolve_target = resolve_targets.as_ref().and_then(|targets| {
                        if i < targets.len() {
                            Some(&*targets[i].view)
                        } else {
                            None
                        }
                    });

                    Some(wgpu::RenderPassColorAttachment {
                        view: &view.view,
                        resolve_target,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: wgpu::StoreOp::Store,
                        },
                    })
                })
                .collect();

            // Build depth/stencil attachment if provided
            let depth_stencil = depth_stencil_attachment.map(|view| {
                wgpu::RenderPassDepthStencilAttachment {
                    view: &view.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_depth.unwrap_or(1.0) as f32),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }
            });

            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &attachments,
                depth_stencil_attachment: depth_stencil,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&pipeline.pipeline);

            // Set bind groups
            if let Some(groups) = bind_groups {
                for (index, group) in groups.iter().enumerate() {
                    pass.set_bind_group(index as u32, &group.bind_group, &[]);
                }
            }

            // Set vertex buffers
            for (index, buffer) in vertex_buffers.iter().enumerate() {
                pass.set_vertex_buffer(index as u32, buffer.buffer.slice(..));
            }

            // Set index buffer
            let idx_format = match index_format.as_str() {
                "uint32" => wgpu::IndexFormat::Uint32,
                _ => wgpu::IndexFormat::Uint16,
            };
            pass.set_index_buffer(index_buffer.buffer.slice(..), idx_format);

            // Draw indexed
            pass.draw_indexed(0..index_count, 0, 0..1);

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
