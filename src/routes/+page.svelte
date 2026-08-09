<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let version = $state("…");

  invoke<string>("app_version").then((v) => {
    version = v;
  });
</script>

<main class="container">
  <h1>vStickier</h1>
  <p>{version}</p>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    color: #0f0f0f;
    background-color: #f6f6f6;
  }

  .container {
    margin: 0;
    padding-top: 20vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  h1 {
    margin: 0;
    text-align: center;
  }

  p {
    color: #888;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
