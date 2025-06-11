<script lang="ts">
	let { children } = $props();

	import { Modal, Navigation } from "@skeletonlabs/skeleton-svelte";
	// Icons
	import IconMenu from "@lucide/svelte/icons/menu";
	import IconSettings from "@lucide/svelte/icons/settings";
	import { ContactRound, Map } from "@lucide/svelte";
	import "../../style.css";
	import Settings from "./components/layout/Settings.svelte";
	import { page } from "$app/state";

	let innerWidth = $state(0);

	let settingsDrawerState = $state(false);
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
				<Navigation.Tile href="#" title="Menu"
					><IconMenu /></Navigation.Tile
				>
			{/snippet}
			{#snippet tiles()}
				<Navigation.Tile
					labelExpanded="Persons"
					href="/dashboard"
					selected={page.url.pathname ===
						"/dashboard"}
				>
					<ContactRound />
				</Navigation.Tile>
				<Navigation.Tile
					labelExpanded="Map"
					href="/dashboard/map"
					selected={page.url.pathname ===
						"/dashboard/map"}
				>
					<Map />
				</Navigation.Tile>
			{/snippet}
			{#snippet footer()}
				<Navigation.Tile
					labelExpanded="Settings"
					type="button"
					title="settings"
					onclick={() =>
						(settingsDrawerState =
							!settingsDrawerState)}
				>
					<IconSettings />
					<Modal
						open={settingsDrawerState}
						onOpenChange={(e) =>
							(settingsDrawerState =
								e.open)}
						contentBase="bg-surface-100-900 p-4 space-y-4 shadow-xl w-[480px] h-screen"
						positionerJustify="justify-start"
						positionerAlign=""
						positionerPadding=""
						transitionsPositionerIn={{
							x: -480,
							duration: 200,
						}}
						transitionsPositionerOut={{
							x: -480,
							duration: 200,
						}}
					>
						{#snippet content()}
							<header
								class="flex justify-between"
							>
								<h2 class="h2">
									Settings
								</h2>
							</header>
							<article>
								<Settings
									bind:settingsDrawerState
									personCount={page
										.data
										.personCount}
								/>
							</article>
							<footer>
								<button
									type="button"
									class="btn preset-filled"
									onclick={() =>
										(settingsDrawerState = false)}
								>
									Close
								</button>
							</footer>
						{/snippet}
					</Modal>
				</Navigation.Tile>
			{/snippet}
		</Navigation.Rail>
	</div>
	<!-- Content -->
	<div class="flex items-center justify-center ml-15 sm:ml-35">
		{@render children()}
	</div>
</div>
