<script lang="ts">
  import { Modal } from "@skeletonlabs/skeleton-svelte";
  import AddOrChangePersons from "./AddOrChangePersons.svelte";
  import type { ActionData } from "../$types";
  import { api_request, api_url } from "$lib";
  import { error } from "@sveltejs/kit";
  import { authToken, persons } from "../store";
  import { invalidate } from "$app/navigation";
  let openState = $state(false);
  let {
    form,
    personToUpdate,
    children,
  }: {
    form: ActionData;
    personToUpdate: Person;
    children: any;
  } = $props();

  const newP: NewPerson = {
    name: `${personToUpdate.firstName} ${personToUpdate.lastName}`,
    knownFromSourceId: personToUpdate.knownFromSourceId,
    coordinate: personToUpdate.record
      ? ({
          lat: personToUpdate.record.lat,
          lon: personToUpdate.record.lon,
        } as Coordinate)
      : null,
    jobTitle: personToUpdate.jobTitle,
    company: personToUpdate.company,
    linkedin: personToUpdate.linkedin,
    notes: personToUpdate.notes,
  };
  const personId = personToUpdate.id;

  async function deletePerson(personId: number) {
    const response = await api_request(
      fetch,
      `/persons/${personId}`,
      {
        method: "DELETE",
      },
      $authToken
    );
    if (!response.ok)
      error(500, "Something went wrong: " + (await response.text()));

    persons.update((oldPersons) => {
      let idx = oldPersons.findIndex((p) => p.id === personId);
      oldPersons.splice(idx, 1);
      return oldPersons;
    });

    invalidate((url) => url.pathname === "/persons/count"); // Update personCount (this will also update it in maps)
    // ? is it smarter to instead refetch it in /maps?
    openState = false;
  }
</script>

<Modal
  open={openState}
  onOpenChange={(e) => (openState = e.open)}
  contentBase="card bg-surface-100-900 p-4 space-y-4 shadow-xl max-w-screen-sm"
  backdropClasses="backdrop-blur-sm"
>
  {#snippet trigger()}
    {@render children()}
  {/snippet}
  {#snippet content()}
    <AddOrChangePersons
      {form}
      personToUpdate={{ person: newP, personId , record: personToUpdate.record ?? undefined}}
      bind:openState
    />
    <button
      onclick={async () => {
        await deletePerson(personId);
        console.log("DELETED!!");
      }}
      class="btn preset-tonal-error"
    >
      <p>
        Delete <b>{personToUpdate.firstName}</b>
      </p>
    </button>
  {/snippet}
</Modal>
