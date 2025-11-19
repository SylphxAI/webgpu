use napi_derive::napi;

/// GPU supported features following WebGPU spec
#[napi]
pub struct GpuSupportedFeatures {
    pub(crate) features: wgpu::Features,
}

#[napi]
impl GpuSupportedFeatures {
    /// Check if a feature is supported
    #[napi]
    pub fn has(&self, feature: String) -> bool {
        match feature.as_str() {
            "depth-clip-control" => self.features.contains(wgpu::Features::DEPTH_CLIP_CONTROL),
            "depth32float-stencil8" => self.features.contains(wgpu::Features::DEPTH32FLOAT_STENCIL8),
            "texture-compression-bc" => self.features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC),
            "texture-compression-etc2" => self.features.contains(wgpu::Features::TEXTURE_COMPRESSION_ETC2),
            "texture-compression-astc" => self.features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC),
            "timestamp-query" => self.features.contains(wgpu::Features::TIMESTAMP_QUERY),
            "indirect-first-instance" => self.features.contains(wgpu::Features::INDIRECT_FIRST_INSTANCE),
            "shader-f16" => self.features.contains(wgpu::Features::SHADER_F16),
            "rg11b10ufloat-renderable" => self.features.contains(wgpu::Features::RG11B10UFLOAT_RENDERABLE),
            "bgra8unorm-storage" => self.features.contains(wgpu::Features::BGRA8UNORM_STORAGE),
            "float32-filterable" => self.features.contains(wgpu::Features::FLOAT32_FILTERABLE),
            _ => false,
        }
    }

    /// Get the number of features supported
    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.features.bits().count_ones()
    }
}
