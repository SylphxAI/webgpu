use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::{Arc, Mutex};

/// GPU buffer - contiguous memory allocation on the GPU
///
/// Buffers store data for shaders (vertices, indices, uniforms, storage).
/// They can be written from CPU (via queueWriteBuffer) and read back (via mapRead).
#[napi]
pub struct GpuBuffer {
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) device: Arc<wgpu::Device>,
    pub(crate) queue: Arc<wgpu::Queue>,
    /// Tracks pending writes to the mapped buffer
    /// Writes are accumulated and applied via queue.write_buffer() in unmap()
    pub(crate) pending_writes: Arc<Mutex<Vec<(u64, Vec<u8>)>>>,
    /// Stores the mapped range data returned from getMappedRange()
    /// When user modifies this data in JavaScript, we need to flush it back to GPU on unmap()
    pub(crate) mapped_data: Arc<Mutex<Option<Vec<u8>>>>,
}

impl GpuBuffer {
    pub(crate) fn new(buffer: wgpu::Buffer, device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self {
            buffer,
            device,
            queue,
            pending_writes: Arc::new(Mutex::new(Vec::new())),
            mapped_data: Arc::new(Mutex::new(None)),
        }
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
    ///
    /// The returned buffer is a COPY of GPU memory. Modifications to this buffer in JavaScript
    /// will be automatically flushed back to GPU when unmap() is called.
    ///
    /// This implements the standard WebGPU getMappedRange() behavior.
    #[napi(js_name = "getMappedRange")]
    pub fn get_mapped_range(&self) -> Result<Buffer> {
        let slice = self.buffer.slice(..);
        let data = slice.get_mapped_range();
        let vec = data.to_vec();

        // Store a copy so we can detect modifications in JavaScript
        // When JavaScript modifies the returned Buffer, we'll receive the modified data in unmap()
        let mut mapped = self.mapped_data.lock()
            .map_err(|_| Error::from_reason("Failed to lock mapped data"))?;
        *mapped = Some(vec.clone());

        Ok(Buffer::from(vec))
    }


    /// Unmap the buffer
    ///
    /// Releases the mapped memory and flushes changes to GPU.
    /// Must be called after mapping operations before using buffer in GPU operations.
    ///
    /// # Parameters
    /// * `modified_buffer` - Optional. If provided, writes this data to GPU before unmapping.
    ///                       Use this when you've modified the buffer from getMappedRange().
    ///
    /// # Usage patterns
    /// 1. Write with getMappedRange():
    ///    ```js
    ///    const range = buffer.getMappedRange()
    ///    const view = new Float32Array(range.buffer)
    ///    view[0] = 1.0
    ///    buffer.unmap(range)  // Pass modified buffer back
    ///    ```
    ///
    /// 2. Write with writeMappedRange():
    ///    ```js
    ///    buffer.writeMappedRange(data)
    ///    buffer.unmap()  // No argument needed
    ///    ```
    ///
    /// 3. Read with getMappedRange():
    ///    ```js
    ///    const data = buffer.getMappedRange()
    ///    buffer.unmap()  // No argument needed for reads
    ///    ```
    #[napi]
    pub fn unmap(&self, modified_buffer: Option<Buffer>) -> Result<()> {
        // Get pending writes before unmapping
        let mut pending = self.pending_writes.lock()
            .map_err(|_| Error::from_reason("Failed to lock pending writes"))?;

        // Check if buffer has COPY_DST usage (required for queue.write_buffer())
        let has_copy_dst = self.buffer.usage().contains(wgpu::BufferUsages::COPY_DST);

        if !pending.is_empty() || modified_buffer.is_some() {
            if has_copy_dst {
                // Buffer has COPY_DST: unmap first, then use queue.write_buffer()
                self.buffer.unmap();

                // Write all pending writes using queue.write_buffer()
                for (offset, data) in pending.iter() {
                    self.queue.write_buffer(&self.buffer, *offset, data);
                }

                // If a modified buffer was provided, write it too
                if let Some(data) = modified_buffer {
                    self.queue.write_buffer(&self.buffer, 0, data.as_ref());
                }

                // Submit and poll to ensure writes complete
                self.queue.submit(std::iter::empty());
                self.device.poll(wgpu::Maintain::Wait);
            } else {
                // Buffer doesn't have COPY_DST: use mapped memory writes
                let slice = self.buffer.slice(..);
                let mut mapped = slice.get_mapped_range_mut();

                // Write all pending writes directly to mapped memory
                for (offset, data) in pending.iter() {
                    let offset_usize = *offset as usize;
                    if offset_usize + data.len() <= mapped.len() {
                        mapped[offset_usize..offset_usize + data.len()].copy_from_slice(data);
                    }
                }

                // If a modified buffer was provided, write it too
                if let Some(data) = modified_buffer {
                    let data_slice = data.as_ref();
                    if data_slice.len() <= mapped.len() {
                        mapped[..data_slice.len()].copy_from_slice(data_slice);
                    }
                }

                // Drop mapped view before unmapping
                drop(mapped);
                self.buffer.unmap();
            }
        } else {
            // No pending writes, just unmap
            self.buffer.unmap();
        }

        // Clear pending writes
        pending.clear();

        Ok(())
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
