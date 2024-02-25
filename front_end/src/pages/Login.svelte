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
    <p class="text-6xl pb-4">Login</p>
    <div class="grid lg:grid-cols-2 gap-4">
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
        <button class="btn btn-primary lg:w-32 md:w-32" on:click={handleLogin}
            >Login</button
        >
        <div class="lg:col-span-2">
            or <button
                on:click={() => {
                    push("/createuser/");
                }}>Create a User</button
            >
        </div>
    </div>
</div>
