use serde::{Deserialize, Serialize};

/// Size of the generated thumbnail
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThumbnailSize {
    /// 32x32 pixels
    Small,
    /// 64x64 pixels  
    #[default]
    Medium,
    /// 128x128 pixels
    Large,
    /// Custom dimensions
    Custom { width: u32, height: u32 },
}

/// Request to generate a thumbnail from a video
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailRequest {
    /// Video URL (http/https) or local file path
    pub source: String,
    /// Optional thumbnail size (defaults to Medium)
    pub size: Option<ThumbnailSize>,
    /// Optional output path. If not provided, returns base64 encoded image
    pub output_path: Option<String>,
}

/// Response containing the generated thumbnail
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailResponse {
    /// Base64 encoded PNG image data (if output_path was not specified)
    pub base64: Option<String>,
    /// Path to the saved thumbnail file (if output_path was specified)
    pub path: Option<String>,
    /// Width of the generated thumbnail
    pub width: u32,
    /// Height of the generated thumbnail
    pub height: u32,
}
