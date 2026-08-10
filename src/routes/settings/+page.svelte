<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";

  type Settings = {
    font_family: string;
    font_size: number;
    theme: string;
    opacity: number;
    show_preview_button: boolean;
    show_action_buttons: boolean;
    enable_color_cycle: boolean;
    show_status_bar: boolean;
    show_line_numbers: boolean;
    wrap_text: boolean;
  };

  const FONT_PRESETS = [
    "ui-monospace, SFMono-Regular, Menlo, monospace",
    "Menlo, Monaco, monospace",
    "JetBrains Mono, monospace",
    "Fira Code, monospace",
    "Courier New, monospace",
  ];

  let settings = $state<Settings | null>(null);
  let unlisteners: (() => void)[] = [];

  onMount(() => {
    void invoke<Settings>("get_settings").then((s) => (settings = s));
    void listen<Settings>("settings-changed", (e) => (settings = e.payload)).then((u) =>
      unlisteners.push(u)
    );
    return () => unlisteners.forEach((u) => u());
  });

  function save() {
    if (!settings) return;
    void invoke("save_settings", { newSettings: settings });
  }

  function set<K extends keyof Settings>(key: K, value: Settings[K]) {
    if (!settings) return;
    settings = { ...settings, [key]: value };
    save();
  }
</script>

<main>
  <h1>aVimStickies Settings</h1>

  {#if settings}
    <label>
      <span>Font family</span>
      <select value={settings.font_family} onchange={(e) => set("font_family", e.currentTarget.value)}>
        {#each FONT_PRESETS as f}
          <option value={f}>{f.split(",")[0].trim()}</option>
        {/each}
      </select>
    </label>

    <label>
      <span>Font size</span>
      <div class="row">
        <button onclick={() => set("font_size", Math.max(10, settings!.font_size - 1))}>−</button>
        <input
          type="number"
          min="10"
          max="24"
          value={settings.font_size}
          onchange={(e) => set("font_size", Number(e.currentTarget.value))}
        />
        <button onclick={() => set("font_size", Math.min(24, settings!.font_size + 1))}>＋</button>
      </div>
    </label>

    <label>
      <span>Theme</span>
      <select value={settings.theme} onchange={(e) => set("theme", e.currentTarget.value)}>
        <option value="light">Light</option>
        <option value="dark">Dark</option>
      </select>
    </label>

    <label>
      <span>Window opacity ({Math.round(settings.opacity * 100)}%)</span>
      <input
        type="range"
        min="0.6"
        max="1"
        step="0.05"
        value={settings.opacity}
        oninput={(e) => set("opacity", Number(e.currentTarget.value))}
      />
    </label>

    <label class="check">
      <input
        type="checkbox"
        checked={settings.show_preview_button}
        onchange={(e) => set("show_preview_button", e.currentTarget.checked)}
      />
      <span>Show preview button in note header</span>
    </label>

    <label class="check">
      <input
        type="checkbox"
        checked={settings.show_action_buttons}
        onchange={(e) => set("show_action_buttons", e.currentTarget.checked)}
      />
      <span>Show Pin / New / Delete buttons in note header</span>
    </label>

    <label class="check">
      <input
        type="checkbox"
        checked={settings.enable_color_cycle}
        onchange={(e) => set("enable_color_cycle", e.currentTarget.checked)}
      />
      <span>Enable Cmd+Shift+C to cycle note colors</span>
    </label>

    <label class="check">
      <input
        type="checkbox"
        checked={settings.show_status_bar}
        onchange={(e) => set("show_status_bar", e.currentTarget.checked)}
      />
      <span>Show status bar (auto-save dot, char count)</span>
    </label>

    <label class="check">
      <input
        type="checkbox"
        checked={settings.show_line_numbers}
        onchange={(e) => set("show_line_numbers", e.currentTarget.checked)}
      />
      <span>Show line numbers in the editor</span>
    </label>

    <label class="check">
      <input
        type="checkbox"
        checked={settings.wrap_text}
        onchange={(e) => set("wrap_text", e.currentTarget.checked)}
      />
      <span>Wrap long lines in the editor</span>
    </label>
    <p class="hint">
      <b>Keybinds: <br></b>
      - Cmd+P preview<br>
      - Cmd+Shift+P pin<br>
      - Cmd+Delete delete<br>
      - Cmd+= / Cmd+- font size<br>
      - Cmd+, settings <br>
      - Cmd+Shift+F search <br>
      - Cmd+N new note <br>
    </p>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    font-size: 13px;
    background: #f6f6f6;
    color: #1f1f1f;
  }

  main {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  h1 {
    margin: 0 0 4px;
    font-size: 15px;
  }

  label {
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-weight: 500;
  }

  label.check {
    flex-direction: row;
    align-items: center;
    gap: 6px;
    font-weight: 400;
  }

  .row {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  select,
  input[type="number"] {
    padding: 4px 6px;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 13px;
  }

  input[type="range"] {
    width: 100%;
  }

  input[type="number"]::-webkit-outer-spin-button,
  input[type="number"]::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  input[type="number"] {
    -moz-appearance: textfield;
    appearance: textfield;
  }

  button {
    padding: 4px 10px;
    border: 1px solid #ccc;
    border-radius: 4px;
    background: #fff;
    cursor: pointer;
    font-size: 13px;
  }

  .hint {
    color: #777;
    font-size: 11px;
    line-height: 1.6;
  }
</style>
