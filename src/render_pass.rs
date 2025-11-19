use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Render pass encoder following WebGPU spec
/// Records commands for rendering operations
#[napi]
pub struct GpuRenderPassEncoder {
    // Store as erased pointer to avoid lifetime issues
    pub(crate) pass: Option<*mut ()>,
}

#[napi]
impl GpuRenderPassEncoder {
    /// Set the pipeline for this render pass (WebGPU standard method)
    #[napi(js_name = "setPipeline")]
    pub fn set_pipeline(&mut self, pipeline: &crate::GpuRenderPipeline) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                pass.set_pipeline(&pipeline.pipeline);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Set a bind group for this render pass (WebGPU standard method)
    #[napi(js_name = "setBindGroup")]
    pub fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: &crate::GpuBindGroup,
        dynamic_offsets: Option<Vec<u32>>,
    ) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                let offsets = dynamic_offsets.unwrap_or_default();
                pass.set_bind_group(index, &bind_group.bind_group, &offsets);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Set the vertex buffer for this render pass (WebGPU standard method)
    #[napi(js_name = "setVertexBuffer")]
    pub fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer: &crate::GpuBuffer,
        offset: Option<f64>,
        size: Option<f64>,
    ) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                let buffer_slice = if let (Some(off), Some(sz)) = (offset, size) {
                    buffer.buffer.slice(off as u64..(off as u64 + sz as u64))
                } else if let Some(off) = offset {
                    buffer.buffer.slice(off as u64..)
                } else {
                    buffer.buffer.slice(..)
                };
                pass.set_vertex_buffer(slot, buffer_slice);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Set the index buffer for this render pass (WebGPU standard method)
    #[napi(js_name = "setIndexBuffer")]
    pub fn set_index_buffer(
        &mut self,
        buffer: &crate::GpuBuffer,
        index_format: String,
        offset: Option<f64>,
        size: Option<f64>,
    ) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                let format = match index_format.as_str() {
                    "uint16" => wgpu::IndexFormat::Uint16,
                    "uint32" => wgpu::IndexFormat::Uint32,
                    _ => return Err(Error::from_reason(format!("Invalid index format: {}", index_format))),
                };
                let buffer_slice = if let (Some(off), Some(sz)) = (offset, size) {
                    buffer.buffer.slice(off as u64..(off as u64 + sz as u64))
                } else if let Some(off) = offset {
                    buffer.buffer.slice(off as u64..)
                } else {
                    buffer.buffer.slice(..)
                };
                pass.set_index_buffer(buffer_slice, format);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Draw primitives (WebGPU standard method)
    #[napi]
    pub fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: Option<u32>,
        first_vertex: Option<u32>,
        first_instance: Option<u32>,
    ) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                pass.draw(
                    first_vertex.unwrap_or(0)..first_vertex.unwrap_or(0) + vertex_count,
                    first_instance.unwrap_or(0)..first_instance.unwrap_or(0) + instance_count.unwrap_or(1),
                );
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Draw indexed primitives (WebGPU standard method)
    #[napi(js_name = "drawIndexed")]
    pub fn draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: Option<u32>,
        first_index: Option<u32>,
        base_vertex: Option<i32>,
        first_instance: Option<u32>,
    ) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                pass.draw_indexed(
                    first_index.unwrap_or(0)..first_index.unwrap_or(0) + index_count,
                    base_vertex.unwrap_or(0),
                    first_instance.unwrap_or(0)..first_instance.unwrap_or(0) + instance_count.unwrap_or(1),
                );
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Draw primitives using parameters from a buffer (WebGPU standard method)
    #[napi(js_name = "drawIndirect")]
    pub fn draw_indirect(
        &mut self,
        indirect_buffer: &crate::GpuBuffer,
        indirect_offset: f64,
    ) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                pass.draw_indirect(&indirect_buffer.buffer, indirect_offset as u64);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Draw indexed primitives using parameters from a buffer (WebGPU standard method)
    #[napi(js_name = "drawIndexedIndirect")]
    pub fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &crate::GpuBuffer,
        indirect_offset: f64,
    ) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                pass.draw_indexed_indirect(&indirect_buffer.buffer, indirect_offset as u64);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Execute render bundles (WebGPU standard method)
    #[napi(js_name = "executeBundles")]
    pub fn execute_bundles(&mut self, bundles: Vec<&crate::GpuRenderBundle>) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                let bundle_refs: Vec<&wgpu::RenderBundle> = bundles.iter().map(|b| &b.bundle).collect();
                pass.execute_bundles(bundle_refs);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Set the viewport for this render pass (WebGPU standard method)
    #[napi(js_name = "setViewport")]
    pub fn set_viewport(
        &mut self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        min_depth: f64,
        max_depth: f64,
    ) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                pass.set_viewport(x as f32, y as f32, width as f32, height as f32, min_depth as f32, max_depth as f32);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Set the scissor rectangle for this render pass (WebGPU standard method)
    #[napi(js_name = "setScissorRect")]
    pub fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                pass.set_scissor_rect(x, y, width, height);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Set the blend constant for this render pass (WebGPU standard method)
    #[napi(js_name = "setBlendConstant")]
    pub fn set_blend_constant(&mut self, color: Vec<f64>) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            if color.len() < 4 {
                return Err(Error::from_reason("Blend constant must have 4 components (RGBA)"));
            }
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                pass.set_blend_constant(wgpu::Color {
                    r: color[0],
                    g: color[1],
                    b: color[2],
                    a: color[3],
                });
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Set the stencil reference value for this render pass (WebGPU standard method)
    #[napi(js_name = "setStencilReference")]
    pub fn set_stencil_reference(&mut self, reference: u32) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                pass.set_stencil_reference(reference);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// End the render pass (WebGPU standard method)
    /// After calling this, the pass encoder can no longer be used
    #[napi]
    pub fn end(&mut self) {
        // Take the pass pointer and drop it properly
        if let Some(pass_ptr) = self.pass.take() {
            unsafe {
                // Cast back to the correct type and reconstruct the Box to drop it
                let _ = Box::from_raw(pass_ptr as *mut wgpu::RenderPass<'static>);
            }
        }
    }

    /// Push a debug group (WebGPU standard method)
    #[napi(js_name = "pushDebugGroup")]
    pub fn push_debug_group(&mut self, label: String) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                pass.push_debug_group(&label);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Pop a debug group (WebGPU standard method)
    #[napi(js_name = "popDebugGroup")]
    pub fn pop_debug_group(&mut self) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                pass.pop_debug_group();
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }

    /// Insert a debug marker (WebGPU standard method)
    #[napi(js_name = "insertDebugMarker")]
    pub fn insert_debug_marker(&mut self, label: String) -> Result<()> {
        if let Some(pass_ptr) = self.pass {
            unsafe {
                let pass = &mut *(pass_ptr as *mut wgpu::RenderPass<'_>);
                pass.insert_debug_marker(&label);
            }
            Ok(())
        } else {
            Err(Error::from_reason("Render pass already ended"))
        }
    }
}

// Implement Drop to ensure the pass is properly cleaned up if not manually ended
impl Drop for GpuRenderPassEncoder {
    fn drop(&mut self) {
        if let Some(pass_ptr) = self.pass.take() {
            unsafe {
                let _ = Box::from_raw(pass_ptr as *mut wgpu::RenderPass<'static>);
            }
        }
    }
}
