<script>
    import init, { main_js } from "$lib/pkg/backend";
    import { onMount } from "svelte";

    onMount(async () => {
        await init().then(() => main_js());
    });
</script>

<h1>Welcome to SvelteKit</h1>
<p>
    Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the
    documentation
</p>
