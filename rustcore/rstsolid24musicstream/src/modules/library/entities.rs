//! Library-related data structures

/// Audio metadata
#[derive(Debug, Clone)]
pub struct AudioMetadata {
    pub bitrate: u32,
    pub sample_rate: u32,
    pub channels: u8,
    pub format: String,
}
