<script lang="ts">
  import Editor from "$lib/components/editor.svelte";
  import Tree from "$lib/components/tree.svelte";
  import { EllipsisVertical } from "@steeze-ui/heroicons";
  import { Icon } from "@steeze-ui/svelte-icon";
  import { Pane, PaneGroup, PaneResizer } from "paneforge";

  let data = $state("");
</script>

<div class="h-dvh w-dvw p-2">
  <PaneGroup direction="horizontal">
    <Pane defaultSize={20} class="rounded-2xl bg-gray-900 p-4">
      <Tree class="w-full"></Tree>
    </Pane>
    <PaneResizer
      class="relative flex w-2 items-center justify-center bg-gray-950 text-[#c4c1bf]"
    >
      <div
        class="z-10 flex h-6 w-4 items-center justify-center rounded-sm bg-accent-200"
      >
        <Icon src={EllipsisVertical} class="size-16 text-accent-950"></Icon>
      </div>
    </PaneResizer>
    <Pane defaultSize={40} class="rounded-2xl bg-gray-900">
      <Editor path="" bind:uri={data}></Editor>
    </Pane>
    <PaneResizer
      class="relative flex w-2 items-center justify-center bg-gray-950 text-[#c4c1bf]"
    >
      <div
        class="z-10 flex h-6 w-4 items-center justify-center rounded-sm bg-accent-200"
      >
        <Icon src={EllipsisVertical} class="size-16 text-accent-950"></Icon>
      </div>
    </PaneResizer>
    <Pane defaultSize={40} class="rounded-2xl bg-gray-900">
      <img src={data} alt="typst document" />
    </Pane>
  </PaneGroup>
</div>
