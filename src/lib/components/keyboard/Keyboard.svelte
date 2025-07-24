<script lang="ts">
  import Key from "./Key.svelte";
  import { SvelteSet } from "svelte/reactivity";
  import { listen } from "@tauri-apps/api/event";
  import layout from "$lib/layout";

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
