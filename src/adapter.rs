use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct GpuAdapter {
    pub(crate) adapter: wgpu::Adapter,
}

impl GpuAdapter {
    pub(crate) fn new(adapter: wgpu::Adapter) -> Self {
        Self { adapter }
    }
}

#[napi]
impl GpuAdapter {
    /// Get adapter information
    #[napi]
    pub fn get_info(&self) -> AdapterInfo {
        let info = self.adapter.get_info();
        AdapterInfo {
            name: info.name,
            vendor: info.vendor,
            device: info.device,
            device_type: format!("{:?}", info.device_type),
            backend: format!("{:?}", info.backend),
        }
    }

    /// Get adapter features
    #[napi]
    pub fn get_features(&self) -> Vec<String> {
        let features = self.adapter.features();

        let mut result = Vec::new();
        if features.contains(wgpu::Features::DEPTH_CLIP_CONTROL) {
            result.push("depth-clip-control".to_string());
        }
        if features.contains(wgpu::Features::TIMESTAMP_QUERY) {
            result.push("timestamp-query".to_string());
        }
        if features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC) {
            result.push("texture-compression-bc".to_string());
        }
        // Add more features as needed
        result
    }

    /// Get adapter limits
    #[napi]
    pub fn get_limits(&self) -> AdapterLimits {
        let limits = self.adapter.limits();
        AdapterLimits {
            max_texture_dimension_1d: limits.max_texture_dimension_1d,
            max_texture_dimension_2d: limits.max_texture_dimension_2d,
            max_texture_dimension_3d: limits.max_texture_dimension_3d,
            max_bind_groups: limits.max_bind_groups,
            max_buffer_size: limits.max_buffer_size as i64, // Convert to i64
        }
    }

    /// Request a device from this adapter
    #[napi]
    pub async fn request_device(&self) -> Result<crate::GpuDevice> {
        let (device, queue) = self.adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| Error::from_reason(format!("Failed to request device: {}", e)))?;

        Ok(crate::GpuDevice::new(device, queue))
    }
}

#[napi(object)]
pub struct AdapterInfo {
    pub name: String,
    pub vendor: u32,
    pub device: u32,
    pub device_type: String,
    pub backend: String,
}

#[napi(object)]
pub struct AdapterLimits {
    pub max_texture_dimension_1d: u32,
    pub max_texture_dimension_2d: u32,
    pub max_texture_dimension_3d: u32,
    pub max_bind_groups: u32,
    pub max_buffer_size: i64, // u64 not supported by napi, use i64
}
