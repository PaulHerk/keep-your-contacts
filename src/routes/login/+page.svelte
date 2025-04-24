<script lang="ts">
    import { onMount } from "svelte";
    import { api_url } from "../state.svelte";
    import { api_get } from "$lib";

    const login = async (name: String, password: String) => {
        try {
            const response = await api_get(api_url + "/auth/login", {
                method: "POST",
                body: JSON.stringify({ name, password }),
            });
            if (response.ok) {
                console.error(await response.text(), response.headers);
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
