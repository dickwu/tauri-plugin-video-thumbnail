<script>
  import { generateThumbnail, getThumbnailDataUrl } from 'tauri-plugin-video-thumbnail-api'

  let videoUrl = $state('https://www.w3schools.com/html/mov_bbb.mp4')
  let thumbnailSrc = $state('')
  let loading = $state(false)
  let error = $state('')
  let dimensions = $state({ width: 0, height: 0 })

  async function handleGenerateThumbnail() {
    loading = true
    error = ''
    thumbnailSrc = ''

    try {
      const result = await generateThumbnail({
        source: videoUrl,
        size: 'large'
      })

      if (result.base64) {
        thumbnailSrc = `data:image/png;base64,${result.base64}`
        dimensions = { width: result.width, height: result.height }
      }
    } catch (e) {
      error = e.toString()
    } finally {
      loading = false
    }
  }
</script>

<main class="container">
  <h1>Video Thumbnail Generator</h1>

  <div class="row">
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
  </div>

  <div class="form">
    <input
      type="text"
      placeholder="Enter video URL or local path"
      bind:value={videoUrl}
    />
    <button onclick={handleGenerateThumbnail} disabled={loading}>
      {loading ? 'Generating...' : 'Generate Thumbnail'}
    </button>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if thumbnailSrc}
    <div class="result">
      <h3>Generated Thumbnail ({dimensions.width}x{dimensions.height})</h3>
      <img src={thumbnailSrc} alt="Video thumbnail" class="thumbnail" />
    </div>
  {/if}
</main>

<style>
  .container {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
    text-align: center;
  }

  .form {
    display: flex;
    gap: 1rem;
    margin: 2rem 0;
    flex-wrap: wrap;
    justify-content: center;
  }

  input {
    flex: 1;
    min-width: 300px;
    padding: 0.75rem;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 1rem;
  }

  button {
    padding: 0.75rem 1.5rem;
    background-color: #ffc131;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
    font-weight: bold;
    transition: background-color 0.2s;
  }

  button:hover:not(:disabled) {
    background-color: #ffaa00;
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .error {
    color: #ff4444;
    margin: 1rem 0;
    padding: 1rem;
    background-color: #ffeeee;
    border-radius: 4px;
  }

  .result {
    margin-top: 2rem;
  }

  .thumbnail {
    max-width: 100%;
    border: 2px solid #ccc;
    border-radius: 8px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 0.3s;
  }

  .logo:hover {
    filter: drop-shadow(0 0 2em #ffc131);
  }
</style>
