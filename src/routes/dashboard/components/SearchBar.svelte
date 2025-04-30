<script lang="ts">
  import { enhance } from "$app/forms";
  import { Search } from "@lucide/svelte";

  let {
    filterTerm = $bindable(),
    page = $bindable(),
    knownFromSources,
  }: {
    filterTerm: string;
    page: number;
    knownFromSources: KnownFromSource[];
  } = $props();

  let globalSearch = $state("");
  let knownFromSearch = $state("");

  function search() {
    let search = "";
    search = globalSearch.trim() !== "" ? "&global_search=" + globalSearch : "";
    search +=
      knownFromSearch !== "" ? "&known_from_search=" + knownFromSearch : "";
    filterTerm = search;
    page = 0;
  }
</script>

<form class="mx-auto w-full max-w-md space-y-4">
  <div class="input-group grid-cols-[auto_1fr_auto]">
    <div class="ig-cell preset-tonal">
      <Search size={16} />
    </div>
    <input
      class="ig-input"
      type="search"
      placeholder="Search..."
      bind:value={globalSearch}
      oninput={(e) => {
        if (e.currentTarget.value === "") filterTerm = "";
      }}
    />
    <select class="select" onchange={search} bind:value={knownFromSearch}>
      <option value=""></option>
      {#each knownFromSources as source}
        <option value={source.sourceId}> {source.sourceName}</option>
      {/each}
    </select>
    <button class="ig-btn preset-filled" onclick={search}>Search</button>
  </div>
</form>
