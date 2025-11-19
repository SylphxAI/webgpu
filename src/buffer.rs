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

    /// Map the buffer asynchronously for reading or writing
    ///
    /// Asynchronously maps the buffer for CPU access.
    /// mode: "READ" or "WRITE"
    /// Buffer must have MAP_READ or MAP_WRITE usage flag.
    #[napi(js_name = "mapAsync")]
    pub async fn map_async(&self, mode: String) -> Result<()> {
        let slice = self.buffer.slice(..);

        let map_mode = match mode.as_str() {
            "READ" => wgpu::MapMode::Read,
            "WRITE" => wgpu::MapMode::Write,
            _ => return Err(Error::from_reason(format!("Invalid map mode: {}. Use 'READ' or 'WRITE'", mode))),
        };

        let (sender, receiver) = futures::channel::oneshot::channel();
        slice.map_async(map_mode, move |result| {
            let _ = sender.send(result);
        });

        self.device.poll(wgpu::Maintain::Wait);

        receiver.await
            .map_err(|_| Error::from_reason("Failed to receive map result"))?
            .map_err(|e| Error::from_reason(format!("Failed to map buffer: {:?}", e)))?;

        Ok(())
    }

    /// Get the mapped range as a buffer
    ///
    /// Returns the mapped data as a Node.js Buffer.
    /// Must be called after mapAsync() succeeds or if buffer created with mappedAtCreation: true.
    /// The buffer must remain mapped until unmap() is called.
    ///
    /// NOTE: For write operations, use writeMappedRange() to write data back to GPU.
    #[napi(js_name = "getMappedRange")]
    pub fn get_mapped_range(&self) -> Result<Buffer> {
        let slice = self.buffer.slice(..);
        let data = slice.get_mapped_range();
        Ok(Buffer::from(data.to_vec()))
    }

    /// Write data to the mapped range
    ///
    /// Writes data from a Node.js Buffer to the GPU buffer's mapped memory.
    /// Must be called while the buffer is mapped (after mapAsync or if created with mappedAtCreation: true).
    /// After writing, call unmap() to make the data available to GPU operations.
    ///
    /// # Arguments
    /// * `data` - The data to write
    /// * `offset` - Byte offset into the buffer (optional, default 0)
    #[napi(js_name = "writeMappedRange")]
    pub fn write_mapped_range(&self, data: Buffer, offset: Option<u32>) -> Result<()> {
        let offset = offset.unwrap_or(0) as usize;
        let data_slice = data.as_ref();

        let slice = self.buffer.slice(..);
        let mut mapped = slice.get_mapped_range_mut();

        // Check bounds
        if offset + data_slice.len() > mapped.len() {
            return Err(Error::from_reason(format!(
                "Data size ({} bytes) + offset ({} bytes) exceeds buffer size ({} bytes)",
                data_slice.len(), offset, mapped.len()
            )));
        }

        // Copy data to mapped range
        mapped[offset..offset + data_slice.len()].copy_from_slice(data_slice);

        Ok(())
    }

    /// Unmap the buffer
    ///
    /// Releases the mapped memory. Must be called after mapping operations before using buffer in GPU operations.
    /// Any data written with writeMappedRange() will be flushed to the GPU.
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
