use napi_derive::napi;

/// GPU supported limits following WebGPU spec
#[napi(object)]
#[derive(Clone)]
pub struct GpuSupportedLimits {
    #[napi(js_name = "maxTextureDimension1D")]
    pub max_texture_dimension_1d: u32,
    #[napi(js_name = "maxTextureDimension2D")]
    pub max_texture_dimension_2d: u32,
    #[napi(js_name = "maxTextureDimension3D")]
    pub max_texture_dimension_3d: u32,
    #[napi(js_name = "maxTextureArrayLayers")]
    pub max_texture_array_layers: u32,
    #[napi(js_name = "maxBindGroups")]
    pub max_bind_groups: u32,
    #[napi(js_name = "maxBindGroupsPlusVertexBuffers")]
    pub max_bind_groups_plus_vertex_buffers: u32,
    #[napi(js_name = "maxBindingsPerBindGroup")]
    pub max_bindings_per_bind_group: u32,
    #[napi(js_name = "maxDynamicUniformBuffersPerPipelineLayout")]
    pub max_dynamic_uniform_buffers_per_pipeline_layout: u32,
    #[napi(js_name = "maxDynamicStorageBuffersPerPipelineLayout")]
    pub max_dynamic_storage_buffers_per_pipeline_layout: u32,
    #[napi(js_name = "maxSampledTexturesPerShaderStage")]
    pub max_sampled_textures_per_shader_stage: u32,
    #[napi(js_name = "maxSamplersPerShaderStage")]
    pub max_samplers_per_shader_stage: u32,
    #[napi(js_name = "maxStorageBuffersPerShaderStage")]
    pub max_storage_buffers_per_shader_stage: u32,
    #[napi(js_name = "maxStorageTexturesPerShaderStage")]
    pub max_storage_textures_per_shader_stage: u32,
    #[napi(js_name = "maxUniformBuffersPerShaderStage")]
    pub max_uniform_buffers_per_shader_stage: u32,
    #[napi(js_name = "maxUniformBufferBindingSize")]
    pub max_uniform_buffer_binding_size: i64,
    #[napi(js_name = "maxStorageBufferBindingSize")]
    pub max_storage_buffer_binding_size: i64,
    #[napi(js_name = "minUniformBufferOffsetAlignment")]
    pub min_uniform_buffer_offset_alignment: u32,
    #[napi(js_name = "minStorageBufferOffsetAlignment")]
    pub min_storage_buffer_offset_alignment: u32,
    #[napi(js_name = "maxVertexBuffers")]
    pub max_vertex_buffers: u32,
    #[napi(js_name = "maxBufferSize")]
    pub max_buffer_size: i64,
    #[napi(js_name = "maxVertexAttributes")]
    pub max_vertex_attributes: u32,
    #[napi(js_name = "maxVertexBufferArrayStride")]
    pub max_vertex_buffer_array_stride: u32,
    #[napi(js_name = "maxInterStageShaderComponents")]
    pub max_inter_stage_shader_components: u32,
    #[napi(js_name = "maxComputeWorkgroupStorageSize")]
    pub max_compute_workgroup_storage_size: u32,
    #[napi(js_name = "maxComputeInvocationsPerWorkgroup")]
    pub max_compute_invocations_per_workgroup: u32,
    #[napi(js_name = "maxComputeWorkgroupSizeX")]
    pub max_compute_workgroup_size_x: u32,
    #[napi(js_name = "maxComputeWorkgroupSizeY")]
    pub max_compute_workgroup_size_y: u32,
    #[napi(js_name = "maxComputeWorkgroupSizeZ")]
    pub max_compute_workgroup_size_z: u32,
    #[napi(js_name = "maxComputeWorkgroupsPerDimension")]
    pub max_compute_workgroups_per_dimension: u32,
}

impl GpuSupportedLimits {
    pub(crate) fn from_wgpu(limits: &wgpu::Limits) -> Self {
        Self {
            max_texture_dimension_1d: limits.max_texture_dimension_1d,
            max_texture_dimension_2d: limits.max_texture_dimension_2d,
            max_texture_dimension_3d: limits.max_texture_dimension_3d,
            max_texture_array_layers: limits.max_texture_array_layers,
            max_bind_groups: limits.max_bind_groups,
            max_bind_groups_plus_vertex_buffers: limits.max_bind_groups + limits.max_vertex_buffers,
            max_bindings_per_bind_group: limits.max_bindings_per_bind_group,
            max_dynamic_uniform_buffers_per_pipeline_layout: limits.max_dynamic_uniform_buffers_per_pipeline_layout,
            max_dynamic_storage_buffers_per_pipeline_layout: limits.max_dynamic_storage_buffers_per_pipeline_layout,
            max_sampled_textures_per_shader_stage: limits.max_sampled_textures_per_shader_stage,
            max_samplers_per_shader_stage: limits.max_samplers_per_shader_stage,
            max_storage_buffers_per_shader_stage: limits.max_storage_buffers_per_shader_stage,
            max_storage_textures_per_shader_stage: limits.max_storage_textures_per_shader_stage,
            max_uniform_buffers_per_shader_stage: limits.max_uniform_buffers_per_shader_stage,
            max_uniform_buffer_binding_size: limits.max_uniform_buffer_binding_size as i64,
            max_storage_buffer_binding_size: limits.max_storage_buffer_binding_size as i64,
            min_uniform_buffer_offset_alignment: limits.min_uniform_buffer_offset_alignment,
            min_storage_buffer_offset_alignment: limits.min_storage_buffer_offset_alignment,
            max_vertex_buffers: limits.max_vertex_buffers,
            max_buffer_size: limits.max_buffer_size as i64,
            max_vertex_attributes: limits.max_vertex_attributes,
            max_vertex_buffer_array_stride: limits.max_vertex_buffer_array_stride,
            max_inter_stage_shader_components: limits.max_inter_stage_shader_components,
            max_compute_workgroup_storage_size: limits.max_compute_workgroup_storage_size,
            max_compute_invocations_per_workgroup: limits.max_compute_invocations_per_workgroup,
            max_compute_workgroup_size_x: limits.max_compute_workgroup_size_x,
            max_compute_workgroup_size_y: limits.max_compute_workgroup_size_y,
            max_compute_workgroup_size_z: limits.max_compute_workgroup_size_z,
            max_compute_workgroups_per_dimension: limits.max_compute_workgroups_per_dimension,
        }
    }
}
