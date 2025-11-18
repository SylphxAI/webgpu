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

    /// Create a render pipeline (simplified - to be implemented)
    #[napi]
    pub fn create_render_pipeline(&self, _descriptor: crate::RenderPipelineDescriptor) -> Result<crate::GpuRenderPipeline> {
        // TODO: Implement render pipeline creation
        // The complexity here is due to lifetime issues with vertex buffers and fragment targets
        // For now, focus on compute pipeline which is simpler
        Err(Error::from_reason("Render pipeline not yet implemented"))
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
