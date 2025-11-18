use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;

/// Pipeline layout descriptor
#[napi(object)]
pub struct PipelineLayoutDescriptor {
    pub label: Option<String>,
    // Note: bind_group_layouts will be passed separately to avoid External Vec issues
}

/// Pipeline layout
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

/// Compute pipeline
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

/// Render pipeline
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

// Helper functions for parsing
pub(crate) fn parse_texture_format(format: &str) -> wgpu::TextureFormat {
    match format {
        "rgba8unorm" => wgpu::TextureFormat::Rgba8Unorm,
        "bgra8unorm" => wgpu::TextureFormat::Bgra8Unorm,
        "rgba16float" => wgpu::TextureFormat::Rgba16Float,
        "rgba32float" => wgpu::TextureFormat::Rgba32Float,
        "depth24plus" => wgpu::TextureFormat::Depth24Plus,
        "depth32float" => wgpu::TextureFormat::Depth32Float,
        _ => wgpu::TextureFormat::Rgba8Unorm,
    }
}

pub(crate) fn parse_vertex_format(format: &str) -> wgpu::VertexFormat {
    match format {
        "float32" => wgpu::VertexFormat::Float32,
        "float32x2" => wgpu::VertexFormat::Float32x2,
        "float32x3" => wgpu::VertexFormat::Float32x3,
        "float32x4" => wgpu::VertexFormat::Float32x4,
        "uint32" => wgpu::VertexFormat::Uint32,
        "sint32" => wgpu::VertexFormat::Sint32,
        _ => wgpu::VertexFormat::Float32x3,
    }
}

pub(crate) fn parse_primitive_topology(topology: Option<&String>) -> wgpu::PrimitiveTopology {
    match topology.map(|s| s.as_str()) {
        Some("point-list") => wgpu::PrimitiveTopology::PointList,
        Some("line-list") => wgpu::PrimitiveTopology::LineList,
        Some("line-strip") => wgpu::PrimitiveTopology::LineStrip,
        Some("triangle-list") | None => wgpu::PrimitiveTopology::TriangleList,
        Some("triangle-strip") => wgpu::PrimitiveTopology::TriangleStrip,
        _ => wgpu::PrimitiveTopology::TriangleList,
    }
}

pub(crate) fn parse_load_op(op: &str) -> wgpu::LoadOp<wgpu::Color> {
    match op {
        "load" => wgpu::LoadOp::Load,
        "clear" => wgpu::LoadOp::Clear(wgpu::Color::BLACK),
        _ => wgpu::LoadOp::Clear(wgpu::Color::BLACK),
    }
}

pub(crate) fn parse_store_op(op: &str) -> wgpu::StoreOp {
    match op {
        "store" => wgpu::StoreOp::Store,
        "discard" => wgpu::StoreOp::Discard,
        _ => wgpu::StoreOp::Store,
    }
}
