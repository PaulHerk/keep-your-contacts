<script lang="ts">
    import { get_persons } from "$lib/index";
    import "../components/Table.svelte";
    import "../style.css";
    import { onMount } from "svelte";
    import { browser } from "$app/environment";
    import Table from "../components/Table.svelte";
    import { persons } from "../state.svelte";
    import AddPerson from "../components/AddPerson.svelte";

    onMount(() => {
        if (browser) {
            get_persons().then((data) => {
                persons.push(...data);
            });
        }
    });
</script>

<p
    class="bg-gradient-to-r from-pink-500 to-violet-500 bg-clip-text text-5xl font-extrabold text-transparent ..."
>
    Know Your Contacts
</p>

<Table />
<AddPerson />
