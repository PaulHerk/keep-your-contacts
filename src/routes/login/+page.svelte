<script lang="ts">
	import { browser } from "$app/environment";
	import { enhance } from "$app/forms";
	import { goto } from "$app/navigation";
	import type { PageProps } from "./$types";

	let { form: form2 }: PageProps = $props();
	if (form2?.success && browser) goto("/dashboard");
</script>

<form method="POST" action="?/login" use:enhance>
	{#if form2?.wrongCredentials}
		<p class="error">Wrong credentials!</p>
	{/if}
	{#if form2?.emailMissing}
		<p class="error">Email is missing!</p>{/if}
	{#if form2?.userTaken}
		<p class="error">This user already exists!</p>{/if}
	<label>
		Name
		<input name="name" type="text" required />
	</label>
	<label>
		Email
		<input name="email" type="email" />
	</label>
	<label>
		Password
		<input name="password" type="password" required />
	</label>
	<button>Log in</button>
	<button formaction="?/register">Register</button>
</form>

{#if form2 && !form2.success}
	<b>{form2.message}</b>
{:else if form2 && form2.success}
	<b>{form2.message}</b>
{/if}
