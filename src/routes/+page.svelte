<script lang="ts">
    import { put } from "$lib/index";
    import { page } from "$app/state";
    import { onMount } from "svelte";
    let list = [
        {
            name: "John",
            known_from: "kpmg",
            position: "intern",
            city: "Berlin",
        },
        {
            name: "Doe",
            known_from: "university",
            position: "student",
            city: "Munich",
        },
        {
            name: "Alice",
            known_from: "conference",
            position: "manager",
            city: "Hamburg",
        },
        {
            name: "Bob",
            known_from: "workshop",
            position: "developer",
            city: "Frankfurt",
        },
        {
            name: "Eve",
            known_from: "meetup",
            position: "designer",
            city: "Stuttgart",
        },
    ];
    onMount(() => {
        console.log(put("test"));
    });
</script>

<div class="columnContainer">
    <div class="rowContainer">
        {#each Object.keys(list[0]) as header}
            <h1>{header}</h1>
        {/each}
    </div>
    {#each list as person, index}
        <div class="rowContainer">
            <p>{index}</p>
            <p>{person.name}</p>
            <p>{person.known_from}</p>
            <p>{person.position}</p>
            <p>{person.city}</p>
        </div>
    {/each}
</div>

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
