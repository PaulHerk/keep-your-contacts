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
        if (browser && persons.length === 0) {
            get_persons().then((data) => {
                persons.push(...data);
            });
        }
    });
</script>

<div class="grid h-screen grid-rows-[auto_1fr_auto]">
    <!-- Header -->
    <header class="p-4">
        <p
            class="bg-gradient-to-r from-pink-500 to-violet-500 bg-clip-text text-5xl font-extrabold text-transparent ..."
        >
            Know Your Contacts
        </p>
    </header>
    <!-- Grid Columns -->
    <div class="grid grid-cols-1 md:grid-cols-[auto_1fr]">
        <!-- Left Sidebar. -->
        <aside class="p-4"></aside>
        <!-- Main Content -->
        <main class="space-y-4 p-4">
            <div class="flex flex-col gap-4">
                <div class="self-center">
                    <AddPerson />
                </div>
                <Table />
            </div>
        </main>
    </div>
    <!-- Footer
    <footer class="p-4">This is a cool footer</footer> -->
</div>
