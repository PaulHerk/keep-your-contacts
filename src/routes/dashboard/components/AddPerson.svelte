<script lang="ts">
  import { Modal } from "@skeletonlabs/skeleton-svelte";
  import { enhance } from "$app/forms";
  import type { PageProps } from "../$types";

  let newPerson: NewPerson | undefined = $state();
  let openState = $state(false);

  function modalClose() {
    openState = false;
  }
</script>

{#snippet input(key: keyof NewPerson, required = false)}
  <label class="label">
    <span class="label-text">{key}</span>
    <input class="input" type="text" placeholder={key} name={key} {required} />
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
      <article>
        {@render input("name", true)}
        <!-- {@render input("known_from_source_id")} -->
        {@render input("coordinateOrOsmSearch")}
        {@render input("jobTitle")}
        {@render input("company")}
        {@render input("linkedin")}
        {@render input("notes")}
      </article>
      <footer class="flex justify-end gap-4">
        <button type="button" class="btn preset-tonal" onclick={modalClose}
          >Cancel</button
        >
        <input type="submit" value="Confirm" class="btn preset-filled" />
        <!-- <button class="btn preset-filled">Confirm</button> -->
      </footer>
    </form>
  {/snippet}
</Modal>
