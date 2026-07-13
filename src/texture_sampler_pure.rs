//! Pure texture/sampler descriptor SSOT (ADR-168 rust_impl deepen for native/texture-sampler).
//!
//! String-token defaults and allowlists without GPU device / napi wrappers.
//! Mirrors sampler create defaults + common texture format/dimension tokens.

/// Sampler address-mode tokens accepted by the native binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressModeToken {
    ClampToEdge,
    Repeat,
    MirrorRepeat,
}

/// Filter-mode tokens (mag/min/mipmap).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterModeToken {
    Nearest,
    Linear,
}

/// Compare-function tokens for depth/comparison samplers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareFunctionToken {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

/// Pure sampler defaults applied when JS omits optional descriptor fields.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SamplerDefaults {
    pub address_mode: AddressModeToken,
    pub mag_filter: FilterModeToken,
    pub min_filter: FilterModeToken,
    pub mipmap_filter: FilterModeToken,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub max_anisotropy: u16,
}

#[must_use]
pub fn sampler_defaults() -> SamplerDefaults {
    SamplerDefaults {
        address_mode: AddressModeToken::ClampToEdge,
        mag_filter: FilterModeToken::Nearest,
        min_filter: FilterModeToken::Nearest,
        mipmap_filter: FilterModeToken::Nearest,
        lod_min_clamp: 0.0,
        lod_max_clamp: 32.0,
        max_anisotropy: 1,
    }
}

#[must_use]
pub fn parse_address_mode_token(mode: Option<&str>) -> AddressModeToken {
    match mode {
        Some("repeat") => AddressModeToken::Repeat,
        Some("mirror-repeat") => AddressModeToken::MirrorRepeat,
        Some("clamp-to-edge") | None => AddressModeToken::ClampToEdge,
        _ => AddressModeToken::ClampToEdge,
    }
}

#[must_use]
pub fn parse_filter_mode_token(mode: Option<&str>) -> FilterModeToken {
    match mode {
        Some("linear") => FilterModeToken::Linear,
        Some("nearest") | None => FilterModeToken::Nearest,
        _ => FilterModeToken::Nearest,
    }
}

#[must_use]
pub fn parse_compare_function_token(func: Option<&str>) -> Option<CompareFunctionToken> {
    match func {
        Some("never") => Some(CompareFunctionToken::Never),
        Some("less") => Some(CompareFunctionToken::Less),
        Some("equal") => Some(CompareFunctionToken::Equal),
        Some("less-equal") => Some(CompareFunctionToken::LessEqual),
        Some("greater") => Some(CompareFunctionToken::Greater),
        Some("not-equal") => Some(CompareFunctionToken::NotEqual),
        Some("greater-equal") => Some(CompareFunctionToken::GreaterEqual),
        Some("always") => Some(CompareFunctionToken::Always),
        _ => None,
    }
}

/// Texture dimension tokens accepted by createTexture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureDimensionToken {
    D1,
    D2,
    D3,
}

#[must_use]
pub fn parse_texture_dimension_token(dim: Option<&str>) -> TextureDimensionToken {
    match dim {
        Some("1d") => TextureDimensionToken::D1,
        Some("3d") => TextureDimensionToken::D3,
        Some("2d") | None => TextureDimensionToken::D2,
        _ => TextureDimensionToken::D2,
    }
}

/// Core texture formats used by standard tests / common clients.
#[must_use]
pub fn supported_texture_formats() -> &'static [&'static str] {
    &[
        "rgba8unorm",
        "bgra8unorm",
        "rgba16float",
        "rgba32float",
        "depth24plus",
        "depth32float",
    ]
}

#[must_use]
pub fn is_supported_texture_format(format: &str) -> bool {
    supported_texture_formats().contains(&format)
}

/// Default texture view aspect token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureAspectToken {
    All,
    StencilOnly,
    DepthOnly,
}

#[must_use]
pub fn default_texture_view_aspect() -> TextureAspectToken {
    TextureAspectToken::All
}

#[must_use]
pub fn parse_texture_aspect_token(aspect: Option<&str>) -> TextureAspectToken {
    match aspect {
        Some("stencil-only") => TextureAspectToken::StencilOnly,
        Some("depth-only") => TextureAspectToken::DepthOnly,
        Some("all") | None => TextureAspectToken::All,
        _ => TextureAspectToken::All,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sampler_defaults_match_native_create_path() {
        let d = sampler_defaults();
        assert_eq!(d.address_mode, AddressModeToken::ClampToEdge);
        assert_eq!(d.mag_filter, FilterModeToken::Nearest);
        assert_eq!(d.min_filter, FilterModeToken::Nearest);
        assert_eq!(d.mipmap_filter, FilterModeToken::Nearest);
        assert_eq!(d.lod_min_clamp, 0.0);
        assert_eq!(d.lod_max_clamp, 32.0);
        assert_eq!(d.max_anisotropy, 1);
    }

    #[test]
    fn address_mode_tokens_roundtrip() {
        assert_eq!(
            parse_address_mode_token(Some("repeat")),
            AddressModeToken::Repeat
        );
        assert_eq!(
            parse_address_mode_token(Some("mirror-repeat")),
            AddressModeToken::MirrorRepeat
        );
        assert_eq!(
            parse_address_mode_token(Some("clamp-to-edge")),
            AddressModeToken::ClampToEdge
        );
        assert_eq!(parse_address_mode_token(None), AddressModeToken::ClampToEdge);
        assert_eq!(
            parse_address_mode_token(Some("unknown")),
            AddressModeToken::ClampToEdge
        );
    }

    #[test]
    fn filter_and_compare_tokens() {
        assert_eq!(
            parse_filter_mode_token(Some("linear")),
            FilterModeToken::Linear
        );
        assert_eq!(
            parse_filter_mode_token(Some("nearest")),
            FilterModeToken::Nearest
        );
        assert_eq!(parse_filter_mode_token(None), FilterModeToken::Nearest);
        assert_eq!(
            parse_compare_function_token(Some("less-equal")),
            Some(CompareFunctionToken::LessEqual)
        );
        assert_eq!(parse_compare_function_token(None), None);
        assert_eq!(parse_compare_function_token(Some("bogus")), None);
    }

    #[test]
    fn texture_dimension_and_formats() {
        assert_eq!(
            parse_texture_dimension_token(Some("3d")),
            TextureDimensionToken::D3
        );
        assert_eq!(
            parse_texture_dimension_token(None),
            TextureDimensionToken::D2
        );
        assert!(is_supported_texture_format("rgba8unorm"));
        assert!(is_supported_texture_format("depth32float"));
        assert!(!is_supported_texture_format("not-a-format"));
        assert_eq!(default_texture_view_aspect(), TextureAspectToken::All);
        assert_eq!(parse_texture_aspect_token(Some("depth-only")), TextureAspectToken::DepthOnly);
        assert_eq!(parse_texture_aspect_token(Some("stencil-only")), TextureAspectToken::StencilOnly);
        assert_eq!(parse_texture_aspect_token(None), TextureAspectToken::All);
        assert_eq!(supported_texture_formats().len(), 6);
    }
}
