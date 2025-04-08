<script lang="ts">
    import { browser } from "$app/environment";
    import { persons } from "../state.svelte";

    let sortBy: keyof Person = $state("first_name");
    let sortAsc: boolean = $state(true);

    let sortedPersons = $derived(
        [...persons].sort((a, b) => {
            return a[sortBy] > b[sortBy] == sortAsc ? 1 : -1;
        }),
    );
</script>

{#if browser && persons.length > 0}
    <div class="columnContainer">
        <div class="rowContainer">
            <!-- svelte-ignore event_directive_deprecated -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            {#each Object.keys(persons[0]) as header}
                <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                <h1
                    on:click={() => {
                        if (sortBy === (header as keyof Person)) {
                            sortAsc = !sortAsc;
                        }
                        sortBy = header as keyof Person;
                    }}
                >
                    {header}
                </h1>
            {/each}
        </div>
        {#each sortedPersons as person}
            <div class="rowContainer">
                {#each Object.values(person) as attribute}
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
