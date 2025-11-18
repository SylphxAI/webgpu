use napi_derive::napi;
use std::sync::Arc;

/// Texture descriptor
#[napi(object)]
pub struct TextureDescriptor {
    pub label: Option<String>,
    pub width: u32,
    pub height: u32,
    pub depth: Option<u32>,
    pub format: String,
    pub usage: u32,
    pub dimension: Option<String>,
    pub mip_level_count: Option<u32>,
    pub sample_count: Option<u32>,
}

/// Texture
#[napi]
pub struct GpuTexture {
    pub(crate) texture: Arc<wgpu::Texture>,
}

impl GpuTexture {
    pub(crate) fn new(texture: wgpu::Texture) -> Self {
        Self {
            texture: Arc::new(texture),
        }
    }
}

#[napi]
impl GpuTexture {
    /// Create a view of this texture
    #[napi]
    pub fn create_view(&self, label: Option<String>) -> GpuTextureView {
        let view = self.texture.create_view(&wgpu::TextureViewDescriptor {
            label: label.as_deref(),
            format: None,
            dimension: None,
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        GpuTextureView::new(view)
    }

    /// Get texture width
    #[napi]
    pub fn width(&self) -> u32 {
        self.texture.width()
    }

    /// Get texture height
    #[napi]
    pub fn height(&self) -> u32 {
        self.texture.height()
    }

    /// Destroy the texture
    #[napi]
    pub fn destroy(&self) {
        self.texture.destroy();
    }
}

/// Texture view
#[napi]
pub struct GpuTextureView {
    pub(crate) view: Arc<wgpu::TextureView>,
}

impl GpuTextureView {
    pub(crate) fn new(view: wgpu::TextureView) -> Self {
        Self {
            view: Arc::new(view),
        }
    }
}
