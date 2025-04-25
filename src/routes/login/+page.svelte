<script lang="ts">
    import { browser } from "$app/environment";
    import { goto } from "$app/navigation";
    import type { PageProps } from "./$types";

    let { form }: PageProps = $props();
    if (form?.success && browser) goto("/dashboard");
</script>

<form method="POST" action="?/login">
    {#if form?.wrongCredentials}
        <p class="error">Wrong credentials!</p>
    {/if}
    {#if form?.missing}
        <p class="error">Email is missing!</p>{/if}
    {#if form?.userTaken}
        <p class="error">This user already exists!</p>{/if}
    <label>
        Name
        <input name="name" type="text" required />
    </label>
    <label>
        Email
        <input name="email" type="email" value={form?.email ?? ""} />
    </label>
    <label>
        Password
        <input name="password" type="password" required />
    </label>
    <button>Log in</button>
    <button formaction="?/register">Register</button>
</form>

{#if form && !form.success}
    <b>{form.message}</b>
{:else if form && form.success}
    <b>{form.message}</b>
{/if}
