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
    /// Tracks the current map state of the buffer
    /// Values: "unmapped", "pending", "mapped"
    pub(crate) map_state: Arc<Mutex<String>>,
    /// Tracks active getMappedRange() calls to prevent overlapping ranges
    /// Each entry is (offset, size) of an active range
    pub(crate) active_ranges: Arc<Mutex<Vec<(u64, u64)>>>,
}

impl GpuBuffer {
    pub(crate) fn new(buffer: wgpu::Buffer, device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self {
            buffer,
            device,
            queue,
            pending_writes: Arc::new(Mutex::new(Vec::new())),
            mapped_data: Arc::new(Mutex::new(None)),
            map_state: Arc::new(Mutex::new("unmapped".to_string())),
            active_ranges: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub(crate) fn new_mapped(buffer: wgpu::Buffer, device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self {
            buffer,
            device,
            queue,
            pending_writes: Arc::new(Mutex::new(Vec::new())),
            mapped_data: Arc::new(Mutex::new(None)),
            map_state: Arc::new(Mutex::new("mapped".to_string())),
            active_ranges: Arc::new(Mutex::new(Vec::new())),
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

    /// Get the current map state of the buffer
    ///
    /// Returns one of: "unmapped", "pending", "mapped"
    #[napi(js_name = "mapState")]
    pub fn map_state(&self) -> Result<String> {
        let state = self.map_state.lock()
            .map_err(|_| Error::from_reason("Failed to lock map state"))?;
        Ok(state.clone())
    }

    /// Map the buffer asynchronously for reading or writing
    ///
    /// Asynchronously maps the buffer for CPU access.
    /// mode: "READ" or "WRITE"
    /// Buffer must have MAP_READ or MAP_WRITE usage flag.
    #[napi(js_name = "mapAsync")]
    pub async fn map_async(&self, mode: String) -> Result<()> {
        // Set state to pending
        {
            let mut state = self.map_state.lock()
                .map_err(|_| Error::from_reason("Failed to lock map state"))?;
            *state = "pending".to_string();
        }

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

        let result = receiver.await
            .map_err(|_| Error::from_reason("Failed to receive map result"))?
            .map_err(|e| Error::from_reason(format!("Failed to map buffer: {:?}", e)));

        // Update state based on result
        if result.is_ok() {
            let mut state = self.map_state.lock()
                .map_err(|_| Error::from_reason("Failed to lock map state"))?;
            *state = "mapped".to_string();
        } else {
            let mut state = self.map_state.lock()
                .map_err(|_| Error::from_reason("Failed to lock map state"))?;
            *state = "unmapped".to_string();
        }

        result
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
    ///
    /// # Parameters
    /// * `offset` - Byte offset into the buffer (optional, default 0). Must be multiple of 8.
    /// * `size` - Number of bytes to return (optional, default remaining bytes). Must be multiple of 4.
    #[napi(js_name = "getMappedRange")]
    pub fn get_mapped_range(&self, offset: Option<u32>, size: Option<u32>) -> Result<Buffer> {
        // Validate map state (WebGPU spec requirement)
        let state = self.map_state.lock()
            .map_err(|_| Error::from_reason("Failed to lock map state"))?;
        if state.as_str() != "mapped" {
            return Err(Error::from_reason(format!(
                "Buffer must be mapped before calling getMappedRange(). Current state: {}",
                state
            )));
        }
        drop(state);

        let buffer_size = self.buffer.size();
        let offset = offset.unwrap_or(0) as u64;
        let size = size.map(|s| s as u64).unwrap_or(buffer_size - offset);

        // Validate alignment (WebGPU spec requirements)
        if offset % 8 != 0 {
            return Err(Error::from_reason(format!(
                "Offset ({}) must be a multiple of 8",
                offset
            )));
        }
        if size % 4 != 0 {
            return Err(Error::from_reason(format!(
                "Size ({}) must be a multiple of 4",
                size
            )));
        }

        // Validate bounds
        if offset + size > buffer_size {
            return Err(Error::from_reason(format!(
                "Range (offset {} + size {}) exceeds buffer size ({})",
                offset, size, buffer_size
            )));
        }

        // Check for overlapping ranges (WebGPU spec requirement)
        let mut ranges = self.active_ranges.lock()
            .map_err(|_| Error::from_reason("Failed to lock active ranges"))?;

        let range_start = offset;
        let range_end = offset + size;

        for (active_offset, active_size) in ranges.iter() {
            let active_start = *active_offset;
            let active_end = active_offset + active_size;

            // Two ranges overlap if: range1.start < range2.end AND range1.end > range2.start
            if range_start < active_end && range_end > active_start {
                return Err(Error::from_reason(format!(
                    "getMappedRange() range [{}, {}) overlaps with existing range [{}, {})",
                    range_start, range_end, active_start, active_end
                )));
            }
        }

        // No overlap detected, add this range to active ranges
        ranges.push((offset, size));
        drop(ranges);

        let slice = self.buffer.slice(offset..offset + size);
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
    ///                       Note: In JavaScript, this is handled automatically by the wrapper.
    ///
    /// # WebGPU Standard Usage (JavaScript)
    /// ```js
    /// // Write pattern
    /// const range = buffer.getMappedRange()
    /// const view = new Float32Array(range)
    /// view[0] = 1.0
    /// buffer.unmap()  // Automatically flushes changes
    ///
    /// // Read pattern
    /// const data = buffer.getMappedRange()
    /// const view = new Float32Array(data)
    /// console.log(view[0])
    /// buffer.unmap()
    /// ```
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

        // Clear active ranges (all getMappedRange calls are invalidated on unmap)
        let mut ranges = self.active_ranges.lock()
            .map_err(|_| Error::from_reason("Failed to lock active ranges"))?;
        ranges.clear();

        // Update map state to unmapped
        let mut state = self.map_state.lock()
            .map_err(|_| Error::from_reason("Failed to lock map state"))?;
        *state = "unmapped".to_string();

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
