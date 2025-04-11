<script lang="ts">
    import { add_person } from "$lib";
    import { onMount } from "svelte";
    import { persons } from "../state.svelte";
    import { Modal } from "@skeletonlabs/skeleton-svelte";

    let newPerson: NewPerson = $state(initializeNewPerson());
    function initializeNewPerson() {
        return {
            first_name: "",
            last_name: "",
            city: "",
            job: "",
            note: "",
        } as NewPerson;
    }
    let openState = $state(false);

    function modalClose() {
        openState = false;
    }
    function addPerson() {
        try {
            add_person(newPerson).then((new_person) => {
                persons.push(new_person);
            });
            newPerson = initializeNewPerson();
            modalClose();
        } catch (e) {
            console.error(e);
        }
    }
</script>

<Modal
    open={openState}
    onOpenChange={(e) => (openState = e.open)}
    triggerBase="btn btn-lg preset-filled-tertiary-500"
    contentBase="card bg-surface-100-900 p-4 space-y-4 shadow-xl max-w-screen-sm"
    backdropClasses="backdrop-blur-sm"
>
    {#snippet trigger()}Add Person{/snippet}
    {#snippet content()}
        <header class="flex justify-between">
            <h2 class="h2">Add Person</h2>
        </header>
        <form class="flex flex-col gap-4 p-4" onsubmit={addPerson}>
            <article>
                {#each Object.keys(newPerson) as key}
                    <label class="label">
                        <span class="label-text">{key}</span>
                        <input
                            class="input"
                            type="text"
                            placeholder={key}
                            required={true}
                            bind:value={newPerson[key as keyof NewPerson]}
                        />
                    </label>
                {/each}
            </article>
            <footer class="flex justify-end gap-4">
                <button
                    type="button"
                    class="btn preset-tonal"
                    onclick={modalClose}>Cancel</button
                >
                <input
                    type="submit"
                    value="Confirm"
                    class="btn preset-filled"
                />
            </footer>
        </form>
    {/snippet}
</Modal>
