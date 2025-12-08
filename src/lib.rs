use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::VideoThumbnail;
#[cfg(mobile)]
use mobile::VideoThumbnail;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the video-thumbnail APIs.
pub trait VideoThumbnailExt<R: Runtime> {
    fn video_thumbnail(&self) -> &VideoThumbnail<R>;
}

impl<R: Runtime, T: Manager<R>> crate::VideoThumbnailExt<R> for T {
    fn video_thumbnail(&self) -> &VideoThumbnail<R> {
        self.state::<VideoThumbnail<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("video-thumbnail")
        .invoke_handler(tauri::generate_handler![commands::generate_thumbnail])
        .setup(|app, api| {
            #[cfg(mobile)]
            let video_thumbnail = mobile::init(app, api)?;
            #[cfg(desktop)]
            let video_thumbnail = desktop::init(app, api)?;
            app.manage(video_thumbnail);
            Ok(())
        })
        .build()
}
