use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;

#[napi]
pub struct GpuDevice {
    pub(crate) device: Arc<wgpu::Device>,
    pub(crate) queue_internal: Arc<wgpu::Queue>,
    features: crate::GpuSupportedFeatures,
    limits: crate::GpuSupportedLimits,
}

impl GpuDevice {
    pub(crate) fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        let features = crate::GpuSupportedFeatures {
            features: device.features(),
        };
        let limits = crate::GpuSupportedLimits::from_wgpu(&device.limits());

        Self {
            device: Arc::new(device),
            queue_internal: Arc::new(queue),
            features,
            limits,
        }
    }
}

#[napi]
impl GpuDevice {
    /// Get the queue for this device (WebGPU standard property)
    #[napi(getter)]
    pub fn queue(&self) -> crate::GpuQueue {
        crate::GpuQueue::new(self.queue_internal.clone())
    }

    /// Get the supported features for this device (WebGPU standard property)
    #[napi(getter)]
    pub fn features(&self) -> crate::GpuSupportedFeatures {
        crate::GpuSupportedFeatures {
            features: self.features.features,
        }
    }

    /// Get the supported limits for this device (WebGPU standard property)
    #[napi(getter)]
    pub fn limits(&self) -> crate::GpuSupportedLimits {
        self.limits.clone()
    }

    /// Get the label of this device (WebGPU standard property)
    #[napi(getter)]
    pub fn label(&self) -> Option<String> {
        None // wgpu doesn't expose device labels after creation
    }

