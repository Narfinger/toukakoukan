<script lang="ts">
    import { user_creation_enabled } from "../js/api";
    import { postLogin } from "../js/auth";
    import { push } from "svelte-spa-router";

    let username: string;
    let password: string;
    let errorMessage = "";

    const enabled = user_creation_enabled();
    async function handleLogin() {
        let loginResponse = await postLogin(username, password);
        if (!loginResponse) {
            errorMessage = "Login Denied";
            console.log("Login Denied");
        } else {
            push("/expenses");
        }
    }
</script>

{#if errorMessage}
    <div>
        <h2>Error</h2>
        {errorMessage}
    </div>
{/if}
<div class="p-8 container mx-auto">
    <p class="text-6xl pb-4">Login</p>
    <div class="grid lg:grid-cols-2 gap-4">
        <form on:submit|preventDefault={handleLogin}>
            <input
                class="input input-bordered input-primary"
                type="username"
                placeholder="Username"
                bind:value={username}
            />
            <input
                class="input input-bordered input-secondary pb-2"
                type="password"
                placeholder="Password"
                bind:value={password}
            />
            <button class="btn btn-primary lg:w-32 md:w-32" type="submit"
                >Login</button
            >
        </form>
        {#await enabled then show_creation}
            {#if show_creation}
                or <button
                    on:click={() => {
                        push("/createuser/");
                    }}>Create a User</button
                >
            {:else}
                User creation not enabled
            {/if}
        {/await}
    </div>
</div>
