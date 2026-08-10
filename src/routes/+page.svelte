<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { listen } from "@tauri-apps/api/event";
  import { LogicalSize } from "@tauri-apps/api/dpi";
  import { createNoteEditor, type NoteEditor } from "$lib/editor";
  import { COLORS, DEFAULT_COLOR, nextColor } from "$lib/colors";
  import type { Note, Settings } from "$lib/types";
  import NoteHeader from "$lib/components/NoteHeader.svelte";
  import PreviewPane from "$lib/components/PreviewPane.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import UpdateBanner from "$lib/components/UpdateBanner.svelte";

  let noteId = "";
  let note = $state<Note | null>(null);
  let error = $state("");
  let editorEl = $state<HTMLDivElement>();
  let mode = $state<"edit" | "preview">("edit");
  let html = $state("");
  let updateVersion = $state("");
  let settings = $state<Settings | null>(null);
  let dirty = $state(false);
  let counts = $state({ chars: 0, lines: 0 });
  let paletteOpen = $state(false);
  let collapsed = $state(false);

  let view: NoteEditor | null = null;
  let saveTimer: ReturnType<typeof setTimeout> | null = null;
  let reloadTimer: ReturnType<typeof setTimeout> | null = null;
  let created = false;
  let unlisteners: (() => void)[] = [];
  let savedSize: LogicalSize | null = null;

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
      view?.applyLineNumbers(settings.show_line_numbers);
    }).then((u) => unlisteners.push(u));
    void listen<string>("update-available", (e) => {
      updateVersion = e.payload;
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
    root.style.setProperty("--note-bg", COLORS[note?.color ?? DEFAULT_COLOR] ?? COLORS[DEFAULT_COLOR]);
  }

  function createEditor(doc: string) {
    if (!editorEl) return;
    view = createNoteEditor(editorEl, {
      doc,
      showLineNumbers: settings?.show_line_numbers ?? true,
      onChange: (content) => {
        dirty = true;
        setCounts(content);
        scheduleSave();
      },
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
    const content = view.getContent();
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
      if (view && !dirty) {
        view.setDoc(fresh.content);
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
      html = await invoke<string>("render_markdown", { content: view.getContent() });
      view.setReadOnly(true);
      (document.activeElement as HTMLElement | null)?.blur();
      mode = "preview";
    } else {
      mode = "edit";
      view.setReadOnly(false);
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

  async function installUpdate() {
    await invoke("install_update");
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
    void pickColor(nextColor(note.color));
  }

  function changeFontSize(delta: number) {
    if (!settings) return;
    settings = { ...settings, font_size: Math.min(24, Math.max(10, settings.font_size + delta)) };
    applySettings();
    void invoke("save_settings", { newSettings: settings });
  }

  async function toggleCollapse() {
    if (collapsed) await expand();
    else await collapse();
  }

  async function collapse() {
    const win = getCurrentWindow();
    const phys = await win.innerSize();
    const scale = await win.scaleFactor();
    savedSize = new LogicalSize(phys.width / scale, phys.height / scale);
    const headerH = document.querySelector("header")?.clientHeight ?? 24;
    await win.setMinSize(new LogicalSize(100, 20));
    await win.setSize(new LogicalSize(savedSize.width, headerH + 2));
    collapsed = true;
  }

  async function expand() {
    if (!savedSize) return;
    const win = getCurrentWindow();
    await win.setSize(savedSize);
    await win.setMinSize(new LogicalSize(220, 160));
    collapsed = false;
    view?.requestMeasure();
  }
</script>

<main>
  {#if error}
    <p class="error">{error}</p>
  {:else if note}
    {#if updateVersion}
      <UpdateBanner version={updateVersion} onInstall={installUpdate} />
    {/if}
    <NoteHeader
      title={note.title}
      color={note.color}
      pinned={note.pinned}
      mode={mode}
      paletteOpen={paletteOpen}
      showPreviewButton={settings?.show_preview_button ?? false}
      showActionButtons={settings?.show_action_buttons ?? false}
      onPreview={togglePreview}
      onPin={togglePin}
      onNew={newNote}
      onDelete={deleteNote}
      onClose={closeNote}
      onPickColor={pickColor}
      onTogglePalette={() => (paletteOpen = !paletteOpen)}
      onCollapse={toggleCollapse}
    />
    <div class="editor" class:hidden={mode === "preview" || collapsed} bind:this={editorEl}></div>
    <PreviewPane html={html} hidden={mode === "edit" || collapsed} />
    {#if settings?.show_status_bar && !collapsed}
      <StatusBar dirty={dirty} chars={counts.chars} lines={counts.lines} />
    {/if}
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
