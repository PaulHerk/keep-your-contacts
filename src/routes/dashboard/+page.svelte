<script lang="ts">
	import "./components/Table.svelte";
	import Table from "./components/Table.svelte";
	import EditPersonModal from "./components/EditPersonModal.svelte";
	import type { PageProps } from "./$types";
	import { api_request } from "$lib";
	import {
		authToken,
		knownFromSources,
		persons,
		prevQueryParams,
		settings
	} from "./store";
	import SearchBar from "./components/SearchBar.svelte";
	import { error } from "@sveltejs/kit";
	import { onMount, untrack } from "svelte";
	import { Modal } from "@skeletonlabs/skeleton-svelte";
	import AddOrChangePersons from "./components/AddOrChangePersons.svelte";
	import EditKfs from "./components/EditKFS.svelte";

	let { data, form }: PageProps = $props();
	authToken.set(data.token);
	settings.set(data.loggedInUser.settings)

	let windowInnerWidth = $state(0);
	let page = $state(0);
	let perPage = $derived($settings.perPage);
	let detailed = $derived(windowInnerWidth > 800);
	let filterTerm = $state("");

	let openStateAddP = $state(false);

	onMount(() => ($knownFromSources = data.knownFromSources));

	$effect(() => {
		const queryParams = `/persons?page=${page}&per_page=${perPage}&detailed=true${filterTerm}`;
		// Only fetch if query params changed or store is empty (TODO: why though)
		if (queryParams !== $prevQueryParams) {
			api_request(fetch, queryParams, {}, $authToken).then(
				async (response) => {
					if (!response.ok)
						error(
							500,
							await response.text(),
						);
					let person: Person[] =
						await response.json();
					untrack(() => {
						persons.set(person);
						prevQueryParams.set(
							queryParams,
						);
					});
				},
			);
		}
	});

	$inspect($persons);
</script>

<svelte:window bind:innerWidth={windowInnerWidth} />

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
				<div
					class="self-center flex flex-col md:flex-row gap-8 w-full"
				>
					<Modal
						open={openStateAddP}
						onOpenChange={(e) =>
							(openStateAddP =
								e.open)}
						triggerBase="btn btn-lg preset-filled-tertiary-500"
						contentBase="card bg-surface-100-900 p-4 space-y-4 shadow-xl max-w-screen-sm"
						backdropClasses="backdrop-blur-sm"
					>
						{#snippet trigger()}Add Person{/snippet}
						{#snippet content()}
							<AddOrChangePersons
								{form}
								bind:openState={
									openStateAddP
								}
							/>
						{/snippet}
					</Modal>

					<SearchBar bind:filterTerm bind:page />

					<EditKfs
						kfsIds={[
							...Array(
								$knownFromSources.length,
							).keys(),
						]}
					/>
				</div>
				<Table
					{detailed}
					personCount={data.personCount}
					bind:perPage
					bind:page
				>
					{#snippet editPersonModal(
						personToUpdate: Person,
					)}
						{#key personToUpdate}
							<!-- This key is necessary so that the person values in the EditModal update on repeated editing -->
							<EditPersonModal
								{form}
								{personToUpdate}
							>
								<button
									class="btn btn-md preset-outlined-primary-500"
								>
									Edit
								</button>
							</EditPersonModal>
						{/key}
					{/snippet}
				</Table>
			</div>
		</main>
	</div>
	<!-- Footer
    <footer class="p-4">This is a cool footer</footer> -->
</div>
