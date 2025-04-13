<script lang="ts">
    import { browser } from "$app/environment";
    import { persons } from "../state.svelte";
    import EditModal from "./EditModal.svelte";

    let sortByFirst: keyof Person = $state("first_name");
    let sortBySec: keyof Person | null = $state("job");
    let sortAscFirst: boolean = $state(true);
    let sortAscSec: boolean = $state(true);

    let sortedPersons = $derived(
        [...persons].sort((a, b) => {
            if (a[sortByFirst] === b[sortByFirst] && sortBySec) {
                return a[sortBySec] > b[sortBySec] == sortAscSec ? 1 : -1;
            }
            return a[sortByFirst] > b[sortByFirst] == sortAscFirst ? 1 : -1;
        }),
    );

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

{#if browser && persons.length > 0}
    <div class="table-wrap">
        <table class="table table-fixed">
            <thead>
                <tr>
                    <th>
                        {@render sortingH1("First Added", "id")}
                    </th>
                    <th>{@render sortingH1("First Name", "first_name")}</th>
                    <th>{@render sortingH1("Last Name", "last_name")}</th>
                    <th>{@render sortingH1("City", "city")}</th>
                    <th>{@render sortingH1("Job", "job")}</th>
                    <th>{@render sortingH1("Notes", "note")}</th>
                </tr>
            </thead>
            <tbody class="[&>tr]:hover:preset-tonal-primary">
                {#each sortedPersons as person}
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
