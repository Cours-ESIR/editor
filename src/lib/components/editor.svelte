<script lang="ts">
  import * as monaco from "monaco-editor";

  import { fs } from "@tauri-apps/api";
  import { onMount } from "svelte";

  let {
    path,
    text = $bindable(),
  }: {
    path: string;
    text: string;
  } = $props();

  async function initEditor(div: HTMLDivElement, path: string | undefined) {
    if (path) {
      let buff = await fs.readBinaryFile(path);
      let buff2 = buff as unknown as number[];
      text = String.fromCharCode.apply(null, buff2);
    }

    let editor = monaco.editor.create(div, {
      theme: window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "vs-dark"
        : "vs",
      minimap: {
        enabled: false,
      },
      automaticLayout: true,
      value: text,
    });
    editor.onDidChangeModelContent(async (event) => {
      text = editor.getValue();
    });
  }

  let div: HTMLDivElement;
  onMount(() => {
    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", (e) => {
        monaco.editor.setTheme(e.matches ? "vs-dark" : "vs");
      });
    initEditor(div, path);
  });
</script>

<div class="h-dvh" bind:this={div}></div>
