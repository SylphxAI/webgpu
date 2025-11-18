use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;

/// Pipeline layout descriptor
#[napi(object)]
pub struct PipelineLayoutDescriptor {
    pub label: Option<String>,
    // Note: bind_group_layouts will be passed separately to avoid External Vec issues
}

/// Pipeline layout - defines bind group layouts for a pipeline
///
/// Pipeline layouts specify the organization of bind groups (resources like buffers, textures)
/// that shaders can access during pipeline execution.
#[napi]
pub struct GpuPipelineLayout {
    pub(crate) layout: Arc<wgpu::PipelineLayout>,
}

/// Compute pipeline descriptor
#[napi(object)]
pub struct ComputePipelineDescriptor {
    pub label: Option<String>,
    pub layout: Option<External<GpuPipelineLayout>>,
    pub compute: ComputeStage,
}

#[napi(object)]
pub struct ComputeStage {
    pub module: External<crate::GpuShaderModule>,
    pub entry_point: String,
}

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

/// Render pipeline descriptor (basic)
#[napi(object)]
pub struct RenderPipelineDescriptor {
    pub label: Option<String>,
    pub layout: Option<External<GpuPipelineLayout>>,
    pub vertex: VertexState,
    pub fragment: Option<FragmentState>,
    pub primitive: Option<PrimitiveState>,
}

#[napi(object)]
pub struct VertexState {
    pub module: External<crate::GpuShaderModule>,
    pub entry_point: String,
    pub buffers: Option<Vec<VertexBufferLayout>>,
}

#[napi(object)]
pub struct VertexBufferLayout {
    pub array_stride: i64,
    pub step_mode: Option<String>,
    pub attributes: Vec<VertexAttribute>,
}

#[napi(object)]
pub struct VertexAttribute {
    pub format: String,
    pub offset: i64,
    pub shader_location: u32,
}

#[napi(object)]
pub struct FragmentState {
    pub module: External<crate::GpuShaderModule>,
    pub entry_point: String,
    pub targets: Vec<ColorTargetState>,
}

#[napi(object)]
pub struct ColorTargetState {
    pub format: String,
    pub blend: Option<BlendState>,
    pub write_mask: Option<u32>,
}

#[napi(object)]
pub struct BlendState {
    pub color: BlendComponent,
    pub alpha: BlendComponent,
}

#[napi(object)]
pub struct BlendComponent {
    pub src_factor: String,
    pub dst_factor: String,
    pub operation: String,
}

#[napi(object)]
pub struct PrimitiveState {
    pub topology: Option<String>,
    pub strip_index_format: Option<String>,
    pub front_face: Option<String>,
    pub cull_mode: Option<String>,
}

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

