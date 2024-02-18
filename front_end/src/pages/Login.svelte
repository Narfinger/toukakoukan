<script>
    import { user } from "./../js/store.js";
    import { getSession, postLogin } from "./../js/auth";
    import Expenses from "./Expenses.svelte";
    import { createEventDispatcher } from "svelte";
    import { push, pop, replace } from "svelte-spa-router";

    let username, password;
    let errorMessage = "";

    async function handleLogin() {
        let loginResponse = await postLogin(username, password);
        if (loginResponse.result == "error") {
            errorMessage = loginResponse.message;
        } else {
            getSession();
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
    <h1>Login</h1>
    <div class="p-8">
        <input
            class="input input-bordered input-primary"
            type="username"
            placeholder="Username"
            bind:value={username}
        />
        <input
            class="input input-bordered input-secondary"
            type="password"
            placeholder="Password"
            bind:value={password}
        />
        <button class="btn btn-primary w-64" on:click={handleLogin}
            >Login</button
        >
    </div>
    <div>
        or <button
            on:click={() => {
                push("/createuser/");
            }}>Create a User</button
        >
    </div>
</div>
