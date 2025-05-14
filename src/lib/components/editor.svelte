<script lang="ts">
  let {
    path,
    text = $bindable(),
  }: {
    path: string;
    text: string;
  } = $props();
</script>

<div class="h-dvh p-4 editor" bind:innerText={text} contenteditable>
  <div>Initial content</div>
</div>
<style>
  .editor {
    counter-reset: section;
  }
  .editor > *::before {
    counter-increment: section;
    content: counter(section);
    position: absolute;
    left: 1rem;
    color: #666;
  }
  .editor > * {
    position: relative;
    padding-left: 8rem;
  }
</style>
