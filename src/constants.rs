use napi_derive::napi;

/// Buffer usage flags object
#[napi(object)]
pub struct BufferUsage {
    pub copy_src: u32,
    pub copy_dst: u32,
    pub storage: u32,
    pub uniform: u32,
    pub vertex: u32,
    pub index: u32,
    pub map_read: u32,
    pub map_write: u32,
}

/// Get buffer usage constants
#[napi]
pub fn buffer_usage() -> BufferUsage {
    BufferUsage {
        copy_src: 0x0004,
        copy_dst: 0x0008,
        storage: 0x0080,
        uniform: 0x0040,
        vertex: 0x0020,
        index: 0x0010,
        map_read: 0x0001,
        map_write: 0x0002,
    }
}

/// Map mode object
#[napi(object)]
pub struct MapMode {
    pub read: u32,
    pub write: u32,
}

/// Get map mode constants
#[napi]
pub fn map_mode() -> MapMode {
    MapMode {
        read: 1,
        write: 2,
    }
}

/// Texture usage flags object
#[napi(object)]
pub struct TextureUsage {
    pub copy_src: u32,
    pub copy_dst: u32,
    pub texture_binding: u32,
    pub storage_binding: u32,
    pub render_attachment: u32,
}

/// Get texture usage constants
#[napi]
pub fn texture_usage() -> TextureUsage {
    TextureUsage {
        copy_src: 0x01,
        copy_dst: 0x02,
        texture_binding: 0x04,
        storage_binding: 0x08,
        render_attachment: 0x10,
    }
}
