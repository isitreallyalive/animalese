<script lang="ts">
	import Icon, { type IconifyIcon } from "@iconify/svelte";
	import { invoke } from "@tauri-apps/api/core";

	import { Button } from "../ui/button";

	export interface Props {
		active: boolean;
		label?: string;
		icon?: IconifyIcon;
		width: number;
		key: string;
	}

	async function simulate(press: boolean) {
		return await invoke("simulate", { key, press });
	}

	const { active, label, icon, width, key }: Props = $props();
</script>

<Button
	class="m-1 flex items-center gap-2 *:align-middle {active
		? 'bg-honey'
		: 'hover:bg-honey/50 hover:cursor-pointer'}"
	style={`width: ${width * 40}px`}
	onmousedown={async () => await simulate(true)}
	onmouseup={async () => await simulate(false)}
>
	{#if icon}
		<Icon {icon} class="inline-block h-6 w-6 *:stroke-3" />
	{/if}
	{#if label}
		<span class="text-base">{label}</span>
	{/if}
</Button>
