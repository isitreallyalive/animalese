<script lang="ts">
  import { Button } from "../ui/button";
  import Icon, { type IconifyIcon } from "@iconify/svelte";

  export interface KeyInfo {
    label?: string;
    icon?: IconifyIcon;
  }

  const { label, icon }: KeyInfo = $props();
</script>

<Button class="m-1 flex items-center gap-2 *:align-middle">
  {#if icon}
    <Icon {icon} class="inline-block h-5 w-5" />
  {/if}
  {#if label}
    <span class="text-base">{label}</span>
  {/if}
</Button>
