use napi_derive::napi;

/// Buffer usage flags
#[napi]
pub mod buffer_usage {
    /// Buffer can be used as a source for copy operations
    pub const COPY_SRC: u32 = 0x0004;
    /// Buffer can be used as a destination for copy operations
    pub const COPY_DST: u32 = 0x0008;
    /// Buffer can be used as a storage buffer
    pub const STORAGE: u32 = 0x0080;
    /// Buffer can be used as a uniform buffer
    pub const UNIFORM: u32 = 0x0040;
    /// Buffer can be used as a vertex buffer
    pub const VERTEX: u32 = 0x0020;
    /// Buffer can be used as an index buffer
    pub const INDEX: u32 = 0x0010;
    /// Buffer can be mapped for reading
    pub const MAP_READ: u32 = 0x0001;
    /// Buffer can be mapped for writing
    pub const MAP_WRITE: u32 = 0x0002;
}

/// Map mode flags
#[napi]
pub mod map_mode {
    /// Map for reading
    pub const READ: u32 = 1;
    /// Map for writing
    pub const WRITE: u32 = 2;
}
