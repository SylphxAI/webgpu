use napi::bindgen_prelude::*;
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

/// Sampler
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

fn parse_address_mode(mode: Option<&String>) -> wgpu::AddressMode {
    match mode.map(|s| s.as_str()) {
        Some("clamp-to-edge") => wgpu::AddressMode::ClampToEdge,
        Some("repeat") => wgpu::AddressMode::Repeat,
        Some("mirror-repeat") => wgpu::AddressMode::MirrorRepeat,
        _ => wgpu::AddressMode::ClampToEdge,
    }
}

fn parse_filter_mode(mode: Option<&String>) -> wgpu::FilterMode {
    match mode.map(|s| s.as_str()) {
        Some("linear") => wgpu::FilterMode::Linear,
        Some("nearest") => wgpu::FilterMode::Nearest,
        _ => wgpu::FilterMode::Nearest,
    }
}

fn parse_compare_function(func: Option<&String>) -> Option<wgpu::CompareFunction> {
    match func.map(|s| s.as_str()) {
        Some("never") => Some(wgpu::CompareFunction::Never),
        Some("less") => Some(wgpu::CompareFunction::Less),
        Some("equal") => Some(wgpu::CompareFunction::Equal),
        Some("less-equal") => Some(wgpu::CompareFunction::LessEqual),
        Some("greater") => Some(wgpu::CompareFunction::Greater),
        Some("not-equal") => Some(wgpu::CompareFunction::NotEqual),
        Some("greater-equal") => Some(wgpu::CompareFunction::GreaterEqual),
        Some("always") => Some(wgpu::CompareFunction::Always),
        _ => None,
    }
}

pub(crate) fn create_sampler(device: &wgpu::Device, descriptor: &SamplerDescriptor) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        label: descriptor.label.as_deref(),
        address_mode_u: parse_address_mode(descriptor.address_mode_u.as_ref()),
        address_mode_v: parse_address_mode(descriptor.address_mode_v.as_ref()),
        address_mode_w: parse_address_mode(descriptor.address_mode_w.as_ref()),
        mag_filter: parse_filter_mode(descriptor.mag_filter.as_ref()),
        min_filter: parse_filter_mode(descriptor.min_filter.as_ref()),
        mipmap_filter: parse_filter_mode(descriptor.mipmap_filter.as_ref()),
        lod_min_clamp: descriptor.lod_min_clamp.unwrap_or(0.0) as f32,
        lod_max_clamp: descriptor.lod_max_clamp.unwrap_or(32.0) as f32,
        compare: parse_compare_function(descriptor.compare.as_ref()),
        anisotropy_clamp: descriptor.max_anisotropy.unwrap_or(1) as u16,
        border_color: None,
    })
}
