<script lang="ts">
	import "../app.css";

	import Header from "$lib/components/header";

	let { children } = $props();
</script>

<Header />

{@render children()}
