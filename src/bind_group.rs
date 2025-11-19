use napi_derive::napi;
use std::sync::Arc;

// Note: BindGroupLayoutEntry and BindGroupLayoutDescriptor are now in descriptors.rs

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

/// WebGPU-compliant bind group entry descriptor (without resource references)
/// Resources are passed separately to avoid napi-rs External serialization issues
#[napi(object)]
pub struct BindGroupEntry {
    pub binding: u32,
    pub offset: Option<i64>,      // For buffer bindings
    pub size: Option<i64>,        // For buffer bindings
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
    entry: &crate::BindGroupLayoutEntry,
) -> wgpu::BindGroupLayoutEntry {
    let visibility = parse_visibility(entry.visibility);

    // Determine binding type - WebGPU standard uses buffer/sampler/texture/storageTexture fields
    let ty = if let Some(ref buffer) = entry.buffer {
        let buffer_ty = buffer.ty.as_deref().unwrap_or("uniform");
        wgpu::BindingType::Buffer {
            ty: parse_buffer_binding_type(buffer_ty),
            has_dynamic_offset: buffer.has_dynamic_offset.unwrap_or(false),
            min_binding_size: buffer.min_binding_size.map(|s| std::num::NonZeroU64::new(s as u64)).flatten(),
        }
    } else if entry.sampler.is_some() {
        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
    } else if entry.texture.is_some() {
        wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        }
    } else if entry.storage_texture.is_some() {
        wgpu::BindingType::StorageTexture {
            access: wgpu::StorageTextureAccess::WriteOnly,
            format: wgpu::TextureFormat::Rgba8Unorm,
            view_dimension: wgpu::TextureViewDimension::D2,
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
