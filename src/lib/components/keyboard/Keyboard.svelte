<script lang="ts">
    import Key from "./Key.svelte";

    const layout = [
        ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
        ["a", "s", "d", "f", "g", "h", "j", "k", "l"],
        ["z", "x", "c", "v", "b", "n", "m"]
    ]
</script>

{#each layout as row}
    <div class="flex justify-center mb-2">
        {#each row as key}
            <Key {key} />
        {/each}
    </div>
{/each}