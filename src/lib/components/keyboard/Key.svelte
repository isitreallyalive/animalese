<script lang="ts">
  import { Button } from "../ui/button";
  import Icon, { type IconifyIcon } from "@iconify/svelte";

  export interface KeyInfo {
    active: boolean;
    label?: string;
    icon?: IconifyIcon;
    width: number;
  }

  const { active, label, icon, width }: KeyInfo = $props();
</script>

<Button
  class="m-1 flex items-center gap-2 *:align-middle {active ? 'bg-honey' : ''}"
  style={`width: ${width * 40}px`}
>
  {#if icon}
    <Icon {icon} class="inline-block h-6 w-6 *:stroke-3" />
  {/if}
  {#if label}
    <span class="text-base">{label}</span>
  {/if}
</Button>
