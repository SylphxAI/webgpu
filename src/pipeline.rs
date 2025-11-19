use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;

// Note: PipelineLayoutDescriptor is now in descriptors.rs

/// Pipeline layout - defines bind group layouts for a pipeline
///
/// Pipeline layouts specify the organization of bind groups (resources like buffers, textures)
/// that shaders can access during pipeline execution.
#[napi]
pub struct GpuPipelineLayout {
    pub(crate) layout: Arc<wgpu::PipelineLayout>,
}

// Note: ComputePipelineDescriptor and ComputeStage are now in descriptors.rs

/// Compute pipeline - configured compute shader program
///
/// Compute pipelines execute compute shaders for general-purpose GPU computation.
/// They're created from a shader module, entry point, and pipeline layout.
#[napi]
pub struct GpuComputePipeline {
    pub(crate) pipeline: Arc<wgpu::ComputePipeline>,
}

/// Compute pass descriptor (simplified)
#[napi(object)]
pub struct ComputePassDescriptor {
    pub label: Option<String>,
}

// Note: RenderPipelineDescriptor and all related types are now in descriptors.rs

/// Render pipeline - configured graphics pipeline
///
/// Render pipelines define the complete graphics state: vertex/fragment shaders,
/// vertex layout, blend modes, depth/stencil, and MSAA configuration.
#[napi]
pub struct GpuRenderPipeline {
    pub(crate) pipeline: Arc<wgpu::RenderPipeline>,
}

/// Render pass descriptor
#[napi(object)]
pub struct RenderPassDescriptor {
    pub label: Option<String>,
    pub color_attachments: Vec<RenderPassColorAttachment>,
    pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachment>,
}

#[napi(object)]
pub struct RenderPassColorAttachment {
    pub view: External<crate::GpuTextureView>,
    pub resolve_target: Option<External<crate::GpuTextureView>>,
    pub clear_value: Option<Color>,
    pub load_op: String,
    pub store_op: String,
}

#[napi(object)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[napi(object)]
pub struct RenderPassDepthStencilAttachment {
    pub view: External<crate::GpuTextureView>,
    pub depth_clear_value: Option<f64>,
    pub depth_load_op: Option<String>,
    pub depth_store_op: Option<String>,
    pub stencil_clear_value: Option<u32>,
    pub stencil_load_op: Option<String>,
    pub stencil_store_op: Option<String>,
}

