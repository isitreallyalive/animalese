<script lang="ts">
	import { getIconData } from "@iconify/utils";
	import { icons as lucide } from "@iconify-json/lucide";
	import { getCurrentWindow } from "@tauri-apps/api/window";

	import Button from "./Button.svelte";

	const window = getCurrentWindow();
</script>

<div class="bg-honey flex h-10">
	<div class="ml-4 flex items-center gap-2">
		<img data-tauri-drag-region src="/leaf.png" alt="Leaf" />
		<h1 data-tauri-drag-region class="text-lg font-bold tracking-wide">
			animalese
		</h1>
	</div>
	<div data-tauri-drag-region class="flex-grow"></div>
	<div class="flex h-full items-center">
		<Button
			icon={getIconData(lucide, "minus")!}
			onclick={() => window.minimize()}
		/>
		<Button icon={getIconData(lucide, "x")!} onclick={() => window.close()} />
	</div>
</div>
