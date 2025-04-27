<script lang="ts">
  // import { delete_person } from "$lib";
  import { Modal } from "@skeletonlabs/skeleton-svelte";
  import { getContext } from "svelte";
  import { persons } from "../store";
  let openState = $state(false);
  let { personID } = $props();

  function modalClose() {
    openState = false;
  }
  let index = $derived($persons.findIndex((p) => p.id === personID));
</script>

<Modal
  open={openState}
  onOpenChange={(e) => (openState = e.open)}
  triggerBase="btn btn-md preset-outlined-primary-500"
  backdropClasses="backdrop-blur-sm"
>
  {#snippet trigger()}Edit{/snippet}
  {#snippet content()}
    {#each Object.entries($persons[personID]) as [key, value]}
      <p><b>{key}</b> : {value}</p>
    {/each}
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
        Delete <b>{$persons[index]?.firstName}</b>
      </p>
    </button>
  {/snippet}
</Modal>
