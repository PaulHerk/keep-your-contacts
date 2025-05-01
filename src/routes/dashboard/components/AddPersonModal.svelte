<script lang="ts">
  import { Modal } from "@skeletonlabs/skeleton-svelte";
  import ChangePersons from "./ChangePersons.svelte";
  import type { ActionData } from "../$types";

  let {
    form,
    personCount,
    perPage = $bindable(),
    openState = $bindable(),
  }: {
    form: ActionData;
    personCount: number;
    perPage: number;
    openState: boolean;
  } = $props();
</script>

<header class="flex justify-between">
  <h2 class="h2">Add Person</h2>
</header>
<ChangePersons {form} {personCount} bind:perPage bind:openState />
