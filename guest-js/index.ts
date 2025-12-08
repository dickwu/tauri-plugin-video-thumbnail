import { invoke } from '@tauri-apps/api/core'

export type ThumbnailSize = 
  | 'small'   // 32x32
  | 'medium'  // 64x64
  | 'large'   // 128x128
  | { custom: { width: number; height: number } }

export interface ThumbnailRequest {
  /** Video URL (http/https) or local file path */
  source: string
  /** Optional thumbnail size (defaults to medium) */
  size?: ThumbnailSize
  /** Optional output path. If not provided, returns base64 encoded image */
  outputPath?: string
}

export interface ThumbnailResponse {
  /** Base64 encoded PNG image data (if outputPath was not specified) */
  base64?: string
  /** Path to the saved thumbnail file (if outputPath was specified) */
  path?: string
  /** Width of the generated thumbnail */
  width: number
  /** Height of the generated thumbnail */
  height: number
}

/**
 * Generate a thumbnail from a video URL or local file path
 * 
 * @param request - The thumbnail request configuration
 * @returns Promise containing the thumbnail response with either base64 data or file path
 * 
 * @example
 * // Get thumbnail as base64
 * const result = await generateThumbnail({
 *   source: 'https://example.com/video.mp4',
 *   size: 'medium'
 * })
 * const imgSrc = `data:image/png;base64,${result.base64}`
 * 
 * @example
 * // Save thumbnail to file
 * const result = await generateThumbnail({
 *   source: '/path/to/local/video.mp4',
 *   size: { custom: { width: 200, height: 150 } },
 *   outputPath: '/path/to/thumbnail.png'
 * })
 * console.log(`Saved to: ${result.path}`)
 */
export async function generateThumbnail(request: ThumbnailRequest): Promise<ThumbnailResponse> {
  return await invoke<ThumbnailResponse>('plugin:video-thumbnail|generate_thumbnail', {
    request
  })
}

/**
 * Convenience function to get a thumbnail as a data URL
 * 
 * @param source - Video URL or local file path
 * @param size - Optional thumbnail size
 * @returns Promise containing a data URL that can be used directly in img src
 */
export async function getThumbnailDataUrl(
  source: string, 
  size?: ThumbnailSize
): Promise<string> {
  const result = await generateThumbnail({ source, size })
  if (!result.base64) {
    throw new Error('No base64 data returned')
  }
  return `data:image/png;base64,${result.base64}`
}
