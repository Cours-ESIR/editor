<script lang="ts">
  import Editor from "$lib/components/editor.svelte";
  import * as Resizable from "$lib/components/ui/resizable/index.js";
  import { invoke } from "@tauri-apps/api";
  import { ModeWatcher } from "mode-watcher";

  let data = "";
</script>

<Resizable.PaneGroup direction="horizontal" class="w-full h-dvh">
  <Resizable.Pane defaultSize={50}>
    <Editor path="" bind:uri={data}></Editor>
  </Resizable.Pane>
  <Resizable.Handle withHandle />
  <Resizable.Pane defaultSize={50} class="overflow-auto h-dvh">
      <img src={data} />
  </Resizable.Pane>
</Resizable.PaneGroup>
