<script lang="ts">
    import { browser } from "$app/environment";
    import { delete_person } from "$lib";
    import { persons } from "../state.svelte";

    let sortBy: keyof Person = $state("first_name");
    let sortAsc: boolean = $state(true);

    let sortedPersons = $derived(
        [...persons].sort((a, b) => {
            return a[sortBy] > b[sortBy] == sortAsc ? 1 : -1;
        }),
    );
</script>

{#snippet sortingH1(header: string, attribute: keyof Person)}
    <button
        onclick={() => {
            if (sortBy === (attribute as keyof Person)) {
                sortAsc = !sortAsc;
            }
            sortBy = attribute as keyof Person;
        }}
    >
        <div>
            <p>
                <b>{header}</b>
            </p>
            {#if sortBy === attribute}
                {sortAsc ? "↑" : "↓"}
            {/if}
        </div>
    </button>
{/snippet}

{#if browser && persons.length > 0}
    <div class="table-wrap">
        <table class="table caption-bottom">
            <thead>
                <tr>
                    <th>
                        {@render sortingH1("First Added", "id")}
                    </th>
                    <th>{@render sortingH1("First Name", "first_name")}</th>
                    <th>{@render sortingH1("Last Name", "last_name")}</th>
                    <th>{@render sortingH1("City", "city")}</th>
                    <th>{@render sortingH1("Notes", "note")}</th>
                </tr>
            </thead>
            <tbody class="[&>tr]:hover:preset-tonal-primary">
                {#each sortedPersons as person, id}
                    <tr>
                        <td>
                            <button
                                onclick={() => {
                                    delete_person(person.id).then(() => {
                                        let index = persons.findIndex(
                                            (p) => p.id === person.id,
                                        );
                                        persons.splice(index, 1);
                                    });
                                }}
                                class="btn preset-tonal-error"
                            >
                                <p>Delete <b>{id}</b></p>
                            </button>
                        </td>
                        {#each Object.values(person).slice(1) as attribute}
                            <td><p>{attribute}</p></td>
                        {/each}
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}

<style>
    .columnContainer {
        display: flex;
        justify-content: flex-start;
        align-items: center;
        height: 100vh;
        width: 90vw;
        flex-direction: column;
    }
    .rowContainer {
        display: flex;
        justify-content: space-between;
        align-items: center;
        flex-direction: row;
        width: 80vw;
    }
</style>
