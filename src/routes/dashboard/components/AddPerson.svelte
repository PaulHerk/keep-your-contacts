<script lang="ts">
  import { Modal } from "@skeletonlabs/skeleton-svelte";
  import { enhance } from "$app/forms";
  import type { PageProps } from "../$types";
  import { api_request } from "$lib";
  import { onMount } from "svelte";
  import { error } from "@sveltejs/kit";
  import { CirclePlus, SquarePlus } from "@lucide/svelte";

  let { form } = $props();

  let knownFromSources: KnownFromSource[] = $state([]);
  let selected = $state(0);
  let newSource = $state("");

  $inspect(knownFromSources);

  onMount(async () => {
    api_request(fetch, "/known-from-sources").then(async (response) => {
      if (!response.ok) error(500, await response.text());
      knownFromSources = await response.json();
    });
  });

  function addNew() {
    api_request(fetch, "/known-from-sources", {
      method: "POST",
      body: JSON.stringify({ sourceName: newSource, description: "" }),
    }).then(async (response) => {
      if (!response.ok) error(500, await response.text());
      knownFromSources.push({
        id: selected,
        sourceName: newSource,
        description: "",
      });
    });
  }

  let openState = $state(false);

  function modalClose() {
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
      {required}
    />
  </label>
{/snippet}

<Modal
  open={openState}
  onOpenChange={(e) => (openState = e.open)}
  triggerBase="btn btn-lg preset-filled-tertiary-500"
  contentBase="card bg-surface-100-900 p-4 space-y-4 shadow-xl max-w-screen-sm"
  backdropClasses="backdrop-blur-sm"
>
  {#snippet trigger()}Add Person{/snippet}
  {#snippet content()}
    <header class="flex justify-between">
      <h2 class="h2">Add Person</h2>
    </header>
    <form
      class="flex flex-col gap-4 p-4"
      method="POST"
      action="?/addPerson"
      use:enhance
    >
      <article class="flex flex-col gap-5">
        {@render input("Full Name:", "name", true)}
        <label class="label">
          <span class="label-text">Where do you know this person from?</span>
          <div class="flex gap-4">
            <select bind:value={selected} class="select">
              {#each knownFromSources as source}
                <option value={source.id}>{source.sourceName}</option>
              {/each}
              <option value={knownFromSources.length + 1}>add new</option>
            </select>
            {#if selected === knownFromSources.length + 1}
              <input
                class="input"
                type="text"
                placeholder="add new"
                bind:value={newSource}
              />
              <button class="button" onclick={addNew}>
                <SquarePlus size={30} />
              </button>
            {/if}
          </div>
        </label>
        <!-- {@render input("known_from_source_id")} -->
        {#if form?.placeNotFound}
          <p>{form?.message}</p>
        {/if}
        {@render input("Place:", "coordinate")}
        {@render input("Job Title:", "jobTitle")}
        {@render input("Company:", "company")}
        {@render input("", "linkedin")}
        {@render input("Notes:", "notes")}
      </article>
      <footer class="flex justify-end gap-4">
        <button type="button" class="btn preset-tonal" onclick={modalClose}
          >Cancel</button
        >
        <input
          type="submit"
          value="Confirm"
          class="btn preset-filled"
          onclick={modalClose}
        />
        <!-- <button class="btn preset-filled">Confirm</button> -->
      </footer>
    </form>
  {/snippet}
</Modal>
