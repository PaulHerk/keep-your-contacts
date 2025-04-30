<script lang="ts">
  import { Modal } from "@skeletonlabs/skeleton-svelte";
  import ChangePersons from "./ChangePersons.svelte";
  import type { ActionData } from "../$types";

  let {
    form,
    knownFromSources = $bindable([]),
    personCount = $bindable(),
    perPage = $bindable(),
  }: {
    form: ActionData;
    knownFromSources: KnownFromSource[];
    personCount: number;
    perPage: number;
  } = $props();

  let openState = $state(false);
</script>

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
    <ChangePersons
      {form}
      bind:knownFromSources
      bind:personCount
      bind:perPage
      bind:openState
    />
  {/snippet}
</Modal>
