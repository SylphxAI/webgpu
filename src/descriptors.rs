use napi_derive::napi;

/// Buffer descriptor following WebGPU spec
#[napi(object)]
pub struct BufferDescriptor {
    pub label: Option<String>,
    pub size: i64,
    pub usage: u32,
    #[napi(js_name = "mappedAtCreation")]
    pub mapped_at_creation: Option<bool>,
}

/// Shader module descriptor following WebGPU spec
#[napi(object)]
pub struct ShaderModuleDescriptor {
    pub label: Option<String>,
    pub code: String,
}

/// Pipeline layout descriptor following WebGPU spec
#[napi(object)]
pub struct PipelineLayoutDescriptor {
    pub label: Option<String>,
    // Note: bindGroupLayouts will be passed separately as External references
}

/// Compute pipeline descriptor following WebGPU spec
#[napi(object)]
pub struct ComputePipelineDescriptor {
    pub label: Option<String>,
    // layout will be passed separately as External reference
    pub compute: ComputeStage,
}

#[napi(object)]
pub struct ComputeStage {
    // module will be passed as External reference
    #[napi(js_name = "entryPoint")]
    pub entry_point: String,
}

/// Command encoder descriptor following WebGPU spec
#[napi(object)]
pub struct CommandEncoderDescriptor {
    pub label: Option<String>,
}

/// Bind group descriptor following WebGPU spec
#[napi(object)]
pub struct BindGroupDescriptor {
    pub label: Option<String>,
    // layout and entries will be passed separately
}

/// Bind group layout descriptor following WebGPU spec
#[napi(object)]
pub struct BindGroupLayoutDescriptor {
    pub label: Option<String>,
    pub entries: Vec<BindGroupLayoutEntry>,
}

#[napi(object)]
pub struct BindGroupLayoutEntry {
    pub binding: u32,
    pub visibility: u32,
    pub buffer: Option<BufferBindingLayout>,
    pub sampler: Option<SamplerBindingLayout>,
    pub texture: Option<TextureBindingLayout>,
    #[napi(js_name = "storageTexture")]
    pub storage_texture: Option<StorageTextureBindingLayout>,
}

#[napi(object)]
pub struct BufferBindingLayout {
    #[napi(js_name = "type")]
    pub ty: Option<String>, // "uniform", "storage", "read-only-storage"
    #[napi(js_name = "hasDynamicOffset")]
    pub has_dynamic_offset: Option<bool>,
    #[napi(js_name = "minBindingSize")]
    pub min_binding_size: Option<i64>,
}

#[napi(object)]
pub struct SamplerBindingLayout {
    #[napi(js_name = "type")]
    pub ty: Option<String>, // "filtering", "non-filtering", "comparison"
}

#[napi(object)]
pub struct TextureBindingLayout {
    #[napi(js_name = "sampleType")]
    pub sample_type: Option<String>, // "float", "unfilterable-float", "depth", "sint", "uint"
    #[napi(js_name = "viewDimension")]
    pub view_dimension: Option<String>, // "1d", "2d", "2d-array", "cube", "cube-array", "3d"
    pub multisampled: Option<bool>,
}

#[napi(object)]
pub struct StorageTextureBindingLayout {
    pub access: Option<String>, // "write-only", "read-only", "read-write"
    pub format: String,
    #[napi(js_name = "viewDimension")]
    pub view_dimension: Option<String>,
}

/// Render pipeline descriptor following WebGPU spec
#[napi(object)]
pub struct RenderPipelineDescriptor {
    pub label: Option<String>,
    pub vertex: VertexState,
    pub primitive: Option<PrimitiveState>,
    #[napi(js_name = "depthStencil")]
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: Option<MultisampleState>,
    pub fragment: Option<FragmentState>,
}

#[napi(object)]
pub struct VertexState {
    #[napi(js_name = "entryPoint")]
    pub entry_point: String,
    pub buffers: Option<Vec<VertexBufferLayout>>,
}

#[napi(object)]
pub struct VertexBufferLayout {
    #[napi(js_name = "arrayStride")]
    pub array_stride: i64,
    #[napi(js_name = "stepMode")]
    pub step_mode: Option<String>, // "vertex" or "instance"
    pub attributes: Vec<VertexAttribute>,
}

#[napi(object)]
pub struct VertexAttribute {
    pub format: String,
    pub offset: i64,
    #[napi(js_name = "shaderLocation")]
    pub shader_location: u32,
}

#[napi(object)]
pub struct PrimitiveState {
    pub topology: Option<String>, // "point-list", "line-list", "line-strip", "triangle-list", "triangle-strip"
    #[napi(js_name = "stripIndexFormat")]
    pub strip_index_format: Option<String>, // "uint16" or "uint32"
    #[napi(js_name = "frontFace")]
    pub front_face: Option<String>, // "ccw" or "cw"
    #[napi(js_name = "cullMode")]
    pub cull_mode: Option<String>, // "none", "front", "back"
}

#[napi(object)]
pub struct DepthStencilState {
    pub format: String,
    #[napi(js_name = "depthWriteEnabled")]
    pub depth_write_enabled: Option<bool>,
    #[napi(js_name = "depthCompare")]
    pub depth_compare: Option<String>, // "never", "less", "equal", "less-equal", "greater", "not-equal", "greater-equal", "always"
    #[napi(js_name = "stencilFront")]
    pub stencil_front: Option<StencilFaceState>,
    #[napi(js_name = "stencilBack")]
    pub stencil_back: Option<StencilFaceState>,
    #[napi(js_name = "stencilReadMask")]
    pub stencil_read_mask: Option<u32>,
    #[napi(js_name = "stencilWriteMask")]
    pub stencil_write_mask: Option<u32>,
    #[napi(js_name = "depthBias")]
    pub depth_bias: Option<i32>,
    #[napi(js_name = "depthBiasSlopeScale")]
    pub depth_bias_slope_scale: Option<f64>,
    #[napi(js_name = "depthBiasClamp")]
    pub depth_bias_clamp: Option<f64>,
}

#[napi(object)]
pub struct StencilFaceState {
    pub compare: Option<String>,
    #[napi(js_name = "failOp")]
    pub fail_op: Option<String>,
    #[napi(js_name = "depthFailOp")]
    pub depth_fail_op: Option<String>,
    #[napi(js_name = "passOp")]
    pub pass_op: Option<String>,
}

#[napi(object)]
pub struct MultisampleState {
    pub count: Option<u32>,
    pub mask: Option<u32>,
    #[napi(js_name = "alphaToCoverageEnabled")]
    pub alpha_to_coverage_enabled: Option<bool>,
}

#[napi(object)]
pub struct FragmentState {
    #[napi(js_name = "entryPoint")]
    pub entry_point: String,
    pub targets: Vec<ColorTargetState>,
}

#[napi(object)]
pub struct ColorTargetState {
    pub format: String,
    pub blend: Option<BlendState>,
    #[napi(js_name = "writeMask")]
    pub write_mask: Option<u32>,
}

#[napi(object)]
pub struct BlendState {
    pub color: BlendComponent,
    pub alpha: BlendComponent,
}

#[napi(object)]
pub struct BlendComponent {
    #[napi(js_name = "srcFactor")]
    pub src_factor: String,
    #[napi(js_name = "dstFactor")]
    pub dst_factor: String,
    pub operation: String,
}
