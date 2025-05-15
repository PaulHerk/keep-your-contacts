<script lang="ts">
	import { api_request } from "$lib";
	import { error } from "@sveltejs/kit";
	import { authToken, settings } from "../../store";
	import { get } from "svelte/store";

	let {
		settingsDrawerState = $bindable(), personCount
	}: { settingsDrawerState: boolean; personCount: number} = $props();

	let newSettings = get(settings);

	function updateSettings() {
		api_request(
			fetch,
			"/auth/me",
			{
				method: "PUT",
				body: JSON.stringify({
					newSettings,
				}),
			},
			$authToken,
		).then(async (response) => {
			if (!response.ok) error(500, await response.text());
			settings.update((_oldSettings) => newSettings);
			settingsDrawerState = false;
		});
	}
</script>

{#snippet input(label: string, key: keyof Settings)}
	<label class="label">
		<span class="label-text">{label}</span>
		<input
			class="input"
			type="text"
			placeholder={label}
			name={key}
			bind:value={newSettings[key]}
		/>
	</label>
{/snippet}

<div class="flex flex-col gap-10">
	<select
		name="size"
		id="size"
		class="select max-w-[150px]"
		bind:value={newSettings.perPage}
	>
		{#each [1, 2, 5, 10] as v}
			<option value={v}>Items {v}</option>
		{/each}
		<option value={personCount}>Show All</option>
	</select>
	<button
		type="submit"
		class="btn preset-filled"
		onclick={updateSettings}
	>
		Update
	</button>
</div>