    /// Create a GPU buffer
    #[napi(js_name = "createBuffer")]
    pub fn create_buffer(&self, descriptor: crate::BufferDescriptor) -> crate::GpuBuffer {
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: descriptor.label.as_deref(),
            size: descriptor.size as u64,
            usage: wgpu::BufferUsages::from_bits_truncate(descriptor.usage),
            mapped_at_creation: descriptor.mapped_at_creation.unwrap_or(false),
        });

        crate::GpuBuffer::new(buffer, self.device.clone())
    }

    /// Create a shader module
    #[napi(js_name = "createShaderModule")]
    pub fn create_shader_module(&self, descriptor: crate::ShaderModuleDescriptor) -> Result<GpuShaderModule> {
        let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: descriptor.label.as_deref(),
            source: wgpu::ShaderSource::Wgsl(descriptor.code.into()),
        });

        Ok(GpuShaderModule { shader })
    }

    /// Create a command encoder
    #[napi(js_name = "createCommandEncoder")]
    pub fn create_command_encoder(&self, descriptor: Option<crate::CommandEncoderDescriptor>) -> GpuCommandEncoder {
        let encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: descriptor.as_ref().and_then(|d| d.label.as_deref()),
        });

        GpuCommandEncoder {
            encoder: Some(encoder),
        }
    }

    /// Submit commands to the queue (deprecated - use device.queue.submit() instead)
    /// Note: This consumes the command buffer
    #[napi]
    pub fn queue_submit(&self, command_buffer: &mut GpuCommandBuffer) {
        if let Some(buffer) = command_buffer.buffer.take() {
            self.queue_internal.submit(std::iter::once(buffer));
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

    /// Write data to a buffer using the queue (deprecated - use device.queue.writeBuffer() instead)
    #[napi]
    pub fn queue_write_buffer(&self, buffer: &crate::GpuBuffer, offset: i64, data: Buffer) {
        self.queue_internal.write_buffer(&buffer.buffer, offset as u64, &data);
    }

    /// Copy data from one buffer to another (deprecated - use encoder.copyBufferToBuffer() instead)
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

    /// Copy data from buffer to texture (deprecated - use encoder.copyBufferToTexture() instead)
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

    /// Copy data from texture to buffer (deprecated - use encoder.copyTextureToBuffer() instead)
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
    #[napi(js_name = "createTexture")]
    pub fn create_texture(&self, descriptor: crate::TextureDescriptor) -> crate::GpuTexture {
        let format = crate::parse::parse_texture_format(&descriptor.format);
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
    #[napi(js_name = "createSampler")]
    pub fn create_sampler(&self, descriptor: crate::SamplerDescriptor) -> crate::GpuSampler {
        let sampler = crate::sampler::create_sampler(&self.device, &descriptor);
        crate::GpuSampler::new(sampler)
    }

    /// Create a query set for timestamp or occlusion queries
    #[napi(js_name = "createQuerySet")]
    pub fn create_query_set(&self, descriptor: crate::QuerySetDescriptor) -> Result<crate::GpuQuerySet> {
        let ty = match descriptor.query_type.as_str() {
            "timestamp" => wgpu::QueryType::Timestamp,
            "occlusion" => wgpu::QueryType::Occlusion,
            _ => return Err(Error::from_reason(format!("Invalid query type: {}", descriptor.query_type))),
        };

        let query_set = self.device.create_query_set(&wgpu::QuerySetDescriptor {
            label: descriptor.label.as_deref(),
            ty,
            count: descriptor.count,
        });

        Ok(crate::GpuQuerySet::new(query_set))
    }

    /// Create a bind group layout
    #[napi(js_name = "createBindGroupLayout")]
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

    /// Create a bind group with buffer bindings following WebGPU spec
    #[napi(js_name = "createBindGroup")]
    pub fn create_bind_group_buffers(
        &self,
        descriptor: crate::BindGroupDescriptor,
        buffer_entries: Vec<crate::BindGroupEntryBuffer>,
    ) -> Result<crate::GpuBindGroup> {
        let entries: Vec<_> = buffer_entries
            .iter()
            .map(|entry| {
                wgpu::BindGroupEntry {
                    binding: entry.binding,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &entry.buffer.buffer,
                        offset: entry.offset.unwrap_or(0) as u64,
                        size: entry.size.map(|s| std::num::NonZeroU64::new(s as u64)).flatten(),
                    }),
                }
            })
            .collect();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: descriptor.label.as_deref(),
            layout: &descriptor.layout.layout,
            entries: &entries,
        });

        Ok(crate::GpuBindGroup::new(bind_group))
    }

    /// Create a bind group with texture bindings
    #[napi(js_name = "createBindGroupTextures")]
    pub fn create_bind_group_textures(
        &self,
        descriptor: crate::BindGroupDescriptor,
        texture_entries: Vec<crate::BindGroupEntryTexture>,
    ) -> Result<crate::GpuBindGroup> {
        let entries: Vec<_> = texture_entries
            .iter()
            .map(|entry| {
                wgpu::BindGroupEntry {
                    binding: entry.binding,
                    resource: wgpu::BindingResource::TextureView(&entry.view.view),
                }
            })
            .collect();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: descriptor.label.as_deref(),
            layout: &descriptor.layout.layout,
            entries: &entries,
        });

        Ok(crate::GpuBindGroup::new(bind_group))
    }

    /// Create a bind group with sampler bindings
    #[napi(js_name = "createBindGroupSamplers")]
    pub fn create_bind_group_samplers(
        &self,
        descriptor: crate::BindGroupDescriptor,
        sampler_entries: Vec<crate::BindGroupEntrySampler>,
    ) -> Result<crate::GpuBindGroup> {
        let entries: Vec<_> = sampler_entries
            .iter()
            .map(|entry| {
                wgpu::BindGroupEntry {
                    binding: entry.binding,
                    resource: wgpu::BindingResource::Sampler(&entry.sampler.sampler),
                }
            })
            .collect();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: descriptor.label.as_deref(),
            layout: &descriptor.layout.layout,
            entries: &entries,
        });

        Ok(crate::GpuBindGroup::new(bind_group))
    }

    /// Create a pipeline layout
    #[napi(js_name = "createPipelineLayout")]
    pub fn create_pipeline_layout(
        &self,
        descriptor: crate::PipelineLayoutDescriptor,
    ) -> crate::GpuPipelineLayout {
        let bind_group_layouts_refs: Vec<_> = descriptor.bind_group_layouts
            .iter()
            .map(|l| l.layout.as_ref())
            .collect();

        let layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: descriptor.label.as_deref(),
            bind_group_layouts: &bind_group_layouts_refs,
            push_constant_ranges: &[],
        });

        crate::GpuPipelineLayout {
            layout: std::sync::Arc::new(layout),
        }
    }

    /// Create a compute pipeline following WebGPU spec
    #[napi(js_name = "createComputePipeline")]
    pub fn create_compute_pipeline(
        &self,
        descriptor: crate::ComputePipelineDescriptor,
    ) -> crate::GpuComputePipeline {
        let layout_ref = descriptor.layout.as_ref().map(|l| l.layout.as_ref());

        let pipeline = self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: descriptor.label.as_deref(),
            layout: layout_ref,
            module: &descriptor.compute.module.shader,
            entry_point: &descriptor.compute.entry_point,
        });

        crate::GpuComputePipeline {
            pipeline: std::sync::Arc::new(pipeline),
        }
    }

    /// Create a render pipeline following WebGPU spec
    #[napi(js_name = "createRenderPipeline")]
    pub fn create_render_pipeline(
        &self,
        descriptor: crate::RenderPipelineDescriptor,
    ) -> Result<crate::GpuRenderPipeline> {
        let layout_ref = descriptor.layout.as_ref().map(|l| l.layout.as_ref());
        // Build vertex attributes - need to own them
        let vertex_attributes: Vec<Vec<wgpu::VertexAttribute>> = if let Some(ref buffers) = descriptor.vertex.buffers {
            buffers.iter().map(|buf| {
                buf.attributes.iter().map(|attr| {
                    wgpu::VertexAttribute {
                        format: crate::parse::parse_vertex_format(&attr.format),
                        offset: attr.offset as u64,
                        shader_location: attr.shader_location,
                    }
                }).collect()
            }).collect()
        } else {
            vec![]
        };

        // Build vertex buffer layouts
        let vertex_buffers: Vec<wgpu::VertexBufferLayout> = if let Some(ref buffers) = descriptor.vertex.buffers {
            buffers.iter().enumerate().map(|(i, buf)| {
                let step_mode = match buf.step_mode.as_deref() {
                    Some("instance") => wgpu::VertexStepMode::Instance,
                    _ => wgpu::VertexStepMode::Vertex,
                };

                wgpu::VertexBufferLayout {
                    array_stride: buf.array_stride as u64,
                    step_mode,
                    attributes: &vertex_attributes[i],
                }
            }).collect()
        } else {
            vec![]
        };

        // Build primitive state
        let primitive = if let Some(ref prim) = descriptor.primitive {
            let topology = match prim.topology.as_deref() {
                Some("point-list") => wgpu::PrimitiveTopology::PointList,
                Some("line-list") => wgpu::PrimitiveTopology::LineList,
                Some("line-strip") => wgpu::PrimitiveTopology::LineStrip,
                Some("triangle-strip") => wgpu::PrimitiveTopology::TriangleStrip,
                _ => wgpu::PrimitiveTopology::TriangleList,
            };

            let front_face = match prim.front_face.as_deref() {
                Some("cw") => wgpu::FrontFace::Cw,
                _ => wgpu::FrontFace::Ccw,
            };

            let cull_mode = match prim.cull_mode.as_deref() {
                Some("front") => Some(wgpu::Face::Front),
                Some("back") => Some(wgpu::Face::Back),
                _ => None,
            };

            wgpu::PrimitiveState {
                topology,
                strip_index_format: None,
                front_face,
                cull_mode,
                ..Default::default()
            }
        } else {
            wgpu::PrimitiveState::default()
        };

        // Build depth/stencil state
        let depth_stencil = descriptor.depth_stencil.as_ref().map(|ds| {
            let compare = match ds.depth_compare.as_deref() {
                Some("never") => wgpu::CompareFunction::Never,
                Some("less") => wgpu::CompareFunction::Less,
                Some("equal") => wgpu::CompareFunction::Equal,
                Some("less-equal") => wgpu::CompareFunction::LessEqual,
                Some("greater") => wgpu::CompareFunction::Greater,
                Some("not-equal") => wgpu::CompareFunction::NotEqual,
                Some("greater-equal") => wgpu::CompareFunction::GreaterEqual,
                Some("always") => wgpu::CompareFunction::Always,
                _ => wgpu::CompareFunction::Less,
            };

            wgpu::DepthStencilState {
                format: crate::parse::parse_texture_format(&ds.format),
                depth_write_enabled: ds.depth_write_enabled.unwrap_or(true),
                depth_compare: compare,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }
        });

        // Build multisample state
        let multisample = if let Some(ref ms) = descriptor.multisample {
            wgpu::MultisampleState {
                count: ms.count.unwrap_or(1),
                mask: ms.mask.map(|m| m as u64).unwrap_or(!0),
                alpha_to_coverage_enabled: ms.alpha_to_coverage_enabled.unwrap_or(false),
            }
        } else {
            wgpu::MultisampleState::default()
        };

        // Build fragment targets - need to own them
        let frag_targets: Vec<Option<wgpu::ColorTargetState>> = if let Some(ref frag_desc) = descriptor.fragment {
            frag_desc.targets.iter().map(|target| {
                let blend = target.blend.as_ref().map(|b| {
                    wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: crate::parse::parse_blend_factor(&b.color.src_factor),
                            dst_factor: crate::parse::parse_blend_factor(&b.color.dst_factor),
                            operation: crate::parse::parse_blend_operation(&b.color.operation),
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: crate::parse::parse_blend_factor(&b.alpha.src_factor),
                            dst_factor: crate::parse::parse_blend_factor(&b.alpha.dst_factor),
                            operation: crate::parse::parse_blend_operation(&b.alpha.operation),
                        },
                    }
                });

                Some(wgpu::ColorTargetState {
                    format: crate::parse::parse_texture_format(&target.format),
                    blend,
                    write_mask: target.write_mask.map(|m| wgpu::ColorWrites::from_bits(m).unwrap_or(wgpu::ColorWrites::ALL)).unwrap_or(wgpu::ColorWrites::ALL),
                })
            }).collect()
        } else {
            vec![]
        };

        // Build fragment state
        let fragment = descriptor.fragment.as_ref().map(|frag_desc| {
            wgpu::FragmentState {
                module: &frag_desc.module.shader,
                entry_point: &frag_desc.entry_point,
                targets: &frag_targets,
            }
        });

        let pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: descriptor.label.as_deref(),
            layout: layout_ref,
            vertex: wgpu::VertexState {
                module: &descriptor.vertex.module.shader,
                entry_point: &descriptor.vertex.entry_point,
                buffers: &vertex_buffers,
            },
            fragment,
            primitive,
            depth_stencil,
            multisample,
            multiview: None,
        });

        Ok(crate::GpuRenderPipeline {
            pipeline: std::sync::Arc::new(pipeline),
        })
    }

    /// Create a render bundle - reusable recorded render commands
    /// This creates a bundle that can be executed multiple times in render passes
    #[napi]
    pub fn create_render_bundle(
        &self,
        label: String,
        pipeline: &crate::GpuRenderPipeline,
        vertex_buffers: Vec<&crate::GpuBuffer>,
        vertex_count: u32,
        bind_groups: Option<Vec<&crate::GpuBindGroup>>,
        color_formats: Vec<String>,
    ) -> Result<crate::GpuRenderBundle> {
        // Parse color formats
        let formats: Vec<Option<wgpu::TextureFormat>> = color_formats
            .iter()
            .map(|f| Some(crate::parse::parse_texture_format(f)))
            .collect();

        // Create render bundle encoder
        let mut encoder = self.device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
            label: Some(&label),
            color_formats: &formats,
            depth_stencil: None,
            sample_count: 1,
            multiview: None,
        });

        // Set pipeline
        encoder.set_pipeline(&pipeline.pipeline);

        // Set bind groups if provided
        if let Some(groups) = bind_groups {
            for (index, group) in groups.iter().enumerate() {
                encoder.set_bind_group(index as u32, &group.bind_group, &[]);
            }
        }

        // Set vertex buffers
        for (index, buffer) in vertex_buffers.iter().enumerate() {
            encoder.set_vertex_buffer(index as u32, buffer.buffer.slice(..));
        }

        // Draw
        encoder.draw(0..vertex_count, 0..1);

        // Finish and return bundle
        let bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some(&label),
        });

        Ok(crate::GpuRenderBundle::new(bundle))
    }

    /// Create an indexed render bundle
    #[napi]
    pub fn create_render_bundle_indexed(
        &self,
        label: String,
        pipeline: &crate::GpuRenderPipeline,
        vertex_buffers: Vec<&crate::GpuBuffer>,
        index_buffer: &crate::GpuBuffer,
        index_format: String,
        index_count: u32,
        bind_groups: Option<Vec<&crate::GpuBindGroup>>,
        color_formats: Vec<String>,
    ) -> Result<crate::GpuRenderBundle> {
        // Parse color formats
        let formats: Vec<Option<wgpu::TextureFormat>> = color_formats
            .iter()
            .map(|f| Some(crate::parse::parse_texture_format(f)))
            .collect();

        // Create render bundle encoder
        let mut encoder = self.device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
            label: Some(&label),
            color_formats: &formats,
            depth_stencil: None,
            sample_count: 1,
            multiview: None,
        });

        // Set pipeline
        encoder.set_pipeline(&pipeline.pipeline);

        // Set bind groups if provided
        if let Some(groups) = bind_groups {
            for (index, group) in groups.iter().enumerate() {
                encoder.set_bind_group(index as u32, &group.bind_group, &[]);
            }
        }

        // Set vertex buffers
        for (index, buffer) in vertex_buffers.iter().enumerate() {
            encoder.set_vertex_buffer(index as u32, buffer.buffer.slice(..));
        }

        // Set index buffer
        let idx_format = match index_format.as_str() {
            "uint16" => wgpu::IndexFormat::Uint16,
            "uint32" => wgpu::IndexFormat::Uint32,
            _ => wgpu::IndexFormat::Uint16,
        };
        encoder.set_index_buffer(index_buffer.buffer.slice(..), idx_format);

        // Draw indexed
        encoder.draw_indexed(0..index_count, 0, 0..1);

        // Finish and return bundle
        let bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
            label: Some(&label),
        });

        Ok(crate::GpuRenderBundle::new(bundle))
    }

    /// Destroy the device
    #[napi]
    pub fn destroy(&self) {
        // wgpu devices are automatically cleaned up
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

    /// Execute a compute pass with indirect dispatch
    /// The indirect buffer contains dispatch parameters (workgroups_x, workgroups_y, workgroups_z)
    #[napi]
    pub fn compute_pass_indirect(
        &mut self,
        pipeline: &crate::GpuComputePipeline,
        bind_groups: Vec<&crate::GpuBindGroup>,
        indirect_buffer: &crate::GpuBuffer,
        indirect_offset: u32,
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

            pass.dispatch_workgroups_indirect(&indirect_buffer.buffer, indirect_offset as u64);

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

    /// Execute a render pass with indirect drawing
    /// The indirect buffer contains draw parameters (vertex_count, instance_count, first_vertex, first_instance)
    #[napi]
    pub fn render_pass_indirect(
        &mut self,
        pipeline: &crate::GpuRenderPipeline,
        vertex_buffers: Vec<&crate::GpuBuffer>,
        indirect_buffer: &crate::GpuBuffer,
        indirect_offset: u32,
        color_attachments: Vec<&crate::GpuTextureView>,
        clear_colors: Option<Vec<Vec<f64>>>,
        bind_groups: Option<Vec<&crate::GpuBindGroup>>,
        depth_stencil_attachment: Option<&crate::GpuTextureView>,
        clear_depth: Option<f64>,
        resolve_targets: Option<Vec<&crate::GpuTextureView>>,
    ) -> Result<()> {
        if let Some(ref mut enc) = self.encoder {
            // Build color attachments (same as render_pass)
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

            if let Some(groups) = bind_groups {
                for (index, group) in groups.iter().enumerate() {
                    pass.set_bind_group(index as u32, &group.bind_group, &[]);
                }
            }

            for (index, buffer) in vertex_buffers.iter().enumerate() {
                pass.set_vertex_buffer(index as u32, buffer.buffer.slice(..));
            }

            pass.draw_indirect(&indirect_buffer.buffer, indirect_offset as u64);

            drop(pass);
            Ok(())
        } else {
            Err(Error::from_reason("Command encoder already finished"))
        }
    }

    /// Execute a render pass with indexed indirect drawing
    /// The indirect buffer contains draw parameters (index_count, instance_count, first_index, base_vertex, first_instance)
    #[napi]
    pub fn render_pass_indexed_indirect(
        &mut self,
        pipeline: &crate::GpuRenderPipeline,
        vertex_buffers: Vec<&crate::GpuBuffer>,
        index_buffer: &crate::GpuBuffer,
        index_format: String,
        indirect_buffer: &crate::GpuBuffer,
        indirect_offset: u32,
        color_attachments: Vec<&crate::GpuTextureView>,
        clear_colors: Option<Vec<Vec<f64>>>,
        bind_groups: Option<Vec<&crate::GpuBindGroup>>,
        depth_stencil_attachment: Option<&crate::GpuTextureView>,
        clear_depth: Option<f64>,
        resolve_targets: Option<Vec<&crate::GpuTextureView>>,
    ) -> Result<()> {
        if let Some(ref mut enc) = self.encoder {
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

            if let Some(groups) = bind_groups {
                for (index, group) in groups.iter().enumerate() {
                    pass.set_bind_group(index as u32, &group.bind_group, &[]);
                }
            }

            for (index, buffer) in vertex_buffers.iter().enumerate() {
                pass.set_vertex_buffer(index as u32, buffer.buffer.slice(..));
            }

            let format = match index_format.as_str() {
                "uint16" => wgpu::IndexFormat::Uint16,
                "uint32" => wgpu::IndexFormat::Uint32,
                _ => wgpu::IndexFormat::Uint16,
            };
            pass.set_index_buffer(index_buffer.buffer.slice(..), format);

            pass.draw_indexed_indirect(&indirect_buffer.buffer, indirect_offset as u64);

            drop(pass);
            Ok(())
        } else {
            Err(Error::from_reason("Command encoder already finished"))
        }
    }

    /// Write a timestamp to a query set
    /// query_set: the query set to write to
    /// query_index: the index of the query to write (0 to count-1)
    #[napi]
    pub fn write_timestamp(
        &mut self,
        query_set: &crate::GpuQuerySet,
        query_index: u32,
    ) -> Result<()> {
        if let Some(ref mut enc) = self.encoder {
            enc.write_timestamp(&query_set.query_set, query_index);
            Ok(())
        } else {
            Err(Error::from_reason("Command encoder already finished"))
        }
    }

    /// Resolve query results to a buffer
    /// query_set: the query set to resolve
    /// first_query: the first query index to resolve
    /// query_count: the number of queries to resolve
    /// destination: the buffer to write results to
    /// destination_offset: the byte offset in the destination buffer
    #[napi]
    pub fn resolve_query_set(
        &mut self,
        query_set: &crate::GpuQuerySet,
        first_query: u32,
        query_count: u32,
        destination: &crate::GpuBuffer,
        destination_offset: u32,
    ) -> Result<()> {
        if let Some(ref mut enc) = self.encoder {
            enc.resolve_query_set(
                &query_set.query_set,
                first_query..first_query + query_count,
                &destination.buffer,
                destination_offset as u64,
            );
            Ok(())
        } else {
            Err(Error::from_reason("Command encoder already finished"))
        }
    }

    /// Execute render bundles in a render pass
    /// This is more efficient than recording the same commands multiple times
    #[napi]
    pub fn render_pass_bundles(
        &mut self,
        bundles: Vec<&crate::GpuRenderBundle>,
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

            // Execute bundles
            let bundle_refs: Vec<&wgpu::RenderBundle> = bundles.iter().map(|b| &b.bundle).collect();
            pass.execute_bundles(bundle_refs);

            drop(pass);
            Ok(())
        } else {
            Err(Error::from_reason("Command encoder already finished"))
        }
    }

    /// Copy data from one buffer to another (WebGPU standard method)
    #[napi(js_name = "copyBufferToBuffer")]
    pub fn copy_buffer_to_buffer_standard(
        &mut self,
        source: &crate::GpuBuffer,
        source_offset: i64,
        destination: &crate::GpuBuffer,
        destination_offset: i64,
        size: i64,
    ) -> Result<()> {
        if let Some(ref mut enc) = self.encoder {
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

    /// Copy data from buffer to texture (WebGPU standard method)
    #[napi(js_name = "copyBufferToTexture")]
    pub fn copy_buffer_to_texture_standard(
        &mut self,
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
        if let Some(ref mut enc) = self.encoder {
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

    /// Copy data from texture to buffer (WebGPU standard method)
    #[napi(js_name = "copyTextureToBuffer")]
    pub fn copy_texture_to_buffer_standard(
        &mut self,
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
        if let Some(ref mut enc) = self.encoder {
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

    /// Finish encoding and return a command buffer
    #[napi]
    pub fn finish(&mut self) -> GpuCommandBuffer {
        let buffer = self.encoder.take().map(|e| e.finish());
        GpuCommandBuffer { buffer }
    }
}

#[napi]
pub struct GpuCommandBuffer {
    pub(crate) buffer: Option<wgpu::CommandBuffer>,
}
