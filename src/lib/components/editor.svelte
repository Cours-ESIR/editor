<script lang="ts">
  import * as monaco from "monaco-editor";

  import { fs, invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";

  let {
    path,
    uri = $bindable(),
  }: {
    path: string;
    uri: string;
  } = $props();

  let texte = $state("= demo \n $1/2$");

  async function initEditor(div: HTMLDivElement, path: string | undefined) {
    if (path) {
      let buff = await fs.readBinaryFile(path);
      let buff2 = buff as unknown as number[];
      texte = String.fromCharCode.apply(null, buff2);
    }

    let editor = monaco.editor.create(div, {
      theme: window.matchMedia("(prefers-color-scheme: dark)").matches ? "vs-dark" : "vs",
      minimap: {
        enabled: false,
      },
      automaticLayout: true,
      value: texte,
    });

    editor.onDidChangeModelContent(async (event) => {
      texte = editor.getValue();
      invoke("update_project", { content: texte }).then();
      invoke("compile_project").then();
      let data = await invoke("render_project", { page: 0 });
      uri = `data:image/svg+xml;utf8,${encodeURIComponent(data)}`;
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
