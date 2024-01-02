<script>
    import { user } from "./../js/store.js";
    import { getSession, postLogin } from "./../js/auth";
    import Expenses from "./Expenses.svelte";
    import { createEventDispatcher } from "svelte";
    import { push, pop, replace } from "svelte-spa-router";

    let isProduction = import.meta.env.MODE === "production";

    let username, password;
    let errorMessage = "";

    async function handleLogin() {
        if (!isProduction) {
            push("/expenses");
            return;
        }
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
<div>
    <h2>please login</h2>
    <div>
        <label for="username">Username</label>
        <input
            class="input"
            type="username"
            placeholder="username"
            bind:value={username}
        />
        <label for="password">Password</label>
        <input
            class="input"
            type="password"
            placeholder="password"
            bind:value={password}
        />
        <button on:click={handleLogin}> Login </button>
    </div>
</div>
