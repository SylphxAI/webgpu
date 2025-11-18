use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;

/// GPU buffer - contiguous memory allocation on the GPU
///
/// Buffers store data for shaders (vertices, indices, uniforms, storage).
/// They can be written from CPU (via queueWriteBuffer) and read back (via mapRead).
#[napi]
pub struct GpuBuffer {
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) device: Arc<wgpu::Device>,
}

impl GpuBuffer {
    pub(crate) fn new(buffer: wgpu::Buffer, device: Arc<wgpu::Device>) -> Self {
        Self { buffer, device }
    }
}

#[napi]
impl GpuBuffer {
    /// Get the size of the buffer
    #[napi]
    pub fn size(&self) -> u32 {
        self.buffer.size() as u32
    }

    /// Get the usage flags of the buffer
    #[napi]
    pub fn usage(&self) -> u32 {
        self.buffer.usage().bits()
    }

    /// Map the buffer for reading
    ///
    /// Asynchronously maps the buffer for CPU read access.
    /// Buffer must have MAP_READ usage flag.
    /// Returns a Node.js Buffer containing the data.
    #[napi]
    pub async fn map_read(&self) -> Result<Buffer> {
        let slice = self.buffer.slice(..);

        let (sender, receiver) = futures::channel::oneshot::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });

        self.device.poll(wgpu::Maintain::Wait);

        receiver.await
            .map_err(|_| Error::from_reason("Failed to receive map result"))?
            .map_err(|e| Error::from_reason(format!("Failed to map buffer: {:?}", e)))?;

        let data = slice.get_mapped_range();
        Ok(Buffer::from(data.to_vec()))
    }

    /// Unmap the buffer
    ///
    /// Releases the mapped memory. Must be called after mapRead before using buffer in GPU operations.
    #[napi]
    pub fn unmap(&self) {
        self.buffer.unmap();
    }

    /// Write data to buffer using mapped memory
    #[napi]
    pub async fn write_buffer(&self, _data: Buffer) -> Result<()> {
        // For buffers created with MAP_WRITE, we can map and write
        // But for now, we'll use the queue.write_buffer approach which is simpler
        // This requires the buffer to have COPY_DST usage

        // Note: This implementation requires passing the queue, but we don't have it
        // For now, we'll use a different approach - create buffer with mapped_at_creation
        Err(Error::from_reason("writeBuffer not yet implemented - use device.queueWriteBuffer instead"))
    }

    /// Destroy the buffer
    ///
    /// Explicitly releases GPU resources. Buffers are automatically destroyed when dropped.
    #[napi]
    pub fn destroy(&self) {
        self.buffer.destroy();
    }
}
