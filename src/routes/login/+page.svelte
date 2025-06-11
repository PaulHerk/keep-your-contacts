<script lang="ts">
	import { browser } from "$app/environment";
	import { enhance } from "$app/forms";
	import { goto } from "$app/navigation";
	import type { PageProps } from "./$types";

	let { form: form2 }: PageProps = $props();
	if (form2?.success && browser) goto("/dashboard");
</script>

<form method="POST" action="?/login" use:enhance class="h-full m-10 flex flex-col md:flex-row gap-10 content-center justify-center align-middle">
	{#if form2?.wrongCredentials}
		<p class="error">Wrong credentials!</p>
	{/if}
	{#if form2?.emailMissing}
		<p class="error">Email is missing!</p>{/if}
	{#if form2?.userTaken}
		<p class="error">This user already exists!</p>{/if}
	<label>
		Name
		<input name="name" type="text" class="input" required />
	</label>
	<label>
		Email
		<input name="email" type="email" class="input"/>
	</label>
	<label>
		Password
		<input name="password" type="password" class="input" required />
	</label>
	<button class="btn preset-tonal-primary">Log in</button>
	<button class="btn preset-tonal-secondary" formaction="?/register">Register</button>
</form>

{#if form2 && !form2.success}
	<b>{form2.message}</b>
{:else if form2 && form2.success}
	<b>{form2.message}</b>
{/if}
