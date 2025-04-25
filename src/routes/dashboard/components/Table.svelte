<script lang="ts">
  import { browser } from "$app/environment";
  import { orderBy } from "natural-orderby";
  import EditModal from "./EditModal.svelte";
  import { getContext } from "svelte";
  import { persons } from "../store";

  let { detailed } = $props();

  let sortByFirst: keyof Person = $state("first_name");
  let sortBySec: keyof Person | null = $state("last_name");
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
  <div class="table-wrap">
    <table class="table table-fixed">
      <thead>
        <tr>
          <th>
            {@render sortingH1("Added First", "createdAt")}
          </th>
          <th>{@render sortingH1("First Name", "first_name")}</th>
          <th>{@render sortingH1("Last Name", "last_name")}</th>
          {#if detailed}
            <th>{@render sortingH1("City", "company")}</th>
            <th>{@render sortingH1("Job", "job_title")}</th>
            <th>{@render sortingH1("Notes", "notes")}</th>
          {/if}
          <th>{@render sortingH1("Record", "record")}</th>
        </tr>
      </thead>
      <tbody class="[&>tr]:hover:preset-tonal-primary">
        {#each $persons as person}
          <tr>
            <td>
              <EditModal personID={person.id} />
            </td>
            <td><p class="text-wrap">{person.first_name}</p></td>
            <td><p class="text-wrap">{person.last_name}</p></td>
            {#if detailed}
              <td><p class="text-wrap">{person.company}</p></td>
              <td><p class="text-wrap">{person.job_title}</p></td>
              <td><p class="text-wrap">{person.notes}</p></td>
            {/if}
            <td><p class="text-wrap">{regionNames.of(person.record.cc)}</p></td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
{/if}
