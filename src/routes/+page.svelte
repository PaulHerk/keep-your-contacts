<script lang="ts">
    import { add_persons, get_persons } from "$lib/index";
    import "../components/Table.svelte";
    import "../style.css";
    import { onMount } from "svelte";
    import { browser } from "$app/environment";
    import Table from "../components/Table.svelte";
    import { persons } from "../state.svelte";
    import AddPersonPopup from "../components/AddPersonPopup.svelte";

    onMount(async () => {
        if (browser) {
            get_persons().then((data) => {
                persons.slice();
                persons.push(...data);
            });
        }
    });
    let addPersonPopup: HTMLDialogElement;
</script>

<button on:click={() => addPersonPopup.showModal()}> Add Person </button>
<AddPersonPopup bind:addPersonPopup />

<Table />
