<script lang="ts">
    import { onMount } from "svelte";
    import { api_url } from "../state.svelte";

    const login = async (name: String, password: String) => {
        try {
            const response = await fetch(`${api_url}/auth/login`, {
                credentials: "include",
                headers: { "Content-Type": "application/json" },
                method: "POST",
                body: JSON.stringify({ name, password }),
            });
            if (response.ok) {
                console.error(await response.text());
            } else if (response.status === 400) {
                console.error(response.text, "f");
            } else {
                console.error(response.status, "ff");
            }
        } catch (e) {
            console.error("asfi", e);
        }
    };
    onMount(async () => {
        await login("richard", "very");
    });
</script>
