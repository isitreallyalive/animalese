<script lang="ts">
	import { listen } from "@tauri-apps/api/event";
	import { SvelteSet } from "svelte/reactivity";

	import layout from "$lib/layout";

	import Key from "./Key.svelte";

	let activeKeys = new SvelteSet<string>();

	$effect(() => {
		// setup
		const press = listen<string>("press", e => activeKeys.add(e.payload));
		const release = listen<string>("release", e =>
			activeKeys.delete(e.payload),
		);

		// teardown
		return () => {
			press.then(unlisten => unlisten());
			release.then(unlisten => unlisten());
		};
	});
</script>

{#each layout as row, i (i)}
	<div class="mb-2 flex justify-center">
		{#each row as data (data.key)}
			{@const active = activeKeys.has(data.key)}
			<Key {active} {...data} />
		{/each}
	</div>
{/each}
