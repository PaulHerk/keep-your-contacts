<script lang="ts">
	import { enhance } from "$app/forms";
	import type { ActionData } from "../$types";
	import { api_request } from "$lib";
	import { error } from "@sveltejs/kit";
	import { SquarePlus } from "@lucide/svelte";
	import { applyAction } from "$app/forms";
	import { authToken, knownFromSources, persons } from "../store";
	import { invalidate } from "$app/navigation";
	import EditKfs from "./EditKFS.svelte";

	let {
		form,
		personToUpdate = undefined,
		personPlaceToAdd,
		openState = $bindable(false),
	}: {
		form: ActionData;
		personToUpdate?: {
			person: NewPerson;
			personId: number;
			record?: PlaceRecord;
		};
		personPlaceToAdd?: CoordinateSearch;
		openState: boolean;
	} = $props();

	let selectedKfs = $state(
		personToUpdate ? personToUpdate.person.knownFromSourceId : null, // TODO: this does'nt update
	);
	let kfs = $derived(
		selectedKfs !== null ? $knownFromSources[selectedKfs] : null,
	);
	let placeStr = $derived.by(() => {
		if (personPlaceToAdd) return personPlaceToAdd.search; // maybe lazy load place string?

		if (personToUpdate?.record) return personToUpdate.record.search;

		if (kfs) return kfs.locationSearch;
	});
	$inspect(personToUpdate?.record?.search);

	let newSource = $state("");

	function addNewKnownFromSource() {
		api_request(
			fetch,
			"/known-from-sources",
			{
				method: "POST",
				body: JSON.stringify({
					sourceName: newSource,
				}),
			},
			$authToken,
		).then(async (response) => {
			if (!response.ok) error(500, await response.text());
			let newSourceId = await response.json();
			knownFromSources.update((oldSources) => [
				...oldSources,
				{
					sourceId: newSourceId,
					sourceName: newSource,
					description: "",
				} as KnownFromSource,
			]);
			selectedKfs = $knownFromSources.length - 1;
		});
	}

	function modalCloseAndAddPerson(newPerson: Person) {
		persons.update((oldP) => [...oldP, newPerson]);
		invalidate((url) => url.pathname === "/persons/count"); // Update personCount (this will also update it in maps)
		openState = false;
	}

	function modalCloseAndUpdatePersons(newPerson: Person) {
		persons.update((oldPersons) => {
			if (
				!oldPersons.some(
					(p) =>
						p.id ===
						personToUpdate?.personId,
				)
			)
				error(500, "idfk at this point");

			let idx = oldPersons.findIndex(
				(p) => p.id === personToUpdate?.personId,
			);
			oldPersons.splice(idx, 1, newPerson);
			return oldPersons; // TODO: on place change does not update
		});
		openState = false;
	}
</script>

{#snippet input(label: string, key: keyof NewPerson, required = false)}
	<label class="label">
		<span class="label-text">{label}</span>
		<input
			class="input"
			type="text"
			placeholder={label}
			name={key}
			value={personToUpdate ? personToUpdate.person[key] : ""}
			{required}
		/>
	</label>
{/snippet}

<form
	class="flex flex-col gap-4 p-4"
	method="POST"
	action={personToUpdate
		? "/dashboard?/updatePerson"
		: "/dashboard?/addPerson"}
	use:enhance={({ formData, formElement, cancel }) => {
		if (formElement.id === "cancel") cancel();
		if (selectedKfs === $knownFromSources.length)
			formData.delete("knownFromSourceId"); // ensures that user cannot set the KFSID to the "add new" value
		if (personToUpdate)
			formData.append(
				"personId",
				personToUpdate.personId.toString(),
			);
		if (personPlaceToAdd)
			formData.set("place", JSON.stringify(personPlaceToAdd));

		if (selectedKfs === $knownFromSources.length)
			selectedKfs = null;
		if (selectedKfs !== null) {
			formData.set(
				"knownFromSources",
				$knownFromSources
					.findIndex(
						(_kfs, idx) =>
							(idx =
								selectedKfs as number), // Typescript, are u ok?
					)
					.toString(),
			);
		}

		return async ({ result }) => {
			if (result.type === "success") {
				const newP = (
					result.data as {
						success: boolean;
						newPerson: Person;
					}
				).newPerson;

				personToUpdate
					? modalCloseAndUpdatePersons(newP)
					: modalCloseAndAddPerson(newP);
			}
			await applyAction(result);
		};
	}}
>
	<header>
		<h2 class="h2">
			{personToUpdate ? "Update Person" : "Add Person"}
		</h2>
	</header>
	<article class="flex flex-col gap-5">
		{@render input("Full Name:", "name", true)}
		<label class="label">
			<span class="label-text"
				>Where do you know this person from?</span
			>
			<div class="flex gap-4">
				<select
					bind:value={selectedKfs}
					class="select"
					name="knownFromSourceId"
				>
					<option value={null}></option>
					{#each $knownFromSources as source, idx}
						<option value={idx}
							>{source.sourceName}</option
						>
					{/each}
					<option value={$knownFromSources.length}
						>add new</option
					>
				</select>
				{#if selectedKfs === $knownFromSources.length}
					<input
						class="input"
						type="text"
						placeholder="add new"
						bind:value={newSource}
					/>
					<button
						class="button"
						type="button"
						onclick={addNewKnownFromSource}
					>
						<SquarePlus size={30} />
					</button>
				{:else if selectedKfs !== null}
					<EditKfs kfsIds={[selectedKfs]} />
				{/if}
			</div>
		</label>

		<label class="label">
			{#if form?.placeNotFound}
				<p class="text-red-600">{form?.message}</p>
			{/if}
			<span class="label-text">Place:</span>
			<input
				class="input"
				type="text"
				placeholder="Place:"
				name="place"
				value={placeStr}
			/>
		</label>

		{@render input("Job Title:", "jobTitle")}
		{@render input("Company:", "company")}
		{@render input("", "linkedin")}
		{@render input("Notes:", "notes")}
	</article>
	<footer class="flex justify-end gap-4">
		<button
			type="reset"
			class="btn preset-tonal"
			id="cancel"
			onclick={() => (openState = false)}>Cancel</button
		>
		<input
			type="submit"
			value="Confirm"
			class="btn preset-filled"
		/>
	</footer>
</form>
