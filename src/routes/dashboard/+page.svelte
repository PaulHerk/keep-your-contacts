<script lang="ts">
  import "./components/Table.svelte";
  import "../../style.css";
  import Table from "./components/Table.svelte";
  import AddPerson from "./components/AddPerson.svelte";
  import type { PageProps } from "./$types";
  import { api_request } from "$lib";
  import { persons } from "./store";
  import SearchBar from "./components/SearchBar.svelte";
  import { error } from "@sveltejs/kit";
  import { untrack } from "svelte";

  let { data, form }: PageProps = $props();

  let windowInnerWidth = $state(0);
  let page = $state(0);
  let perPage = $state(10);
  let detailed = $derived(windowInnerWidth > 800);
  let filterTerm = $state("");
  let personCount = $state(data.personCount);

  let knownFromSources: KnownFromSource[] = $state([]);

  $effect(() => {
    api_request(
      fetch,
      `/persons?page=${page}&per_page=${perPage}&detailed=true${filterTerm}` // I have decided to not call the api when detailed changes, it is rather a frontend thing not backend, I will however preserve the query parameter in the api
    ).then(async (response) => {
      if (!response.ok) error(500, await response.text());
      let person: Person[] = await response.json();
      untrack(() => persons.set(person));
    });
  });

  $inspect($persons);
  $inspect(form);
</script>

<svelte:window bind:innerWidth={windowInnerWidth} />

<div class="grid h-screen grid-rows-[auto_1fr_auto]">
  <!-- Header -->
  <header class="p-4">
    <p
      class="bg-gradient-to-r from-pink-500 to-violet-500 bg-clip-text text-5xl font-extrabold text-transparent ..."
    >
      Know Your Contacts
    </p>
  </header>
  <!-- Grid Columns -->
  <div class="grid grid-cols-1 md:grid-cols-[auto_1fr]">
    <!-- Left Sidebar. -->
    <aside class="p-4"></aside>
    <!-- Main Content -->
    <main class="space-y-4 p-4">
      <div class="flex flex-col gap-4">
        <div class="self-center flex flex-col md:flex-row gap-8 w-full">
          <AddPerson
            {form}
            bind:knownFromSources
            bind:personCount
            bind:perPage
          />
          <SearchBar {knownFromSources} bind:filterTerm bind:page />
        </div>
        <Table {detailed} {personCount} bind:perPage bind:page>
          <!-- {#snippet editPersonModal(personToUpdate: Person)}
            <EditPersonModal
              {form}
              {personToUpdate}
              bind:knownFromSources
              bind:personCount
              bind:perPage
            />
          {/snippet} -->
        </Table>
      </div>
    </main>
  </div>
  <!-- Footer
    <footer class="p-4">This is a cool footer</footer> -->
</div>
