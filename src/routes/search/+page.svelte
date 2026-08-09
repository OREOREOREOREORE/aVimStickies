<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  type Result = { id: string; title: string; snippet: string };

  let query = $state("");
  let results = $state<Result[]>([]);
  let timer: ReturnType<typeof setTimeout> | null = null;
  let inputEl = $state<HTMLInputElement>();

  onMount(() => {
    inputEl?.focus();
    window.addEventListener("keydown", (e) => {
      if (e.key === "Escape") getCurrentWindow().close();
    });
  });

  function onInput() {
    if (timer) clearTimeout(timer);
    timer = setTimeout(() => void run(), 200);
  }

  async function run() {
    if (!query.trim()) {
      results = [];
      return;
    }
    results = await invoke<Result[]>("search_notes", { query: query.trim() });
  }

  async function open(id: string) {
    await invoke("open_note", { id });
  }
</script>

<main>
  <input
    placeholder="Search notes…"
    bind:value={query}
    oninput={onInput}
    bind:this={inputEl}
  />
  <ul>
    {#each results as r}
      <li>
        <button class="result" onclick={() => open(r.id)}>
          <span class="title">{r.title}</span>
          <span class="snippet">{r.snippet}</span>
        </button>
      </li>
    {:else}
      {#if query.trim()}
        <li class="empty">No matches</li>
      {/if}
    {/each}
  </ul>
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    font-size: 13px;
    background: #fff;
    color: #1f1f1f;
  }

  main {
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  input {
    padding: 6px 8px;
    border: 1px solid #ccc;
    border-radius: 6px;
    font-size: 14px;
  }

  ul {
    margin: 0;
    padding: 0;
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  button.result {
    display: flex;
    flex-direction: column;
    gap: 2px;
    width: 100%;
    text-align: left;
    padding: 6px 8px;
    border: none;
    border-radius: 6px;
    background: transparent;
    cursor: pointer;
  }

  button.result:hover {
    background: #eef2f7;
  }

  .title {
    font-weight: 600;
  }

  .snippet {
    color: #666;
    font-size: 12px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .empty {
    color: #999;
    padding: 8px;
  }
</style>
