<script>
    import { user } from "./../js/store.js";
    import { getSession, postLogin } from "./../js/auth";
    import Expenses from "./Expenses.svelte";

    let username, password;
    let errorMessage = "";

    async function handleLogin() {
        let loginResponse = await postLogin(username, password);
        if (loginResponse.result == "error") {
            errorMessage = loginResponse.message;
        } else {
            getSession();
        }
    }
</script>

{#if !$user}
    SOMETHING IS WRONG
    {#if errorMessage}
        <div>
            {errorMessage}
        </div>
    {/if}
    <div>
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
