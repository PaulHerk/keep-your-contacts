<script lang="ts">
    import { add_person } from "$lib";
    import { persons } from "../state.svelte";

    export let addPersonPopup: HTMLDialogElement;
    let new_person: NewPerson = {
        first_name: "",
        last_name: "",
        city: "",
        note: "",
    };
</script>

<dialog bind:this={addPersonPopup}>
    {#each Object.keys(new_person) as key}
        <input
            type="text"
            placeholder={key}
            bind:value={new_person[key as keyof NewPerson]}
        />
    {/each}
    <button
        onclick={() => {
            try {
                add_person(new_person).then((new_person) => {
                    persons.push(new_person);
                });
                addPersonPopup.close();
            } catch (e) {
                console.error(e);
            }
        }}
    >
        <h1>Add Person</h1>
    </button>

    <button
        onclick={() => {
            addPersonPopup.close();
        }}
    >
        <h1>Close</h1>
    </button>
</dialog>
