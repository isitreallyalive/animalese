<script lang="ts">
  import { getIconData } from "@iconify/utils";
  import Key, { type KeyInfo } from "./Key.svelte";
  import { icons as lucide, type IconifyJSON } from "@iconify-json/lucide";
  import { platform } from "@tauri-apps/plugin-os";
  import { SvelteSet } from "svelte/reactivity";
  import { listen } from "@tauri-apps/api/event";

  type KeyInfoWithEvent = Omit<KeyInfo, "active"> & {
    event: string;
  };

  function icon(
    iconSet: IconifyJSON,
    iconName: string,
    event: string,
    label?: string,
  ): KeyInfoWithEvent {
    return { icon: getIconData(iconSet, iconName)!, event, label };
  }
  const label = (label: string, event: string): KeyInfoWithEvent => ({
    label,
    event,
  });
  const split = (letters: string, prefix: string = "Key"): KeyInfoWithEvent[] =>
    letters
      .split("")
      .map((c) => label(c.toUpperCase(), `${prefix}${c.toUpperCase()}`));

  const layout: KeyInfoWithEvent[][] = [
    [
      label("`", "Unknown(223)"),
      ...split("1234567890", "Num"),
      label("-", "Minus"),
      label("=", "Equal"),
      icon(lucide, "arrow-left", "Backspace", "Backspace"),
    ],
    [
      icon(lucide, "arrow-right-to-line", "Tab", "Tab"),
      ...split("qwertyuiop"),
      label("[", "LeftBracket"),
      label("]", "RightBracket"),
      label("\\", "BackSlash"),
    ],
    [
      label("Caps", "CapsLock"),
      ...split("asdfghjkl"),
      label(";", "SemiColon"),
      label("'", "BackQuote"),
      icon(lucide, "corner-down-left", "Return", "Enter"),
    ],
    [
      icon(lucide, "arrow-big-up-dash", "ShiftLeft", "Shift"),
      ...split("zxcvbnm"),
      label(",", "Comma"),
      label(".", "Dot"),
      label("/", "Slash"),
      icon(lucide, "arrow-big-up-dash", "ShiftRight", "Shift"),
    ],
    [
      icon(lucide, "command", "ControlLeft", "Ctrl"),
      label(platform() === "windows" ? "Win" : "Cmd", "MetaLeft"),
      label("Alt", "Alt"),
      label(" ", "Space"),
      label("Alt", "AltGr"),
      label("Ctrl", "ControlRight"),
    ],
  ];

  console.log(layout);
  let activeKeys = new SvelteSet<string>();
  $effect(() => {
    // setup
    const press = listen<string>("press", (e) => activeKeys.add(e.payload));
    const release = listen<string>("release", (e) =>
      activeKeys.delete(e.payload),
    );

    // teardown
    return () => {
      press.then((unlisten) => unlisten());
      release.then((unlisten) => unlisten());
    };
  });
</script>

{#each layout as row}
  <div class="mb-2 flex justify-center">
    {#each row as { event: key, ...data }}
      {@const active = activeKeys.has(key)}
      <Key {active} {...data} />
    {/each}
  </div>
{/each}
