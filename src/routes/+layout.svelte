<script lang="ts">
  import { ModeWatcher, setMode } from 'mode-watcher';
	import '../app.css';
	let { children } = $props();
	setMode("system")
</script>

<ModeWatcher />
{@render children()}