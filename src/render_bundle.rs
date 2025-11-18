use napi_derive::napi;

/// Render bundle - pre-recorded render commands that can be reused
///
/// Render bundles record draw commands once and execute them multiple times.
/// This reduces CPU overhead by avoiding re-recording commands every frame.
#[napi]
pub struct GpuRenderBundle {
    pub(crate) bundle: wgpu::RenderBundle,
}

impl GpuRenderBundle {
    pub(crate) fn new(bundle: wgpu::RenderBundle) -> Self {
        Self { bundle }
    }
}

#[napi]
impl GpuRenderBundle {
    /// Destroy the render bundle (automatic when dropped)
    #[napi]
    pub fn destroy(&self) {
        // wgpu render bundles are automatically cleaned up when dropped
    }
}
