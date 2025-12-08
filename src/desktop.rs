use base64::{engine::general_purpose::STANDARD, Engine};
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{BufReader, Cursor, Write};
use std::path::Path;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use thumbnailer::{create_thumbnails, ThumbnailSize as ThumbSize};

use crate::models::*;
use crate::{Error, Result};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<VideoThumbnail<R>> {
    Ok(VideoThumbnail(app.clone()))
}

/// Access to the video-thumbnail APIs.
pub struct VideoThumbnail<R: Runtime>(AppHandle<R>);

impl<R: Runtime> VideoThumbnail<R> {
    /// Generate a thumbnail from a video URL or local file path
    pub async fn generate_thumbnail(&self, request: ThumbnailRequest) -> Result<ThumbnailResponse> {
        let video_data = self.fetch_video_data(&request.source).await?;
        let mime_type = Self::detect_mime_type(&request.source);

        let thumb_size = match request.size.unwrap_or_default() {
            ThumbnailSize::Small => ThumbSize::Small,
            ThumbnailSize::Medium => ThumbSize::Medium,
            ThumbnailSize::Large => ThumbSize::Large,
            ThumbnailSize::Custom { width, height } => ThumbSize::Custom((width, height)),
        };

        // Create thumbnail from video data
        let reader = BufReader::new(Cursor::new(&video_data));
        let thumbnails = create_thumbnails(reader, mime_type, [thumb_size])
            .map_err(|e| Error::Thumbnail(e.to_string()))?;

        let thumbnail = thumbnails
            .into_iter()
            .next()
            .ok_or_else(|| Error::Thumbnail("No thumbnail generated".to_string()))?;

        // Get dimensions
        let (width, height) = thumbnail.size();

        // Convert to PNG bytes using a Cursor that implements Seek
        let mut png_cursor = Cursor::new(Vec::new());
        thumbnail
            .write_png(&mut png_cursor)
            .map_err(|e| Error::Thumbnail(e.to_string()))?;
        let png_data = png_cursor.into_inner();

        // Either save to file or return base64
        if let Some(output_path) = request.output_path {
            let mut file = File::create(&output_path)?;
            file.write_all(&png_data)?;
            Ok(ThumbnailResponse {
                base64: None,
                path: Some(output_path),
                width,
                height,
            })
        } else {
            let base64_data = STANDARD.encode(&png_data);
            Ok(ThumbnailResponse {
                base64: Some(base64_data),
                path: None,
                width,
                height,
            })
        }
    }

    /// Fetch video data from URL or local file
    async fn fetch_video_data(&self, source: &str) -> Result<Vec<u8>> {
        if source.starts_with("http://") || source.starts_with("https://") {
            // Download from URL
            let response = reqwest::get(source).await?;
            if !response.status().is_success() {
                return Err(Error::Http(response.error_for_status().unwrap_err()));
            }
            let bytes = response.bytes().await?;
            Ok(bytes.to_vec())
        } else {
            // Read from local file
            let path = Path::new(source);
            if !path.exists() {
                return Err(Error::InvalidSource(format!("File not found: {}", source)));
            }
            let data = tokio::fs::read(path).await?;
            Ok(data)
        }
    }

    /// Detect MIME type from file extension or URL
    fn detect_mime_type(source: &str) -> mime::Mime {
        let lower = source.to_lowercase();
        let default: mime::Mime = "video/*".parse().unwrap();

        if lower.ends_with(".mp4") || lower.contains(".mp4") {
            "video/mp4".parse().unwrap_or(default)
        } else if lower.ends_with(".webm") {
            "video/webm".parse().unwrap_or(default)
        } else if lower.ends_with(".avi") {
            "video/x-msvideo".parse().unwrap_or(default)
        } else if lower.ends_with(".mov") {
            "video/quicktime".parse().unwrap_or(default)
        } else if lower.ends_with(".mkv") {
            "video/x-matroska".parse().unwrap_or(default)
        } else if lower.ends_with(".flv") {
            "video/x-flv".parse().unwrap_or(default)
        } else if lower.ends_with(".wmv") {
            "video/x-ms-wmv".parse().unwrap_or(default)
        } else {
            // Default to generic video
            default
        }
    }
}
