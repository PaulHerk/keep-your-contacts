<script lang="ts">
    let { children } = $props();

    import { Navigation } from "@skeletonlabs/skeleton-svelte";
    // Icons
    import IconMenu from "@lucide/svelte/icons/menu";
    import IconSettings from "@lucide/svelte/icons/settings";
    import { ContactRound, Map } from "@lucide/svelte";
    import { page } from "$app/state";
    import { onMount, setContext } from "svelte";
    import { api_get } from "$lib";
</script>

<div
    class="card border-surface-100-900 grid h-[640px] w-full grid-cols-[auto_1fr] border-[1px]"
>
    <!-- Component -->
    <Navigation.Rail>
        {#snippet header()}
            <Navigation.Tile href="#" title="Menu"><IconMenu /></Navigation.Tile
            >
        {/snippet}
        {#snippet tiles()}
            <Navigation.Tile
                label="Persons"
                href="/"
                selected={page.url.pathname === "/"}
            >
                <ContactRound />
            </Navigation.Tile>
            <Navigation.Tile
                label="Map"
                href="/map"
                selected={page.url.pathname === "/map"}
            >
                <Map />
            </Navigation.Tile>
        {/snippet}
        {#snippet footer()}
            <Navigation.Tile
                labelExpanded="Settings"
                href="#settings"
                title="settings"><IconSettings /></Navigation.Tile
            >
        {/snippet}
    </Navigation.Rail>
    <!-- Content -->
    <div class="flex items-center justify-center">
        {@render children()}
    </div>
</div>
