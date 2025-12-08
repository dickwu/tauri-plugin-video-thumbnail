use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;
use crate::Result;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_video_thumbnail);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> Result<VideoThumbnail<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("", "VideoThumbnailPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_video_thumbnail)?;
    Ok(VideoThumbnail(handle))
}

/// Access to the video-thumbnail APIs.
pub struct VideoThumbnail<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> VideoThumbnail<R> {
    pub async fn generate_thumbnail(&self, request: ThumbnailRequest) -> Result<ThumbnailResponse> {
        self.0
            .run_mobile_plugin("generateThumbnail", request)
            .map_err(Into::into)
    }
}
