<script lang="ts">
  import { Modal } from "@skeletonlabs/skeleton-svelte";
  import ChangePersons from "./ChangePersons.svelte";
  import type { ActionData } from "../$types";
  let openState = $state(false);
  let {
    form,
    personToUpdate,
    knownFromSources = $bindable([]),
    personCount = $bindable(),
    perPage = $bindable(),
  }: {
    form: ActionData;
    personToUpdate: Person;
    knownFromSources: KnownFromSource[];
    personCount: number;
    perPage: number;
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
</script>

<Modal
  open={openState}
  onOpenChange={(e) => (openState = e.open)}
  triggerBase="btn btn-md preset-outlined-primary-500"
  backdropClasses="backdrop-blur-sm"
>
  {#snippet trigger()}Edit{/snippet}
  {#snippet content()}
    <ChangePersons
      {form}
      personToUpdate={{ person: newP, personId }}
      bind:knownFromSources
      bind:personCount
      bind:perPage
      bind:openState
    />
    <button
      onclick={() => {
        // delete_person(personID).then(() => {
        //     persons.splice(index, 1);
        //     modalClose();
        // });
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
