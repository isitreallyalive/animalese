<script lang="ts">
    import { Button } from "../ui/button";

    interface Props {
        key: string;
    }

    const { key }: Props = $props();
</script>

<Button class="m-1">
    {key}
</Button>