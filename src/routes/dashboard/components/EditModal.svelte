<script lang="ts">
  import { Modal } from "@skeletonlabs/skeleton-svelte";
  let openState = $state(false);
  let { person } = $props();

  function modalClose() {
    openState = false;
  }
</script>

<Modal
  open={openState}
  onOpenChange={(e) => (openState = e.open)}
  triggerBase="btn btn-md preset-outlined-primary-500"
  backdropClasses="backdrop-blur-sm"
>
  {#snippet trigger()}Edit{/snippet}
  {#snippet content()}
    {#each Object.entries(person) as [key, value]}
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
        Delete <b>{person.firstName}</b>
      </p>
    </button>
  {/snippet}
</Modal>
