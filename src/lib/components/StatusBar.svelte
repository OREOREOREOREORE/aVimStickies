<script lang="ts">
  let { dirty, chars, lines }: { dirty: boolean; chars: number; lines: number } = $props();
</script>

<footer>
  <span class="dot" class:saved={!dirty}></span>
  <span>{chars} chars · {lines} lines</span>
</footer>

<style>
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
</style>
