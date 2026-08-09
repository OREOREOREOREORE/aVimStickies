<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { listen } from "@tauri-apps/api/event";
  import { EditorView } from "@codemirror/view";
  import { EditorState } from "@codemirror/state";
  import { basicSetup } from "codemirror";
  import { markdown } from "@codemirror/lang-markdown";
  import { vim } from "@replit/codemirror-vim";

  type Note = { id: string; title: string; content: string; pinned: boolean };

  let noteId = "";
  let note = $state<Note | null>(null);
  let error = $state("");
  let editorEl = $state<HTMLDivElement>();
  let mode = $state<"edit" | "preview">("edit");
  let html = $state("");

  let view: EditorView | null = null;
  let saveTimer: ReturnType<typeof setTimeout> | null = null;
  let reloadTimer: ReturnType<typeof setTimeout> | null = null;
  let created = false;
  let unlisten: (() => void) | null = null;

  function titleOf(content: string): string {
    const first = content
      .split("\n")
      .map((l) => l.trim())
      .find((l) => l.length > 0);
    return first || noteId;
  }

  onMount(() => {
    void load();
    void listen<string>("note-changed", (e) => {
      if (e.payload === noteId) scheduleReload();
    }).then((u) => (unlisten = u));

    return () => {
      if (saveTimer) clearTimeout(saveTimer);
      if (reloadTimer) clearTimeout(reloadTimer);
      unlisten?.();
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

  function scheduleReload() {
    if (reloadTimer) clearTimeout(reloadTimer);
    reloadTimer = setTimeout(() => void reloadFromDisk(), 150);
  }

  async function reloadFromDisk() {
    if (!note) return;
    try {
      const fresh = await invoke<Note>("get_note", { id: note.id });
      const current = view?.state.doc.toString() ?? "";
      if (view && fresh.content !== current) {
        view.dispatch({ changes: { from: 0, to: current.length, insert: fresh.content } });
      }
      note.title = fresh.title;
      note.pinned = fresh.pinned;
      if (mode === "preview") {
        html = await invoke<string>("render_markdown", { content: fresh.content });
      }
    } catch (e) {
      console.error("reload failed", e);
    }
  }

  async function togglePreview() {
    if (!view) return;
    if (mode === "edit") {
      html = await invoke<string>("render_markdown", { content: view.state.doc.toString() });
      mode = "preview";
    } else {
      mode = "edit";
      view.requestMeasure();
      view.focus();
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

  async function togglePin() {
    if (!note) return;
    const next = !note.pinned;
    note.pinned = next;
    try {
      await invoke("set_pinned", { id: note.id, pinned: next });
    } catch (e) {
      console.error("pin failed", e);
      note.pinned = !next;
    }
  }
</script>

<main>
  {#if error}
    <p class="error">{error}</p>
  {:else if note}
    <header data-tauri-drag-region>
      <h1 data-tauri-drag-region>{note.title}</h1>
      <div class="actions">
        <button onclick={togglePreview} title="Toggle preview">
          {mode === "edit" ? "👁" : "✏️"}
        </button>
        <button class:active={note.pinned} onclick={togglePin} title="Pin on top">📌</button>
        <button onclick={newNote} title="New note">＋</button>
        <button onclick={deleteNote} title="Delete note">🗑</button>
        <button onclick={closeNote} title="Hide note">×</button>
      </div>
    </header>
    <div class="editor" class:hidden={mode === "preview"} bind:this={editorEl}></div>
    <div class="preview" class:hidden={mode === "edit"}>{@html html}</div>
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

  button.active {
    color: #1a73e8;
    background: rgba(26, 115, 232, 0.12);
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

  .preview {
    flex: 1;
    overflow: auto;
    padding: 10px 12px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    font-size: 13px;
    line-height: 1.6;
    color: #222;
  }

  .hidden {
    display: none !important;
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
