<script lang="ts">
	import { Modal } from "@skeletonlabs/skeleton-svelte";
	import { authToken, knownFromSources } from "../store";
	import { api_request } from "$lib";
	import { error } from "@sveltejs/kit";

	let { kfsIds }: { kfsIds: number[] } = $props();

	let kfss = $derived(
		$knownFromSources.filter((kfs) =>
			kfsIds.includes(kfs.sourceId),
		),
	);
	let openState = $state(false);
	function modalClose() {
		openState = false;
	}

	function updateKfs(sourceId: number, idx: number) {
		api_request(
			fetch,
			`/known-from-sources/${sourceId}`,
			{
				method: "PUT",
				body: JSON.stringify({
					sourceName: kfss[idx].sourceName,
					description: kfss[idx].description,
					locationSearch:
						kfss[idx].locationSearch,
				}),
			},
			$authToken,
		).then(async (res) => {
			if (!res.ok) error(500, await res.text());
			knownFromSources.update((oldKfss) => {
				let kfsIdx = oldKfss.findIndex(
					(kfs) =>
						(kfs.sourceId =
							kfss[idx].sourceId),
				); // I am not sure whether this is needed. I can imagine there's a situation where the $knownFromSources idx doesn't match up with the kfss idx.
				oldKfss.splice(kfsIdx, 1, kfss[idx]);
				return oldKfss;
			});
		});
	}
</script>

{#snippet input(label: string, key: keyof KnownFromSource, idx: number)}
	<label class="label">
		<span class="label-text">{label}</span>
		<input
			class="input"
			type="text"
			placeholder={label}
			name={key}
			bind:value={kfss[idx][key]}
		/>
	</label>
{/snippet}

<Modal
	open={openState}
	onOpenChange={(e) => (openState = e.open)}
	triggerBase="btn preset-tonal"
	contentBase="bg-surface-100-900 p-4 space-y-4 w-full xs:w-[28rem] h-full flex flex-col justify-center align-center sm:border-r-4 rounded-lg border-zinc-100"
	positionerJustify="justify-start"
	transitionsPositionerIn={{ x: -480, duration: 200 }}
	transitionsPositionerOut={{ x: -480, duration: 200 }}
>
	{#snippet trigger()}Edit {kfsIds.length === 1
			? ""
			: "Known-From-Sources"}{/snippet}
	{#snippet content()}
		<header>
			<h2 class="h2">
				Edit <br />
				{kfsIds.length === 1
					? '"' + kfss[0].sourceName + '"'
					: "Known-From-Sources"}
			</h2>
		</header>
		<article>
			<!-- Carousel -->
			<div
				data-multi-column
				class="snap-x snap-mandatory scroll-smooth flex gap-2 pb-2 overflow-x-auto"
			>
				<!-- Loop through our array of knownFromSources. -->
				{#each kfss as { sourceId }, idx}
					<div
						class="snap-center shrink-0 card py-20 px-4 w-60 xs:w-80 text-center flex flex-col gap-4"
					>
						{@render input(
							"Name",
							"sourceName",
							idx,
						)}
						{@render input(
							"Description",
							"description",
							idx,
						)}
						{@render input(
							"Location",
							"locationSearch",
							idx,
						)}
						<button
							type="button"
							class="btn preset-filled"
							onclick={() =>
								updateKfs(
									sourceId,
									idx,
								)}
							>Update</button
						>
					</div>
				{/each}
			</div>
		</article>
		<footer class="flex justify-end-safe">
			<button
				type="button"
				class="btn preset-filled-surface-500 w-xs"
				onclick={modalClose}>Done</button
			>
		</footer>
	{/snippet}
</Modal>
