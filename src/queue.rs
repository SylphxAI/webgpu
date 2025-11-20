use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;

/// GPU queue for submitting commands following WebGPU spec
#[napi]
pub struct GpuQueue {
    pub(crate) queue: Arc<wgpu::Queue>,
}

impl GpuQueue {
    pub(crate) fn new(queue: Arc<wgpu::Queue>) -> Self {
        Self { queue }
    }
}

#[napi]
impl GpuQueue {
    /// Submit command buffers to the queue (WebGPU standard - accepts array)
    #[napi]
    pub fn submit(&self, command_buffers: Vec<&mut crate::GpuCommandBuffer>) {
        let buffers: Vec<wgpu::CommandBuffer> = command_buffers
            .into_iter()
            .filter_map(|cb| cb.buffer.take())
            .collect();
        self.queue.submit(buffers);
    }

    /// Write data to a buffer using the queue
    #[napi(js_name = "writeBuffer")]
    pub fn write_buffer(&self, buffer: &crate::GpuBuffer, offset: i64, data: Buffer) {
        self.queue.write_buffer(&buffer.buffer, offset as u64, &data);
    }

    /// Get the label of this queue
    #[napi(getter)]
    pub fn label(&self) -> Option<String> {
        None // wgpu doesn't expose queue labels
    }
}
