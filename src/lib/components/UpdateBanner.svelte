<script lang="ts">
  let { version, onInstall }: { version: string; onInstall: () => void } = $props();
</script>

<div class="banner">
  <span>Update to v{version} available</span>
  <button onclick={onInstall}>Install</button>
</div>

<style>
  .banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 4px 8px;
    font-size: 12px;
    background: rgba(26, 115, 232, 0.12);
    color: var(--fg);
    border-bottom: 1px solid var(--header-border);
  }

  .banner button {
    border: 1px solid var(--header-border);
    background: rgba(26, 115, 232, 0.15);
    color: var(--fg);
    border-radius: 4px;
    padding: 1px 8px;
    font-size: 12px;
  }
</style>
