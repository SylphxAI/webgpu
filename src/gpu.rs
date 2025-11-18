use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub struct Gpu {
    instance: wgpu::Instance,
}

#[napi]
impl Gpu {
    /// Create a new GPU instance
    ///
    /// Example:
    /// ```js
    /// const gpu = Gpu.create()
    /// ```
    #[napi(factory)]
    pub fn create() -> Self {
        Self {
            instance: wgpu::Instance::default(),
        }
    }

    /// Request a GPU adapter
    ///
    /// Example:
    /// ```js
    /// const adapter = await gpu.requestAdapter()
    /// ```
    #[napi]
    pub async fn request_adapter(&self, power_preference: Option<String>) -> Result<crate::GpuAdapter> {
        let power_pref = match power_preference.as_deref() {
            Some("low-power") => wgpu::PowerPreference::LowPower,
            Some("high-performance") => wgpu::PowerPreference::HighPerformance,
            _ => wgpu::PowerPreference::default(),
        };

        let adapter = self.instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: power_pref,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| Error::from_reason("No suitable GPU adapter found"))?;

        Ok(crate::GpuAdapter::new(adapter))
    }

    /// Enumerate all available adapters
    #[napi]
    pub fn enumerate_adapters(&self) -> Vec<String> {
        self.instance
            .enumerate_adapters(wgpu::Backends::all())
            .into_iter()
            .map(|adapter| {
                let info = adapter.get_info();
                format!("{} ({:?})", info.name, info.backend)
            })
            .collect()
    }
}
