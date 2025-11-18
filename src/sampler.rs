use napi_derive::napi;
use std::sync::Arc;

/// Sampler descriptor
#[napi(object)]
pub struct SamplerDescriptor {
    pub label: Option<String>,
    pub address_mode_u: Option<String>,
    pub address_mode_v: Option<String>,
    pub address_mode_w: Option<String>,
    pub mag_filter: Option<String>,
    pub min_filter: Option<String>,
    pub mipmap_filter: Option<String>,
    pub lod_min_clamp: Option<f64>,
    pub lod_max_clamp: Option<f64>,
    pub compare: Option<String>,
    pub max_anisotropy: Option<u32>,
}

/// GPU sampler - defines how textures are sampled in shaders
///
/// Samplers control filtering (nearest/linear), address modes (repeat/clamp),
/// and LOD (level of detail) for texture sampling.
#[napi]
pub struct GpuSampler {
    pub(crate) sampler: Arc<wgpu::Sampler>,
}

impl GpuSampler {
    pub(crate) fn new(sampler: wgpu::Sampler) -> Self {
        Self {
            sampler: Arc::new(sampler),
        }
    }
}

pub(crate) fn create_sampler(device: &wgpu::Device, descriptor: &SamplerDescriptor) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        label: descriptor.label.as_deref(),
        address_mode_u: crate::parse::parse_address_mode(descriptor.address_mode_u.as_ref()),
        address_mode_v: crate::parse::parse_address_mode(descriptor.address_mode_v.as_ref()),
        address_mode_w: crate::parse::parse_address_mode(descriptor.address_mode_w.as_ref()),
        mag_filter: crate::parse::parse_filter_mode(descriptor.mag_filter.as_ref()),
        min_filter: crate::parse::parse_filter_mode(descriptor.min_filter.as_ref()),
        mipmap_filter: crate::parse::parse_filter_mode(descriptor.mipmap_filter.as_ref()),
        lod_min_clamp: descriptor.lod_min_clamp.unwrap_or(0.0) as f32,
        lod_max_clamp: descriptor.lod_max_clamp.unwrap_or(32.0) as f32,
        compare: crate::parse::parse_compare_function(descriptor.compare.as_ref()),
        anisotropy_clamp: descriptor.max_anisotropy.unwrap_or(1) as u16,
        border_color: None,
    })
}
