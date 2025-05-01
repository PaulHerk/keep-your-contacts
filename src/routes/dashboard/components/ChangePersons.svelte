<script lang="ts">
  import { Modal } from "@skeletonlabs/skeleton-svelte";
  import { enhance } from "$app/forms";
  import type { ActionData } from "../$types";
  import { api_request, api_url } from "$lib";
  import { onMount } from "svelte";
  import { error } from "@sveltejs/kit";
  import { SquarePlus } from "@lucide/svelte";
  import { applyAction } from "$app/forms";
  import { knownFromSources, persons } from "../store";
  import { invalidate } from "$app/navigation";

  let {
    form,
    personToUpdate = undefined,
    personCount,
    perPage = $bindable(),
    openState = $bindable(false),
  }: {
    form: ActionData;
    personToUpdate?: { person: NewPerson; personId: number };
    personCount: number;
    perPage: number;
    openState: boolean;
  } = $props();

  let selected = $state(
    personToUpdate ? personToUpdate.person.knownFromSourceId : 0 // TODO: this does'nt update
  );
  let newSource = $state("");

  onMount(async () => {
    api_request(fetch, "/known-from-sources").then(async (response) => {
      if (!response.ok) error(500, await response.text());
      $knownFromSources = await response.json();
    });
  }); // TODO: put in load

  function addNewKnownFromSource() {
    api_request(fetch, "/known-from-sources", {
      method: "POST",
      body: JSON.stringify({ sourceName: newSource, description: "" }),
    }).then(async (response) => {
      if (!response.ok) error(500, await response.text());
      let newId = await response.json();
      knownFromSources.update((oldSources) => [
        {
          sourceId: newId,
          sourceName: newSource,
          description: "",
        } as KnownFromSource,
        ...oldSources,
      ]);
      selected = newId;
    });
  }

  function modalCloseAndAddPerson(newPerson: Person) {
    openState = false;
    if (perPage === personCount) perPage += 1;
    $persons.push(newPerson);
    invalidate(api_url + "/persons/count"); // Update personCount (this will also update it in maps)
    // ? is it smarter to instead refetch it in /maps?
  }

  function modalCloseAndUpdatePersons(newPerson: Person) {
    openState = false;
    persons.update((oldPersons) => {
      if (!oldPersons.some((p) => p.id === personToUpdate?.personId))
        error(500, "idfk at this point");

      let idx = oldPersons.findIndex((p) => p.id === personToUpdate?.personId);
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
  action={personToUpdate ? "?/updatePerson" : "?/addPerson"}
  use:enhance={({ formData, formElement, cancel }) => {
    if (formElement.id === "cancel") cancel();
    if (personToUpdate)
      formData.append("personId", personToUpdate.personId.toString());

    return async ({ result }) => {
      if (result.type === "success") {
        const newP = (result.data as { success: boolean; newPerson: Person })
          .newPerson;

        personToUpdate
          ? modalCloseAndUpdatePersons(newP)
          : modalCloseAndAddPerson(newP);
      }
      await applyAction(result);
    };
  }}
>
  <article class="flex flex-col gap-5">
    {@render input("Full Name:", "name", true)}
    <label class="label">
      <span class="label-text">Where do you know this person from?</span>
      <div class="flex gap-4">
        <select bind:value={selected} class="select" name="knownFromSourceId">
          <option value=""></option>
          {#each $knownFromSources as source}
            <option value={source.sourceId}>{source.sourceName}</option>
          {/each}
          <option value={$knownFromSources.length + 1}>add new</option>
        </select>
        {#if selected === $knownFromSources.length + 1}
          <input
            class="input"
            type="text"
            placeholder="add new"
            bind:value={newSource}
          />
          <!-- TODO: Place value is not shown by default right now -->
          <button class="button" type="button" onclick={addNewKnownFromSource}>
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
      <input class="input" type="text" placeholder="Place:" name="coordinate" />
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
    <input type="submit" value="Confirm" class="btn preset-filled" />
  </footer>
</form>
