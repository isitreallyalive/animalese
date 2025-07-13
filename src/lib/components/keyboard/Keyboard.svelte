<script lang="ts">
  import { getIconData } from "@iconify/utils";
  import Key, { type KeyInfo } from "./Key.svelte";
  import { icons as lucide, type IconifyJSON } from "@iconify-json/lucide";
  import { platform } from "@tauri-apps/plugin-os";

  function icon(iconSet: IconifyJSON, iconName: string, label?: string) {
    return { icon: getIconData(iconSet, iconName)!, label };
  }
  const label = (label: string) => ({ label });
  const split = (letters: string) => letters.split("").map((c) => label(c));

  const layout: KeyInfo[][] = [
    [...split("`1234567890-="), icon(lucide, "arrow-left", "Backspace")],
    [
      icon(lucide, "arrow-right-to-line", "Tab"),
      ...split("qwertyuiop"),
      label("["),
      label("]"),
      label("\\"),
    ],
    [
      label("Caps"),
      ...split("asdfghjkl"),
      label(";"),
      label("'"),
      icon(lucide, "corner-down-left", "Enter"),
    ],
    [
      icon(lucide, "arrow-big-up-dash", "Shift"),
      ...split("zxcvbnm"),
      label(","),
      label("."),
      label("/"),
      icon(lucide, "arrow-big-up-dash", "Shift"),
    ],
    [
      icon(lucide, "command", "Ctrl"),
      label(platform() === "windows" ? "Win" : "Cmd"),
      label("Alt"),
      label(" "),
      label("Alt"),
      label("Ctrl")
    ],
  ];
</script>

{#each layout as row}
  <div class="mb-2 flex justify-center">
    {#each row as key}
      <Key {...key} />
    {/each}
  </div>
{/each}
