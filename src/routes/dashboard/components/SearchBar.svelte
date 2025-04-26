<script lang="ts">
  import { enhance } from "$app/forms";

  let { filterTerm = $bindable() } = $props();

  let searchTerm: string = $state("");

  function search() {
    if (searchTerm.trim() === "") filterTerm = "";
    else filterTerm = "&global_search=" + searchTerm;
  }
</script>

<form class="mx-auto w-full max-w-md space-y-4">
  <div class=" flex items-center justify-center gap-2">
    <input
      type="text"
      placeholder="Search..."
      class="input rounded-lg border border-gray-300 p-2"
      bind:value={searchTerm}
    />
    <button
      class="rounded-md bg-blue-500 w-2/5 p-2 text-white"
      onclick={search}
    >
      Search
    </button>
  </div>
</form>
