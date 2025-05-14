<script lang="ts">
    import { invoke } from "@tauri-apps/api";

    let { text }: { text: string } = $props();
    let uri = $state("");

    async function render() {
        await invoke("update_project", { content: text });
        await invoke("compile_project");
        let value: string = await invoke("render_project", { page: 0 });
        uri = `data:image/svg+xml;utf8,${encodeURIComponent(value)}`;
    }

    $effect(() => {
        render();
    });
</script>

<img src={uri} alt="typst document" />
