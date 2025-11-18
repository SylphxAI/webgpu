use napi_derive::napi;
use std::sync::Arc;

/// Bind group layout entry
#[napi(object)]
pub struct BindGroupLayoutEntry {
    pub binding: u32,
    pub visibility: u32,
    pub buffer_type: Option<String>,
    pub sampler_type: Option<String>,
    pub texture_sample_type: Option<String>,
    pub texture_view_dimension: Option<String>,
    pub storage_texture_access: Option<String>,
    pub storage_texture_format: Option<String>,
}

/// Bind group layout descriptor
#[napi(object)]
pub struct BindGroupLayoutDescriptor {
    pub label: Option<String>,
    pub entries: Vec<BindGroupLayoutEntry>,
}

/// Bind group layout - describes resource bindings for shaders
///
/// Bind group layouts define the structure of bind groups: which binding indices
/// contain buffers, textures, or samplers, and their visibility (vertex/fragment/compute).
#[napi]
pub struct GpuBindGroupLayout {
    pub(crate) layout: Arc<wgpu::BindGroupLayout>,
}

impl GpuBindGroupLayout {
    pub(crate) fn new(layout: wgpu::BindGroupLayout) -> Self {
        Self {
            layout: Arc::new(layout),
        }
    }
}

// Note: Bind group creation simplified to avoid napi-rs External/reference issues
// Use create_bind_group_buffers on GpuDevice instead

/// Bind group entry for mixed resources
#[napi(object)]
pub struct BindGroupEntry {
    pub binding: u32,
    pub resource_type: String, // "buffer", "texture", "sampler"
    pub buffer_index: Option<u32>,
    pub texture_index: Option<u32>,
    pub sampler_index: Option<u32>,
}

/// Bind group - collection of resources bound to shaders
///
/// Bind groups connect GPU resources (buffers, textures, samplers) to shader binding points.
/// They're bound to pipelines during command encoding.
#[napi]
pub struct GpuBindGroup {
    pub(crate) bind_group: Arc<wgpu::BindGroup>,
}

impl GpuBindGroup {
    pub(crate) fn new(bind_group: wgpu::BindGroup) -> Self {
        Self {
            bind_group: Arc::new(bind_group),
        }
    }
}

/// Convert visibility flags to wgpu
fn parse_visibility(visibility: u32) -> wgpu::ShaderStages {
    let mut stages = wgpu::ShaderStages::empty();
    if visibility & 0x1 != 0 {
        stages |= wgpu::ShaderStages::VERTEX;
    }
    if visibility & 0x2 != 0 {
        stages |= wgpu::ShaderStages::FRAGMENT;
    }
    if visibility & 0x4 != 0 {
        stages |= wgpu::ShaderStages::COMPUTE;
    }
    stages
}

/// Convert buffer type string to wgpu type
fn parse_buffer_binding_type(ty: &str) -> wgpu::BufferBindingType {
    match ty {
        "uniform" => wgpu::BufferBindingType::Uniform,
        "storage" => wgpu::BufferBindingType::Storage { read_only: false },
        "read-only-storage" => wgpu::BufferBindingType::Storage { read_only: true },
        _ => wgpu::BufferBindingType::Uniform,
    }
}

pub(crate) fn convert_bind_group_layout_entry(
    entry: &BindGroupLayoutEntry,
) -> wgpu::BindGroupLayoutEntry {
    let visibility = parse_visibility(entry.visibility);

    // Determine binding type
    let ty = if let Some(ref buffer_type) = entry.buffer_type {
        wgpu::BindingType::Buffer {
            ty: parse_buffer_binding_type(buffer_type),
            has_dynamic_offset: false,
            min_binding_size: None,
        }
    } else if entry.sampler_type.is_some() {
        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
    } else if entry.texture_sample_type.is_some() {
        wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        }
    } else {
        // Default to uniform buffer
        wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        }
    };

    wgpu::BindGroupLayoutEntry {
        binding: entry.binding,
        visibility,
        ty,
        count: None,
    }
}
