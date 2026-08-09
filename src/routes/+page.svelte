<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { EditorView } from "@codemirror/view";
  import { EditorState } from "@codemirror/state";
  import { basicSetup } from "codemirror";
  import { markdown } from "@codemirror/lang-markdown";
  import { vim } from "@replit/codemirror-vim";

  type Note = { id: string; title: string; content: string };

  let noteId = "";
  let note = $state<Note | null>(null);
  let error = $state("");
  let editorEl = $state<HTMLDivElement>();

  let view: EditorView | null = null;
  let saveTimer: ReturnType<typeof setTimeout> | null = null;
  let created = false;

  function titleOf(content: string): string {
    const first = content
      .split("\n")
      .map((l) => l.trim())
      .find((l) => l.length > 0);
    return first || noteId;
  }

  onMount(() => {
    void load();
    return () => {
      if (saveTimer) clearTimeout(saveTimer);
      view?.destroy();
    };
  });

  $effect(() => {
    if (!created && note && editorEl) {
      created = true;
      createEditor(note.content);
    }
  });

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

  function createEditor(doc: string) {
    if (!editorEl) return;
    view = new EditorView({
      state: EditorState.create({
        doc,
        extensions: [
          basicSetup,
          markdown(),
          vim(),
          EditorView.updateListener.of((u) => {
            if (u.docChanged) scheduleSave();
          }),
        ],
      }),
      parent: editorEl,
    });
    view.focus();
  }

  function scheduleSave() {
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(() => void save(), 500);
  }

  async function save() {
    if (!view || !note) return;
    const content = view.state.doc.toString();
    try {
      await invoke("save_note", { id: note.id, content });
      note.title = titleOf(content);
    } catch (e) {
      console.error("save failed", e);
    }
  }

  async function newNote() {
    await invoke("create_note");
  }

  async function deleteNote() {
    if (saveTimer) clearTimeout(saveTimer);
    await invoke("delete_note", { id: noteId });
  }

  async function closeNote() {
    if (saveTimer) clearTimeout(saveTimer);
    await save();
    getCurrentWindow().close();
  }
</script>

<main>
  {#if error}
    <p class="error">{error}</p>
  {:else if note}
    <header data-tauri-drag-region>
      <h1 data-tauri-drag-region>{note.title}</h1>
      <div class="actions">
        <button onclick={newNote} title="New note">＋</button>
        <button onclick={deleteNote} title="Delete note">🗑</button>
        <button onclick={closeNote} title="Close note">×</button>
      </div>
    </header>
    <div class="editor" bind:this={editorEl}></div>
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

  .editor {
    flex: 1;
    overflow: hidden;
    background: #fffdf4;
  }

  .editor :global(.cm-editor) {
    height: 100%;
  }

  .editor :global(.cm-scroller) {
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-size: 12px;
    line-height: 1.5;
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
