<script module lang="ts">
  export interface EmittedKey {
    key: string;
    simulated: boolean;
  }
</script>

<script lang="ts">
  import Key from "./Key.svelte";
  import { listen } from "@tauri-apps/api/event";
  import layout from "$lib/layout";
  import { SvelteMap } from "svelte/reactivity";

  let emittedKeys = new SvelteMap<string, boolean>();

  $effect(() => {
    // setup
    const press = listen<EmittedKey>(
      "press",
      ({ payload: { key, simulated } }) => {
        emittedKeys.set(key, simulated);
        console.log(emittedKeys);
      },
    );
    const release = listen<EmittedKey>("release", (e) => {
      emittedKeys.delete(e.payload.key);
      console.log(emittedKeys);
    });

    // teardown
    return () => {
      press.then((unlisten) => unlisten());
      release.then((unlisten) => unlisten());
    };
  });
</script>

{#each layout as row}
  <div class="mb-2 flex justify-center">
    {#each row as data}
      {@const active = emittedKeys.has(data.key)}
      {@const simulated = emittedKeys.get(data.key) || false}
      <Key {active} {simulated} {...data} />
    {/each}
  </div>
{/each}
