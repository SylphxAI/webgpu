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

    /// Push an error scope for error handling (WebGPU standard method)
    /// NOTE: wgpu only supports "validation" and "out-of-memory" filters
    #[napi(js_name = "pushErrorScope")]
    pub fn push_error_scope(&self, filter: String) -> Result<()> {
        let filter = match filter.as_str() {
            "validation" => wgpu::ErrorFilter::Validation,
            "out-of-memory" => wgpu::ErrorFilter::OutOfMemory,
            "internal" => return Err(Error::from_reason("wgpu does not support 'internal' error filter")),
            _ => return Err(Error::from_reason(format!("Invalid error filter: {}", filter))),
        };
        self.device.push_error_scope(filter);
        Ok(())
    }

    /// Pop an error scope and return any error (WebGPU standard method)
    #[napi(js_name = "popErrorScope")]
    pub async fn pop_error_scope(&self) -> Result<Option<String>> {
        match self.device.pop_error_scope().await {
            Some(error) => Ok(Some(error.to_string())),
            None => Ok(None),
        }
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

    /// Poll the device
    #[napi]
    pub fn poll(&self, force_wait: Option<bool>) {
        self.device.poll(if force_wait.unwrap_or(false) {
            wgpu::Maintain::Wait
        } else {
            wgpu::Maintain::Poll
        });
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
        layout: &crate::GpuBindGroupLayout,
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
            layout: &layout.layout,
            entries: &entries,
        });

        Ok(crate::GpuBindGroup::new(bind_group))
    }

    /// Create a bind group with texture bindings
    #[napi(js_name = "createBindGroupTextures")]
    pub fn create_bind_group_textures(
        &self,
        descriptor: crate::BindGroupDescriptor,
        layout: &crate::GpuBindGroupLayout,
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
            layout: &layout.layout,
            entries: &entries,
        });

        Ok(crate::GpuBindGroup::new(bind_group))
    }

    /// Create a bind group with sampler bindings
    #[napi(js_name = "createBindGroupSamplers")]
    pub fn create_bind_group_samplers(
        &self,
        descriptor: crate::BindGroupDescriptor,
        layout: &crate::GpuBindGroupLayout,
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
            layout: &layout.layout,
            entries: &entries,
        });

        Ok(crate::GpuBindGroup::new(bind_group))
    }

    /// Create a pipeline layout
    #[napi(js_name = "createPipelineLayout")]
    pub fn create_pipeline_layout(
        &self,
        descriptor: crate::PipelineLayoutDescriptor,
        bind_group_layouts: Vec<&crate::GpuBindGroupLayout>,
    ) -> crate::GpuPipelineLayout {
        let bind_group_layouts_refs: Vec<_> = bind_group_layouts
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
        layout: Option<&crate::GpuPipelineLayout>,
        module: &crate::GpuShaderModule,
    ) -> crate::GpuComputePipeline {
        let layout_ref = layout.map(|l| l.layout.as_ref());

        let pipeline = self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: descriptor.label.as_deref(),
            layout: layout_ref,
            module: &module.shader,
            entry_point: &descriptor.entry_point,
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
        layout: Option<&crate::GpuPipelineLayout>,
        vertex_module: &crate::GpuShaderModule,
        fragment_module: Option<&crate::GpuShaderModule>,
    ) -> Result<crate::GpuRenderPipeline> {
        let layout_ref = layout.map(|l| l.layout.as_ref());
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
        let fragment = if descriptor.fragment.is_some() && fragment_module.is_some() {
            let frag_desc = descriptor.fragment.as_ref().unwrap();
            let frag_mod = fragment_module.unwrap();
            Some(wgpu::FragmentState {
                module: &frag_mod.shader,
                entry_point: &frag_desc.entry_point,
                targets: &frag_targets,
            })
        } else {
            None
        };

        let pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: descriptor.label.as_deref(),
            layout: layout_ref,
            vertex: wgpu::VertexState {
                module: &vertex_module.shader,
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

    /// Begin a compute pass following WebGPU standard
    /// Returns a compute pass encoder for recording compute commands
    #[napi(js_name = "beginComputePass")]
    pub fn begin_compute_pass(&mut self, descriptor: Option<crate::pipeline::ComputePassDescriptor>) -> Result<crate::GpuComputePassEncoder> {
        if let Some(ref mut enc) = self.encoder {
            let pass = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: descriptor.as_ref().and_then(|d| d.label.as_deref()),
                timestamp_writes: None,
            });
            // SAFETY: This is unsafe because we're extending the lifetime of the ComputePass
            // from being tied to the encoder to 'static. This works because:
            // 1. The pass must be ended (dropped) before encoder.finish() is called
            // 2. We use transmute to force the lifetime to 'static
            // 3. We erase the type to *mut () to avoid lifetime issues in the struct
            // 4. The JavaScript API enforces correct usage order
            let pass_static: wgpu::ComputePass<'static> = unsafe { std::mem::transmute(pass) };
            let pass_ptr = Box::into_raw(Box::new(pass_static)) as *mut ();
            Ok(crate::GpuComputePassEncoder {
                pass: Some(pass_ptr),
            })
        } else {
            Err(Error::from_reason("Command encoder already finished"))
        }
    }

    /// Begin a render pass following WebGPU standard
    /// Returns a render pass encoder for recording render commands
    #[napi(js_name = "beginRenderPass")]
    pub fn begin_render_pass(&mut self, descriptor: crate::pipeline::RenderPassDescriptor) -> Result<crate::GpuRenderPassEncoder> {
        if let Some(ref mut enc) = self.encoder {
            // Convert color attachments
            let color_attachments: Vec<Option<wgpu::RenderPassColorAttachment>> = descriptor.color_attachments
                .iter()
                .map(|attachment| {
                    let load_op = match attachment.load_op.as_str() {
                        "clear" => {
                            let clear_value = attachment.clear_value.as_ref().map(|c| wgpu::Color {
                                r: c.r,
                                g: c.g,
                                b: c.b,
                                a: c.a,
                            }).unwrap_or(wgpu::Color::BLACK);
                            wgpu::LoadOp::Clear(clear_value)
                        },
                        _ => wgpu::LoadOp::Load,
                    };
                    let store_op = match attachment.store_op.as_str() {
                        "discard" => wgpu::StoreOp::Discard,
                        _ => wgpu::StoreOp::Store,
                    };
                    Some(wgpu::RenderPassColorAttachment {
                        view: &attachment.view.view,
                        resolve_target: attachment.resolve_target.as_ref().map(|t| t.view.as_ref()),
                        ops: wgpu::Operations {
                            load: load_op,
                            store: store_op,
                        },
                    })
                })
                .collect();

            // Convert depth/stencil attachment
            let depth_stencil_attachment = descriptor.depth_stencil_attachment.as_ref().map(|attachment| {
                let depth_ops = if attachment.depth_load_op.is_some() || attachment.depth_store_op.is_some() {
                    let load = match attachment.depth_load_op.as_deref() {
                        Some("clear") => wgpu::LoadOp::Clear(attachment.depth_clear_value.unwrap_or(1.0) as f32),
                        _ => wgpu::LoadOp::Load,
                    };
                    let store = match attachment.depth_store_op.as_deref() {
                        Some("discard") => wgpu::StoreOp::Discard,
                        _ => wgpu::StoreOp::Store,
                    };
                    Some(wgpu::Operations { load, store })
                } else {
                    None
                };

                let stencil_ops = if attachment.stencil_load_op.is_some() || attachment.stencil_store_op.is_some() {
                    let load = match attachment.stencil_load_op.as_deref() {
                        Some("clear") => wgpu::LoadOp::Clear(attachment.stencil_clear_value.unwrap_or(0)),
                        _ => wgpu::LoadOp::Load,
                    };
                    let store = match attachment.stencil_store_op.as_deref() {
                        Some("discard") => wgpu::StoreOp::Discard,
                        _ => wgpu::StoreOp::Store,
                    };
                    Some(wgpu::Operations { load, store })
                } else {
                    None
                };

                wgpu::RenderPassDepthStencilAttachment {
                    view: &attachment.view.view,
                    depth_ops,
                    stencil_ops,
                }
            });

            let pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: descriptor.label.as_deref(),
                color_attachments: &color_attachments,
                depth_stencil_attachment,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // SAFETY: This is unsafe because we're extending the lifetime of the RenderPass
            // from being tied to the encoder to 'static. This works because:
            // 1. The pass must be ended (dropped) before encoder.finish() is called
            // 2. We use transmute to force the lifetime to 'static
            // 3. We erase the type to *mut () to avoid lifetime issues in the struct
            // 4. The JavaScript API enforces correct usage order
            let pass_static: wgpu::RenderPass<'static> = unsafe { std::mem::transmute(pass) };
            let pass_ptr = Box::into_raw(Box::new(pass_static)) as *mut ();
            Ok(crate::GpuRenderPassEncoder {
                pass: Some(pass_ptr),
            })
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
