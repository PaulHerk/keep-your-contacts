<script lang="ts">
    import { browser } from "$app/environment";
    import { persons } from "../state.svelte";
</script>

{#if browser && persons.length > 0}
    <div class="columnContainer">
        <div class="rowContainer">
            {#each Object.keys(persons[0]) as header}
                <h1>{header}</h1>
            {/each}
        </div>
        {#each persons as person, index}
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
