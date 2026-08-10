<script lang="ts">
  import { COLORS, COLOR_ORDER, colorOf } from "$lib/colors";

  let {
    title,
    color,
    pinned,
    mode,
    paletteOpen,
    showPreviewButton,
    showActionButtons,
    onPreview,
    onPin,
    onNew,
    onDelete,
    onClose,
    onPickColor,
    onTogglePalette,
    onCollapse,
  }: {
    title: string;
    color: string;
    pinned: boolean;
    mode: "edit" | "preview";
    paletteOpen: boolean;
    showPreviewButton: boolean;
    showActionButtons: boolean;
    onPreview: () => void;
    onPin: () => void;
    onNew: () => void;
    onDelete: () => void;
    onClose: () => void;
    onPickColor: (name: string) => void;
    onTogglePalette: () => void;
    onCollapse: () => void;
  } = $props();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<header
  data-tauri-drag-region
  ondblclick={(e) => {
    if (!(e.target as HTMLElement).closest("button")) onCollapse();
  }}
>
  <h1 data-tauri-drag-region>{title}</h1>
  <div class="actions">
    {#if showPreviewButton}
      <button onclick={onPreview}>{mode === "edit" ? "Preview" : "Edit"}</button>
    {/if}
    {#if showActionButtons}
      <button onclick={onPin}>{pinned ? "Unpin" : "Pin"}</button>
      <button onclick={onNew}>New</button>
      <button onclick={onDelete}>Del</button>
    {/if}
    <button
      class="color-dot"
      style={`background:${colorOf({ color })}`}
      onclick={onTogglePalette}
      title="Note color"
    ></button>
    <button onclick={onClose} title="Hide note">×</button>
  </div>
</header>

{#if paletteOpen}
  <div class="palette">
    {#each COLOR_ORDER as name}
      <button
        class="swatch"
        class:selected={color === name}
        style={`background:${COLORS[name]}`}
        onclick={() => onPickColor(name)}
        title={name}
      ></button>
    {/each}
  </div>
{/if}

<style>
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1px 8px;
    border-bottom: 1px solid var(--header-border);
    -webkit-user-select: none;
    user-select: none;
  }

  h1 {
    margin: 0;
    font-size: 12px;
    line-height: 1;
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 2px;
  }

  button {
    border: none;
    background: transparent;
    border-radius: 4px;
    font-size: 10px;
    line-height: 1;
    padding: 1px 2px;
    cursor: pointer;
    color: #555;
  }

  button:hover {
    background: rgba(0, 0, 0, 0.08);
  }

  :global([data-theme="dark"]) button {
    color: #bbb;
  }

  :global([data-theme="dark"]) button:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .color-dot {
    width: 8px;
    height: 8px;
    padding: 0;
    border-radius: 50%;
    border: 1px solid rgba(0, 0, 0, 0.2);
  }

  .palette {
    position: absolute;
    top: 24px;
    right: 8px;
    display: flex;
    gap: 4px;
    padding: 6px;
    background: var(--panel-bg);
    border: 1px solid var(--header-border);
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    z-index: 10;
  }

  .swatch {
    width: 14px;
    height: 14px;
    padding: 0;
    border-radius: 50%;
    border: 1px solid rgba(0, 0, 0, 0.2);
  }

  .swatch.selected {
    outline: 2px solid var(--fg);
    outline-offset: 1px;
  }
</style>
