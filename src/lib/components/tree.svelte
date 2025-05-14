<script lang="ts">
  import { Folder, FolderOpen, Document } from "@steeze-ui/heroicons";
  import { Icon } from "@steeze-ui/svelte-icon";
  import { Tree, type TreeItem } from "melt/builders";
  import { fly, scale } from "svelte/transition";

  type Item = TreeItem & {
    title: string;
    icon: string;
    children?: Item[];
  };

  const data: Item[] = [
    {
      id: "index.svelte",
      title: "index.svelte",
      icon: "svelte",
    },
    {
      id: "lib",
      title: "lib",
      icon: "folder",
      children: [
        {
          id: "lib/icons",
          title: "icons",
          icon: "folder",
          children: [
            {
              id: "lib/icons/JavaScript.svelte",
              title: "JavaScript.svelte",
              icon: "svelte",
            },
            {
              id: "lib/icons/Svelte.svelte",
              title: "Svelte.svelte",
              icon: "svelte",
            },
          ],
        },
        {
          id: "lib/tree",
          title: "tree",
          icon: "folder",
          children: [
            {
              id: "lib/tree/Tree.svelte",
              title: "Tree.svelte",
              icon: "svelte",
            },
            {
              id: "lib/tree/TreeItem.svelte",
              title: "TreeItem.svelte",
              icon: "svelte",
            },
          ],
        },
      ],
    },
    {
      id: "routes",
      title: "routes",
      icon: "folder",
      children: [
        {
          id: "routes/contents",
          title: "contents",
          icon: "folder",
          children: [
            {
              id: "routes/contents/+layout.svelte",
              title: "+layout.svelte",
              icon: "svelte",
            },
            {
              id: "routes/contents/+page.svelte",
              title: "+page.svelte",
              icon: "svelte",
            },
          ],
        },
      ],
    },
  ];

  const tree = new Tree({
    multiple: true,
    expandOnClick: true,
    items: data,
  });
</script>

{#snippet treeItemIcon(item: (typeof tree)["children"][0])}
  {@const icon = item.item.icon}

  {#if icon === "folder"}
    <Icon
      src={item.expanded ? FolderOpen : Folder}
      theme="solid"
      class="size-5 color-gray-900"
    />
  {:else}
    <Icon src={Document} theme="solid" class="size-5 color-gray-900" />
  {/if}
{/snippet}

{#snippet treeItems(items: (typeof tree)["children"], depth: number = 0)}
  {#each items as item (item.id)}
    <div
      {...item.attrs}
      class=" cursor-pointer rounded-sm !outline-none first:mt-0 [&:focus-visible>:first-child>div]:ring-4 text-[#c4c1bf]"
    >
      <div class="group py-1" style="padding-left: {depth * 1}rem">
        <div
          class="{item.selected
            ? '!bg-accent-500 dark:!bg-accent-200 dark:!text-accent-950 !text-white'
            : ''}
					ring-accent-500 dark:ring-accent-700 flex h-full w-full items-center gap-2 rounded-xl
					px-3 py-1 ring-offset-white transition group-hover:bg-gray-200
					dark:ring-offset-black dark:group-hover:bg-gray-800"
        >
          {@render treeItemIcon(item)}
          <span class="select-none">
            {item.item.title}
          </span>
        </div>
      </div>
      {#if item.children?.length && item.expanded}
        <div
          class="relative"
          in:fly={{ y: -20, duration: 200 }}
          out:fly={{ y: -20, duration: 200 }}
        >
          <div
            class="absolute bottom-2 top-2 w-px bg-gray-200 dark:bg-gray-700"
            style="left: {0.5 + depth * 1}rem"
          ></div>
          {@render treeItems(item.children, depth + 1)}
        </div>
      {/if}
    </div>
  {/each}
{/snippet}

{@render treeItems(tree.children, 0)}
