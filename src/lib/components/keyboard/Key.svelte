<script lang="ts">
  import { Button } from "../ui/button";
  import Icon, { type IconifyIcon } from "@iconify/svelte";

  export interface KeyInfo {
    active: boolean;
    label?: string;
    icon?: IconifyIcon;
  }

  const { active, label, icon }: KeyInfo = $props();
</script>

<Button
  class="m-1 flex items-center gap-2 *:align-middle {active
    ? 'bg-red-400'
    : ''}"
>
  {#if icon}
    <Icon {icon} class="inline-block h-5 w-5" />
  {/if}
  {#if label}
    <span class="text-base">{label}</span>
  {/if}
</Button>
