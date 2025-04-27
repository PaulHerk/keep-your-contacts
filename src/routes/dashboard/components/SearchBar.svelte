<script lang="ts">
  import { enhance } from "$app/forms";
  import { Search } from "@lucide/svelte";

  let { filterTerm = $bindable(), page = $bindable() } = $props();

  let searchTerm: string = $state("");

  function search() {
    if (searchTerm.trim() === "") filterTerm = "";
    else {
      filterTerm = "&global_search=" + searchTerm;
      page = 0;
    }
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
      bind:value={searchTerm}
      oninput={(e) => {
        if (e.currentTarget.value === "") filterTerm = "";
      }}
    />
    <button class="ig-btn preset-filled" onclick={search}>Search</button>
  </div>
</form>
