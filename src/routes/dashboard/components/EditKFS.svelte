<script lang="ts">
	import { Modal } from "@skeletonlabs/skeleton-svelte";
	import { knownFromSources } from "../store";
	import { api_request } from "$lib";
	import { ArrowLeft, ArrowRight } from "@lucide/svelte";

	let { kfsIds }: { kfsIds: number[] } = $props();

	let kfss = $knownFromSources.filter((kfs) =>
		kfsIds.includes(kfs.sourceId),
	);
	let openState = $state(false);
	function modalClose() {
		openState = false;
	}

	function updateKfs() {
		api_request(fetch, "", { method: "POST" }).then(async (res) => {
			console.log(await res.json());
		});
		modalClose();
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
			value={kfss[idx][key]}
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
				Edit {kfsIds.length === 1
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
				{#each kfss as _kfs, idx}
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
					</div>
				{/each}
			</div>
		</article>
		<footer class="flex justify-end gap-4">
			<button
				type="button"
				class="btn preset-tonal"
				onclick={modalClose}>Cancel</button
			>
			<button
				type="button"
				class="btn preset-filled"
				onclick={updateKfs}>Confirm</button
			>
		</footer>
	{/snippet}
</Modal>
