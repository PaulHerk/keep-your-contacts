<script lang="ts">
	import { enhance } from "$app/forms";
	import type { ActionData } from "../$types";
	import { getStringFromRecord, api_request } from "$lib";
	import { onMount } from "svelte";
	import { error } from "@sveltejs/kit";
	import { SquarePlus } from "@lucide/svelte";
	import { applyAction } from "$app/forms";
	import { authToken, knownFromSources, persons } from "../store";
	import { invalidate } from "$app/navigation";

	let {
		form,
		personToUpdate = undefined,
		personCoordinateToAdd,
		openState = $bindable(false),
	}: {
		form: ActionData;
		personToUpdate?: {
			person: NewPerson;
			personId: number;
			record?: PlaceRecord;
		};
		personCoordinateToAdd?: Coordinate;
		openState: boolean;
	} = $props();

	let selected_kfs = $state(
		personToUpdate ? personToUpdate.person.knownFromSourceId : 0, // TODO: this does'nt update
	);
	let kfs = $derived(
		$knownFromSources.find((kfs) => kfs.sourceId === selected_kfs),
	);

	let newSource = $state("");

	function addNewKnownFromSource() {
		api_request(
			fetch,
			"/known-from-sources",
			{
				method: "POST",
				body: JSON.stringify({
					sourceName: newSource,
					description: "",
				}),
			},
			$authToken,
		).then(async (response) => {
			if (!response.ok) error(500, await response.text());
			let newId = await response.json();
			knownFromSources.update((oldSources) => [
				...oldSources,
				{
					sourceId: newId,
					sourceName: newSource,
					description: "",
				} as KnownFromSource,
			]);
			selected_kfs = newId;
		});
	}

	function modalCloseAndAddPerson(newPerson: Person) {
		openState = false;
		persons.update((oldP) => [...oldP, newPerson]);
		invalidate((url) => url.pathname === "/persons/count"); // Update personCount (this will also update it in maps)
	}

	function modalCloseAndUpdatePersons(newPerson: Person) {
		openState = false;
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
			return oldPersons;
		});
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
		if (personToUpdate)
			formData.append(
				"personId",
				personToUpdate.personId.toString(),
			);

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
					bind:value={selected_kfs}
					class="select"
					name="knownFromSourceId"
				>
					<option value=""></option>
					{#each $knownFromSources as source}
						<option value={source.sourceId}
							>{source.sourceName}</option
						>
					{/each}
					<option
						value={$knownFromSources.length +
							1}>add new</option
					>
				</select>
				{#if selected_kfs === $knownFromSources.length + 1}
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
				name="coordinateOrSearch"
				value={personCoordinateToAdd
					? JSON.stringify(personCoordinateToAdd)
					: (kfs?.locationSearch ??
						getStringFromRecord(
							personToUpdate?.record,
						))}
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
