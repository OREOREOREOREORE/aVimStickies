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

  type Note = { id: string; title: string; content: string; pinned: boolean; color: string };
  type Settings = {
    font_family: string;
    font_size: number;
    theme: string;
    opacity: number;
    show_preview_button: boolean;
    show_action_buttons: boolean;
    enable_color_cycle: boolean;
  };

  const COLORS: Record<string, string> = {
    yellow: "#fdf6d8",
    blue: "#d8e6fd",
    green: "#d8f5d8",
    pink: "#fdd8e6",
    purple: "#e8d8fd",
    gray: "#e6e6e6",
  };
  const COLOR_ORDER = ["yellow", "blue", "green", "pink", "purple", "gray"];

  let noteId = "";
  let note = $state<Note | null>(null);
  let error = $state("");
  let editorEl = $state<HTMLDivElement>();
  let mode = $state<"edit" | "preview">("edit");
  let html = $state("");
  let settings = $state<Settings | null>(null);
  let dirty = $state(false);
  let counts = $state({ chars: 0, lines: 0 });
  let paletteOpen = $state(false);

  let view: EditorView | null = null;
  let saveTimer: ReturnType<typeof setTimeout> | null = null;
  let reloadTimer: ReturnType<typeof setTimeout> | null = null;
  let created = false;
  let unlisteners: (() => void)[] = [];

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
    }).then((u) => unlisteners.push(u));
    void listen<Settings>("settings-changed", (e) => {
      settings = e.payload;
      applySettings();
    }).then((u) => unlisteners.push(u));
    window.addEventListener("keydown", onKeydown);

    return () => {
      if (saveTimer) clearTimeout(saveTimer);
      if (reloadTimer) clearTimeout(reloadTimer);
      unlisteners.forEach((u) => u());
      window.removeEventListener("keydown", onKeydown);
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
      const [n, s] = await Promise.all([
        invoke<Note>("get_note", { id: noteId }),
        invoke<Settings>("get_settings"),
      ]);
      note = n;
      settings = s;
      applySettings();
      setCounts(n.content);
    } catch (e) {
      error = String(e);
    }
  }

  function applySettings() {
    if (!settings) return;
    const root = document.documentElement;
    root.style.setProperty("--font-family", settings.font_family);
    root.style.setProperty("--font-size", `${settings.font_size}px`);
    root.style.opacity = String(settings.opacity);
    root.dataset.theme = settings.theme;
    root.style.setProperty("--note-bg", COLORS[note?.color ?? "yellow"] ?? "#fdf6d8");
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
            if (u.docChanged) {
              dirty = true;
              setCounts(u.state.doc.toString());
              scheduleSave();
            }
          }),
        ],
      }),
      parent: editorEl,
    });
    view.focus();
  }

  function setCounts(content: string) {
    counts = { chars: content.length, lines: content.split("\n").length };
  }

  function onKeydown(e: KeyboardEvent) {
    if (!e.metaKey) return;
    const key = e.key;

    if (key === "p") {
      e.preventDefault();
      void togglePreview();
    } else if (key === "P") {
      e.preventDefault();
      void togglePin();
    } else if (key === "Delete" || (key === "Backspace" && e.shiftKey)) {
      e.preventDefault();
      void deleteNote();
    } else if (key === ",") {
      e.preventDefault();
      void invoke("open_settings");
    } else if (key === "F") {
      e.preventDefault();
      void invoke("open_search");
    } else if ((key === "c" || key === "C") && settings?.enable_color_cycle) {
      e.preventDefault();
      cycleColor();
    } else if (key === "=" || key === "+") {
      e.preventDefault();
      changeFontSize(1);
    } else if (key === "-" || key === "_") {
      e.preventDefault();
      changeFontSize(-1);
    }
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
      dirty = false;
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
        dirty = false;
        setCounts(fresh.content);
      }
      note.title = fresh.title;
      note.pinned = fresh.pinned;
      note.color = fresh.color;
      applySettings();
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

  async function deleteNote() {
    if (saveTimer) clearTimeout(saveTimer);
    if (!confirm("Delete this note?")) return;
    await invoke("delete_note", { id: noteId });
  }

  async function closeNote() {
    if (saveTimer) clearTimeout(saveTimer);
    await save();
    getCurrentWindow().close();
  }

  async function newNote() {
    await invoke("create_note");
  }

  async function pickColor(name: string) {
    if (!note) return;
    note.color = name;
    applySettings();
    await invoke("set_note_color", { id: note.id, color: name });
    paletteOpen = false;
  }

  function cycleColor() {
    if (!note) return;
    const idx = COLOR_ORDER.indexOf(note.color);
    const next = COLOR_ORDER[(idx + 1) % COLOR_ORDER.length];
    void pickColor(next);
  }

  function changeFontSize(delta: number) {
    if (!settings) return;
    settings = { ...settings, font_size: Math.min(24, Math.max(10, settings.font_size + delta)) };
    applySettings();
    void invoke("save_settings", { newSettings: settings });
  }
