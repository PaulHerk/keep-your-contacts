<script lang="ts">
    import { add_persons, get_persons } from "$lib/index";
    import { page } from "$app/state";
    import { onMount } from "svelte";
    import { browser } from "$app/environment";

    let list = [
        {
            name: "John",
            known_from: "kpmg",
            position: "intern",
            city: "Berlin",
        },
    ];

    onMount(async () => {
        if (browser) {
            add_persons("John", "", "Berlin", "hi");
            get_persons().then((data) => {
                console.log(data);
            });
        }
        // console.log(put("test"));
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
