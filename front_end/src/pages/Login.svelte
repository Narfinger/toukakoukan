<script>
    import { user } from "./../js/store.js";
    import { getSession, postLogin } from "./../js/auth";
    import Expenses from "./Expenses.svelte";
    import { createEventDispatcher } from "svelte";

    let isProduction = import.meta.env.MODE === "production";

    let username, password;
    let errorMessage = "";

    async function handleLogin() {
        if (!isProduction) {
            getSession();
            window.location.hash = "expenses";
            return;
        }
        let loginResponse = await postLogin(username, password);
        if (loginResponse.result == "error") {
            errorMessage = loginResponse.message;
        } else {
            getSession();
            window.location.hash = "expenses";
        }
    }
</script>

{#if !$user}
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
{:else}
    <Expenses />
{/if}
