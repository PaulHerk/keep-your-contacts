<script lang="ts">
    import "./components/Table.svelte";
    import "../../style.css";
    import Table from "./components/Table.svelte";
    import AddPerson from "./components/AddPerson.svelte";
    import type { PageProps } from "./$types";
    import { api_get } from "$lib";
    import { persons } from "./store";
    import { api_url } from "../state.svelte";
    let { data }: PageProps = $props();

    let page = $state(0);
    let per_page = $state(10);
    let detailed = $state(false);

    $effect(() => {
        api_get(
            `${api_url}/persons?page=${page}&per_page=${per_page}&detailed=${detailed}`,
            {},
        )
            .then(async (response) => {
                const p = await response.json();
                console.log("aslas: ", p);
                persons.set(p);
            })
            .catch((e) => {
                console.error("error: ", e);
            });
    });
    $inspect(persons);
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
