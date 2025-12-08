use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::VideoThumbnailExt;

/// Generate a thumbnail from a video URL or local file path
#[command]
pub(crate) async fn generate_thumbnail<R: Runtime>(
    app: AppHandle<R>,
    request: ThumbnailRequest,
) -> Result<ThumbnailResponse> {
    app.video_thumbnail().generate_thumbnail(request).await
}
