<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type Note = { id: string; title: string; content: string };

  let noteId = "";
  let note = $state<Note | null>(null);
  let error = $state("");

  async function load() {
    const params = new URLSearchParams(window.location.search);
    noteId = params.get("note") ?? "";
    if (!noteId) return;
    try {
      note = await invoke<Note>("get_note", { id: noteId });
    } catch (e) {
      error = String(e);
    }
  }

  load();

  async function newNote() {
    await invoke("create_note");
  }

  async function deleteNote() {
    await invoke("delete_note", { id: noteId });
  }
</script>

<main>
  {#if error}
    <p class="error">{error}</p>
  {:else if note}
    <header>
      <h1>{note.title}</h1>
      <div class="actions">
        <button onclick={newNote} title="New note">＋</button>
        <button onclick={deleteNote} title="Delete note">🗑</button>
      </div>
    </header>
    <pre>{note.content || "(empty — open in neovim to write)"}</pre>
  {:else}
    <p class="loading">Loading…</p>
  {/if}
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    font-size: 13px;
    color: #1f1f1f;
  }

  * {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    background: #fdf6d8;
  }

  main {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 8px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.12);
    -webkit-user-select: none;
    user-select: none;
  }

  h1 {
    margin: 0;
    font-size: 12px;
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .actions {
    display: flex;
    gap: 2px;
  }

  button {
    border: none;
    background: transparent;
    border-radius: 4px;
    font-size: 12px;
    line-height: 1;
    padding: 4px 6px;
    cursor: pointer;
    color: #555;
  }

  button:hover {
    background: rgba(0, 0, 0, 0.08);
  }

  pre {
    margin: 0;
    padding: 8px;
    flex: 1;
    overflow: auto;
    white-space: pre-wrap;
    word-wrap: break-word;
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 12px;
    line-height: 1.5;
    color: #333;
  }

  .error {
    color: #b00020;
    padding: 8px;
  }

  .loading {
    padding: 8px;
    color: #888;
  }
</style>
