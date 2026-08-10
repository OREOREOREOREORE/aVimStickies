<script lang="ts">
  let { html, hidden }: { html: string; hidden: boolean } = $props();
</script>

<div class="preview" class:hidden={hidden}>{@html html}</div>

<style>
  .preview {
    flex: 1;
    overflow: auto;
    padding: 10px 12px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    font-size: 13px;
    line-height: 1.6;
    background: var(--panel-bg);
    color: var(--fg);
  }

  .hidden {
    display: none !important;
  }
</style>
