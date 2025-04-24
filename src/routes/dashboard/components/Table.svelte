<script lang="ts">
    import { browser } from "$app/environment";
    import { orderBy } from "natural-orderby";
    import EditModal from "./EditModal.svelte";
    import { getContext } from "svelte";
    import { persons } from "../store";

    let sortByFirst: keyof Person = $state("first_name");
    let sortBySec: keyof Person | null = $state("last_name");
    let sortAscFirst: boolean = $state(true);
    let sortAscSec: boolean = $state(true);

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
                        {@render sortingH1("First Added", "id")}
                    </th>
                    <th>{@render sortingH1("First Name", "first_name")}</th>
                    <th>{@render sortingH1("Last Name", "last_name")}</th>
                    <th>{@render sortingH1("City", "company")}</th>
                    <th>{@render sortingH1("Job", "job_title")}</th>
                    <th>{@render sortingH1("Notes", "notes")}</th>
                </tr>
            </thead>
            <tbody class="[&>tr]:hover:preset-tonal-primary">
                {#each $persons as person}
                    <tr>
                        <td>
                            <EditModal personID={person.id} />
                        </td>
                        {#each Object.values(person).slice(1) as attribute}
                            <td><p class="text-wrap">{attribute}</p></td>
                        {/each}
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}
