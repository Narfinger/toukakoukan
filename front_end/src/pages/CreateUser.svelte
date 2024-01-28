<script>
    import { user } from "./../js/store.js";
    import { getSession, postLogin } from "./../js/auth";
    import Expenses from "./Expenses.svelte";
    import { createEventDispatcher } from "svelte";
    import { push, pop, replace } from "svelte-spa-router";
    import { ENDPOINT_CREATE_USER } from "../js/endpoints.js";

    let username, password;
    let errorMessage = "";

    async function handleCreate() {
        const res = await fetch(ENDPOINT_CREATE_USER, {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ username: username, password: password }),
        });

        await res;
        push("/");
    }
</script>

<div>
    <h2>please create your user</h2>
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
        <button on:click={handleCreate}> Login </button>
    </div>
</div>
