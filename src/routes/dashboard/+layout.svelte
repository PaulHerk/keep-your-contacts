<script lang="ts">
  let { children } = $props();

  import { Navigation } from "@skeletonlabs/skeleton-svelte";
  // Icons
  import IconMenu from "@lucide/svelte/icons/menu";
  import IconSettings from "@lucide/svelte/icons/settings";
  import { ContactRound, Map } from "@lucide/svelte";
  import { page } from "$app/state";
  import { onMount, setContext } from "svelte";
  import { api_request } from "$lib";

  let innerWidth = $state(0);
</script>

<svelte:window bind:innerWidth />

<div
  class="card border-surface-100-900 grid h-[640px] w-full grid-cols-[auto_1fr]"
>
  <div>
    <!-- Component -->
    <Navigation.Rail
      expanded={innerWidth > 640}
      classes="fixed top-0 bottom-0 left-0"
      width="w-15"
      widthExpanded="w-35"
    >
      {#snippet header()}
        <Navigation.Tile href="#" title="Menu"><IconMenu /></Navigation.Tile>
      {/snippet}
      {#snippet tiles()}
        <Navigation.Tile
          labelExpanded="Persons"
          href="/dashboard"
          selected={page.url.pathname === "/dashboard"}
        >
          <ContactRound />
        </Navigation.Tile>
        <Navigation.Tile
          labelExpanded="Map"
          href="/dashboard/map"
          selected={page.url.pathname === "/dashboard/map"}
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
  </div>
  <!-- Content -->
  <div class="flex items-center justify-center ml-15 sm:ml-35">
    {@render children()}
  </div>
</div>
