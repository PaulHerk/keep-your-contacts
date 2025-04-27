<script lang="ts">
  import { browser } from "$app/environment";
  import { orderBy } from "natural-orderby";
  import EditModal from "./EditModal.svelte";
  import { getContext } from "svelte";
  import { persons } from "../store";
  import { Pagination } from "@skeletonlabs/skeleton-svelte";
  import IconArrowLeft from "@lucide/svelte/icons/arrow-left";
  import IconArrowRight from "@lucide/svelte/icons/arrow-right";
  import IconEllipsis from "@lucide/svelte/icons/ellipsis";
  import IconFirst from "@lucide/svelte/icons/chevrons-left";
  import IconLast from "@lucide/svelte/icons/chevron-right";

  let {
    detailed,
    personCount,
    perPage = $bindable(),
    page = $bindable(),
  } = $props();

  let sortByFirst: keyof Person = $state("firstName");
  let sortBySec: keyof Person | null = $state("lastName");
  let sortAscFirst: boolean = $state(true);
  let sortAscSec: boolean = $state(true);

  const regionNames = new Intl.DisplayNames(["en"], { type: "region" });

  // let persons: Person[] | SimplePerson[] = getContext("persons");

  function sortPersons(attribute: keyof Person) {
    if (sortByFirst === attribute) {
      sortAscFirst = !sortAscFirst;
    } else {
      if (sortBySec === attribute) {
        if (sortAscSec) sortAscSec = false;
        else {
          sortAscSec = true;
          sortBySec = null;
          sortByFirst = attribute;
        }
      } else {
        sortBySec = attribute;
      }
    }
  }
</script>

{#snippet sortingH1(header: string, attribute: keyof Person)}
  <button
    onclick={() => {
      sortPersons(attribute);
    }}
  >
    <div>
      <p>
        <b>{header}</b>
      </p>
      {#if sortByFirst === attribute}
        {sortAscFirst ? "1st ↑" : "1st ↓"}
      {/if}
      {#if sortBySec === attribute && sortAscSec !== null}
        {sortAscSec ? "2nd ↑" : "2nd ↓"}
      {/if}
    </div>
  </button>
{/snippet}

{#if browser && $persons.length > 0}
  <section class="space-y-4">
    <div class="table-wrap">
      <table class="table table-auto md:table-fixed">
        <thead>
          <tr>
            <th>
              {@render sortingH1("Added First", "createdAt")}
            </th>
            <th>{@render sortingH1("First Name", "firstName")}</th>
            <th>{@render sortingH1("Last Name", "lastName")}</th>
            {#if detailed}
              <th>{@render sortingH1("City", "company")}</th>
              <th>{@render sortingH1("Job", "jobTitle")}</th>
              <th>{@render sortingH1("Notes", "notes")}</th>
            {/if}
            <th>{@render sortingH1("Record", "record")}</th>
          </tr>
        </thead>
        <tbody class="[&>tr]:hover:preset-tonal-primary">
          {#each $persons as person, idx}
            <tr>
              <td>
                <EditModal personID={idx} />
              </td>
              <td><p class="text-wrap">{person.firstName}</p></td>
              <td><p class="text-wrap">{person.lastName}</p></td>
              {#if detailed}
                <td><p class="text-wrap">{person.company}</p></td>
                <td><p class="text-wrap">{person.jobTitle}</p></td>
                <td><p class="text-wrap">{person.notes}</p></td>
              {/if}
              <td
                ><p class="text-wrap">
                  {person.record
                    ? regionNames.of(person.record?.cc) +
                      ", " +
                      person.record.admin1
                    : ""}
                </p></td
              >
            </tr>
          {/each}
        </tbody>
      </table>
    </div>

    <footer class="flex flex-col sm:flex-row justify-between">
      <select
        name="size"
        id="size"
        class="select max-w-[150px]"
        bind:value={perPage}
      >
        {#each [1, 2, 5, 10] as v}
          <option value={v}>Items {v}</option>
        {/each}
        <option value={personCount}>Show All</option>
      </select>
      <!-- Pagination -->
      <Pagination
        data={$persons}
        count={$persons.length < personCount ? personCount : $persons.length}
        onPageChange={(e) => (page = e.page - 1)}
        page={page + 1}
        pageSize={perPage}
        onPageSizeChange={(e) => (perPage = e.pageSize)}
        siblingCount={4}
        alternative
      >
        {#snippet labelEllipsis()}<IconEllipsis class="size-4" />{/snippet}
        {#snippet labelNext()}<IconArrowRight class="size-4" />{/snippet}
        {#snippet labelPrevious()}<IconArrowLeft class="size-4" />{/snippet}
        {#snippet labelFirst()}<IconFirst class="size-4" />{/snippet}
        {#snippet labelLast()}<IconLast class="size-4" />{/snippet}
      </Pagination>
    </footer>
  </section>
{/if}
