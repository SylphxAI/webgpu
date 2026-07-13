//! Pure WebGPU flag constants (ADR-168 rust_impl deepen for bindings surface).
//!
//! Mirrors `constants.rs` values without napi wrappers for unit tests / differential.

/// Buffer usage flags (WebGPU spec bitmasks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferUsageFlags {
    pub copy_src: u32,
    pub copy_dst: u32,
    pub storage: u32,
    pub uniform: u32,
    pub vertex: u32,
    pub index: u32,
    pub map_read: u32,
    pub map_write: u32,
    pub indirect: u32,
    pub query_resolve: u32,
}

#[must_use]
pub fn buffer_usage_flags() -> BufferUsageFlags {
    BufferUsageFlags {
        copy_src: 0x0004,
        copy_dst: 0x0008,
        storage: 0x0080,
        uniform: 0x0040,
        vertex: 0x0020,
        index: 0x0010,
        map_read: 0x0001,
        map_write: 0x0002,
        indirect: 0x0100,
        query_resolve: 0x0200,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapModeFlags {
    pub read: u32,
    pub write: u32,
}

#[must_use]
pub fn map_mode_flags() -> MapModeFlags {
    MapModeFlags { read: 1, write: 2 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureUsageFlags {
    pub copy_src: u32,
    pub copy_dst: u32,
    pub texture_binding: u32,
    pub storage_binding: u32,
    pub render_attachment: u32,
}

#[must_use]
pub fn texture_usage_flags() -> TextureUsageFlags {
    TextureUsageFlags {
        copy_src: 0x01,
        copy_dst: 0x02,
        texture_binding: 0x04,
        storage_binding: 0x08,
        render_attachment: 0x10,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_flags_are_distinct_powers() {
        let b = buffer_usage_flags();
        let vals = [
            b.map_read,
            b.map_write,
            b.copy_src,
            b.copy_dst,
            b.index,
            b.vertex,
            b.uniform,
            b.storage,
            b.indirect,
            b.query_resolve,
        ];
        // all nonzero unique
        for v in vals {
            assert_ne!(v, 0);
        }
        let mut s = vals.to_vec();
        s.sort_unstable();
        s.dedup();
        assert_eq!(s.len(), vals.len());
    }

    #[test]
    fn map_mode_bits() {
        let m = map_mode_flags();
        assert_eq!(m.read, 1);
        assert_eq!(m.write, 2);
    }

    #[test]
    fn texture_flags_match_spec_low_bits() {
        let t = texture_usage_flags();
        assert_eq!(t.copy_src, 1);
        assert_eq!(t.render_attachment, 0x10);
    }
}
