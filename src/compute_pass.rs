use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Compute pass encoder following WebGPU spec
/// Records commands for compute shader execution
#[napi]
pub struct GpuComputePassEncoder {
    // Store as erased pointer to avoid lifetime issues
    pub(crate) pass: Option<*mut ()>,
}

#[napi]
impl GpuComputePassEncoder {
    /// Set the pipeline for this compute pass (WebGPU standard method)
    #[napi(js_name = "setPipeline")]
    pub fn set_pipeline(&mut self, pipeline: &crate::GpuComputePipeline) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::ComputePass<'_>);
                pass.set_pipeline(&pipeline.pipeline);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Compute pass already ended"))
        }
    }

    /// Set a bind group for this compute pass (WebGPU standard method)
    #[napi(js_name = "setBindGroup")]
    pub fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: &crate::GpuBindGroup,
        dynamic_offsets: Option<Vec<u32>>,
    ) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::ComputePass<'_>);
                let offsets = dynamic_offsets.unwrap_or_default();
                pass.set_bind_group(index, &bind_group.bind_group, &offsets);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Compute pass already ended"))
        }
    }

    /// Dispatch work to the compute shader (WebGPU standard method)
    #[napi(js_name = "dispatchWorkgroups")]
    pub fn dispatch_workgroups(
        &mut self,
        workgroup_count_x: u32,
        workgroup_count_y: Option<u32>,
        workgroup_count_z: Option<u32>,
    ) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::ComputePass<'_>);
                pass.dispatch_workgroups(
                    workgroup_count_x,
                    workgroup_count_y.unwrap_or(1),
                    workgroup_count_z.unwrap_or(1),
                );
            }
            Ok(())
        } else {
            Err(Error::from_reason("Compute pass already ended"))
        }
    }

    /// Dispatch work using parameters from a buffer (WebGPU standard method)
    #[napi(js_name = "dispatchWorkgroupsIndirect")]
    pub fn dispatch_workgroups_indirect(
        &mut self,
        indirect_buffer: &crate::GpuBuffer,
        indirect_offset: f64,
    ) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::ComputePass<'_>);
                pass.dispatch_workgroups_indirect(&indirect_buffer.buffer, indirect_offset as u64);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Compute pass already ended"))
        }
    }

    /// End the compute pass (WebGPU standard method)
    /// After calling this, the pass encoder can no longer be used
    #[napi]
    pub fn end(&mut self) {
        // Take the pass pointer and drop it properly
        if let Some(pass_ptr) = self.pass.take() {
            unsafe {
                // Cast back to the correct type and reconstruct the Box to drop it
                let _ = Box::from_raw(pass_ptr as *mut wgpu::ComputePass<'static>);
            }
        }
    }

    /// Push a debug group (WebGPU standard method)
    #[napi(js_name = "pushDebugGroup")]
    pub fn push_debug_group(&mut self, label: String) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::ComputePass<'_>);
                pass.push_debug_group(&label);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Compute pass already ended"))
        }
    }

    /// Pop a debug group (WebGPU standard method)
    #[napi(js_name = "popDebugGroup")]
    pub fn pop_debug_group(&mut self) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::ComputePass<'_>);
                pass.pop_debug_group();
            }
            Ok(())
        } else {
            Err(Error::from_reason("Compute pass already ended"))
        }
    }

    /// Insert a debug marker (WebGPU standard method)
    #[napi(js_name = "insertDebugMarker")]
    pub fn insert_debug_marker(&mut self, label: String) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::ComputePass<'_>);
                pass.insert_debug_marker(&label);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Compute pass already ended"))
        }
    }
}

// Implement Drop to ensure the pass is properly cleaned up if not manually ended
impl Drop for GpuComputePassEncoder {
    fn drop(&mut self) {
        if let Some(pass_ptr) = self.pass.take() {
            unsafe {
                let _ = Box::from_raw(pass_ptr as *mut wgpu::ComputePass<'static>);
            }
        }
    }
}
