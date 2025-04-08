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
    $inspect(persons);
</script>

{#snippet sortingH1(header: string)}
    <button
        onclick={() => {
            if (sortBy === (header as keyof Person)) {
                sortAsc = !sortAsc;
            }
            sortBy = header as keyof Person;
        }}
    >
        <div>
            <p>
                sort by
                <b>{header}</b>
            </p>
        </div>
    </button>
{/snippet}

{#if browser && persons.length > 0}
    <div class="columnContainer">
        <div class="rowContainer">
            {@render sortingH1("added_date")}
            {#each Object.keys(persons[0]).slice(1) as header}
                {@render sortingH1(header)}
            {/each}
        </div>
        {#each sortedPersons as person, id}
            <div class="rowContainer">
                <button
                    onclick={() => {
                        delete_person(person.id).then(() => {
                            let index = persons.findIndex(
                                (p) => p.id === person.id,
                            );
                            persons.splice(index, 1);
                        });
                    }}
                >
                    <p>Delete <b>{id}</b></p>
                </button>
                {#each Object.values(person).slice(1) as attribute}
                    <p>{attribute}</p>
                {/each}
            </div>
        {/each}
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
