# Tauri Plugin Video Thumbnail

Generate video thumbnails from URLs or local file paths using the [thumbnailer](https://crates.io/crates/thumbnailer) crate.

## Installation

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-video-thumbnail = "0.1"
```

### JavaScript/TypeScript

```bash
npm install tauri-plugin-video-thumbnail
# or
pnpm add tauri-plugin-video-thumbnail
```

## Setup

1. Register the plugin in your Tauri application:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_video_thumbnail::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

2. Add permissions in your `src-tauri/capabilities/default.json`:

```json
{
  "permissions": [
    "video-thumbnail:default"
  ]
}
```

## Usage

### JavaScript/TypeScript

```typescript
import { generateThumbnail, getThumbnailDataUrl } from 'tauri-plugin-video-thumbnail'

// Generate thumbnail as base64
const result = await generateThumbnail({
  source: 'https://example.com/video.mp4',
  size: 'medium' // 'small' (32x32), 'medium' (64x64), 'large' (128x128)
})

// Use in an img element
const imgElement = document.createElement('img')
imgElement.src = `data:image/png;base64,${result.base64}`

// Or use the convenience function
const dataUrl = await getThumbnailDataUrl('https://example.com/video.mp4')
imgElement.src = dataUrl

// Save thumbnail to file
const saved = await generateThumbnail({
  source: '/path/to/local/video.mp4',
  size: { custom: { width: 200, height: 150 } },
  outputPath: '/path/to/thumbnail.png'
})
console.log(`Saved to: ${saved.path}`)
```

### Rust

```rust
use tauri_plugin_video_thumbnail::{VideoThumbnailExt, ThumbnailRequest, ThumbnailSize};

// In a command or elsewhere with access to AppHandle
let result = app.video_thumbnail()
    .generate_thumbnail(ThumbnailRequest {
        source: "https://example.com/video.mp4".to_string(),
        size: Some(ThumbnailSize::Medium),
        output_path: None,
    })
    .await?;
```

## API

### `generateThumbnail(request: ThumbnailRequest): Promise<ThumbnailResponse>`

Generate a thumbnail from a video source.

#### ThumbnailRequest

| Property | Type | Description |
|----------|------|-------------|
| `source` | `string` | Video URL (http/https) or local file path |
| `size` | `ThumbnailSize` | Optional. `'small'`, `'medium'`, `'large'`, or `{ custom: { width, height } }` |
| `outputPath` | `string` | Optional. If provided, saves thumbnail to this path instead of returning base64 |

#### ThumbnailResponse

| Property | Type | Description |
|----------|------|-------------|
| `base64` | `string \| undefined` | Base64 encoded PNG (if `outputPath` not provided) |
| `path` | `string \| undefined` | Path to saved file (if `outputPath` provided) |
| `width` | `number` | Width of generated thumbnail |
| `height` | `number` | Height of generated thumbnail |

### Thumbnail Sizes

| Size | Dimensions |
|------|------------|
| `small` | 32x32 |
| `medium` | 64x64 |
| `large` | 128x128 |
| `custom` | Any dimensions |

## Supported Video Formats

- MP4
- WebM
- AVI
- MOV
- MKV
- FLV
- WMV

## License

MIT
