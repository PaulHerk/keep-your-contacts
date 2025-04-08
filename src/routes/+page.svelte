<script lang="ts">
    import { get_persons } from "$lib/index";
    import "../components/Table.svelte";
    import "../style.css";
    import { onMount } from "svelte";
    import { browser } from "$app/environment";
    import Table from "../components/Table.svelte";
    import { persons } from "../state.svelte";
    import AddPersonPopup from "../components/AddPersonPopup.svelte";

    onMount(() => {
        if (browser) {
            get_persons().then((data) => {
                persons.push(...data);
            });
        }
    });
    let addPersonPopup: HTMLDialogElement;
</script>

<button
    onclick={() => addPersonPopup.showModal()}
    style="left: 40%; position: relative;"
    ><h1>Add Person</h1>
</button>
<AddPersonPopup bind:addPersonPopup />

<Table />
