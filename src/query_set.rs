use napi_derive::napi;

#[napi]
pub struct GpuQuerySet {
    pub(crate) query_set: wgpu::QuerySet,
}

impl GpuQuerySet {
    pub(crate) fn new(query_set: wgpu::QuerySet) -> Self {
        Self { query_set }
    }
}

#[napi]
impl GpuQuerySet {
    /// Destroy the query set
    #[napi]
    pub fn destroy(&self) {
        // wgpu doesn't have explicit destroy for query sets
        // They're automatically cleaned up when dropped
    }
}