</script>

<main>
  {#if error}
    <p class="error">{error}</p>
  {:else if note}
    <header data-tauri-drag-region>
      <h1 data-tauri-drag-region>{note.title}</h1>
      <div class="actions">
        {#if settings?.show_preview_button}
          <button onclick={togglePreview}>{mode === "edit" ? "Preview" : "Edit"}</button>
        {/if}
        {#if settings?.show_action_buttons}
          <button onclick={togglePin}>{note.pinned ? "Unpin" : "Pin"}</button>
          <button onclick={newNote}>New</button>
          <button onclick={deleteNote}>Del</button>
        {/if}
        <button
          class="color-dot"
          style={`background:${COLORS[note.color] ?? "#fdf6d8"}`}
          onclick={() => (paletteOpen = !paletteOpen)}
          title="Note color"
        ></button>
        <button onclick={closeNote} title="Hide note">×</button>
      </div>
    </header>

    {#if paletteOpen}
      <div class="palette">
        {#each COLOR_ORDER as name}
          <button
            class="swatch"
            class:selected={note.color === name}
            style={`background:${COLORS[name]}`}
            onclick={() => pickColor(name)}
            title={name}
          ></button>
        {/each}
      </div>
    {/if}

    <div class="editor" class:hidden={mode === "preview"} bind:this={editorEl}></div>
    <div class="preview" class:hidden={mode === "edit"}>{@html html}</div>

    <footer>
      <span class="dot" class:saved={!dirty}></span>
      <span>{counts.chars} chars · {counts.lines} lines</span>
    </footer>
  {:else}
    <p class="loading">Loading…</p>
  {/if}
</main>

<style>
  :root {
    --font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    --font-size: 13px;
    --note-bg: #fdf6d8;
    --fg: #1f1f1f;
    --panel-bg: #fffdf4;
    --header-border: rgba(0, 0, 0, 0.12);
  }

  :global([data-theme="dark"]) {
    --fg: #d4d4d4;
    --panel-bg: #1e1e1e;
    --note-bg: #2b2b2b;
    --header-border: rgba(255, 255, 255, 0.12);
  }

  :global(body) {
    margin: 0;
    background: var(--note-bg);
    color: var(--fg);
  }

  * {
    box-sizing: border-box;
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
    border-bottom: 1px solid var(--header-border);
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
    align-items: center;
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

  :global([data-theme="dark"]) button {
    color: #bbb;
  }

  :global([data-theme="dark"]) button:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .color-dot {
    width: 14px;
    height: 14px;
    padding: 0;
    border-radius: 50%;
    border: 1px solid rgba(0, 0, 0, 0.2);
  }

  .palette {
    position: absolute;
    top: 30px;
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
    width: 16px;
    height: 16px;
    padding: 0;
    border-radius: 50%;
    border: 1px solid rgba(0, 0, 0, 0.2);
  }

  .swatch.selected {
    outline: 2px solid var(--fg);
    outline-offset: 1px;
  }

  .editor {
    flex: 1;
    overflow: hidden;
    background: var(--panel-bg);
  }

  .editor :global(.cm-editor) {
    height: 100%;
    background: var(--panel-bg);
    color: var(--fg);
  }

  .editor :global(.cm-scroller) {
    font-family: var(--font-family);
    font-size: var(--font-size);
    line-height: 1.5;
  }

  :global([data-theme="dark"]) .editor :global(.cm-gutters) {
    background: #252526;
    color: #858585;
    border-right: 1px solid #3c3c3c;
  }

  :global([data-theme="dark"]) .editor :global(.cm-activeLine) {
    background: rgba(255, 255, 255, 0.05);
  }

  :global([data-theme="dark"]) .editor :global(.cm-selectionBackground) {
    background: rgba(255, 255, 255, 0.2);
  }

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

  footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 2px 8px;
    font-size: 10px;
    color: var(--fg);
    opacity: 0.7;
    border-top: 1px solid var(--header-border);
  }

  .dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #e67e22;
  }

  .dot.saved {
    background: #2ecc71;
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
