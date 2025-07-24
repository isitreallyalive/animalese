<script lang="ts">
  import { Button } from "../ui/button";
  import Icon, { type IconifyIcon } from "@iconify/svelte";

  interface Props {
    class?: string;
    onclick?: () => void;
    icon: IconifyIcon;
  }

  let { onclick, icon }: Props = $props();
</script>

<Button
  class="group bg-honey flex h-full items-center px-2 hover:cursor-pointer"
  {onclick}
>
  <Icon
    {icon}
    class="transition-transform duration-150 group-hover:scale-115"
  />
</Button>
