/// Helper functions for parsing string descriptors into wgpu types
///
/// These functions convert JavaScript-friendly string formats into
/// strongly-typed wgpu enums and structures.

/// Parse texture format string
pub(crate) fn parse_texture_format(format: &str) -> wgpu::TextureFormat {
    match format {
        "rgba8unorm" => wgpu::TextureFormat::Rgba8Unorm,
        "bgra8unorm" => wgpu::TextureFormat::Bgra8Unorm,
        "rgba16float" => wgpu::TextureFormat::Rgba16Float,
        "rgba32float" => wgpu::TextureFormat::Rgba32Float,
        "depth24plus" => wgpu::TextureFormat::Depth24Plus,
        "depth32float" => wgpu::TextureFormat::Depth32Float,
        _ => wgpu::TextureFormat::Rgba8Unorm,
    }
}

/// Parse vertex format string
pub(crate) fn parse_vertex_format(format: &str) -> wgpu::VertexFormat {
    match format {
        "float32" => wgpu::VertexFormat::Float32,
        "float32x2" => wgpu::VertexFormat::Float32x2,
        "float32x3" => wgpu::VertexFormat::Float32x3,
        "float32x4" => wgpu::VertexFormat::Float32x4,
        "uint32" => wgpu::VertexFormat::Uint32,
        "sint32" => wgpu::VertexFormat::Sint32,
        _ => wgpu::VertexFormat::Float32x3,
    }
}

/// Parse blend mode string into blend state
pub(crate) fn parse_blend_mode(mode: &str) -> wgpu::BlendState {
    match mode {
        "replace" => wgpu::BlendState::REPLACE,
        "alpha" => wgpu::BlendState::ALPHA_BLENDING,
        "additive" => wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        },
        "premultiplied" => wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING,
        _ => wgpu::BlendState::REPLACE,
    }
}

/// Parse address mode for samplers
pub(crate) fn parse_address_mode(mode: Option<&String>) -> wgpu::AddressMode {
    match mode.map(|s| s.as_str()) {
        Some("clamp-to-edge") => wgpu::AddressMode::ClampToEdge,
        Some("repeat") => wgpu::AddressMode::Repeat,
        Some("mirror-repeat") => wgpu::AddressMode::MirrorRepeat,
        _ => wgpu::AddressMode::ClampToEdge,
    }
}

/// Parse filter mode for samplers
pub(crate) fn parse_filter_mode(mode: Option<&String>) -> wgpu::FilterMode {
    match mode.map(|s| s.as_str()) {
        Some("linear") => wgpu::FilterMode::Linear,
        Some("nearest") => wgpu::FilterMode::Nearest,
        _ => wgpu::FilterMode::Nearest,
    }
}

/// Parse compare function for samplers
pub(crate) fn parse_compare_function(func: Option<&String>) -> Option<wgpu::CompareFunction> {
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
