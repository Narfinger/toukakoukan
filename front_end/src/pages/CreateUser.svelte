<script lang="ts">
    import { push } from "svelte-spa-router";
    import { ENDPOINT_CREATE_USER } from "../js/endpoints.js";

    let username: string, password: string;

    async function handleCreate() {
        const res = await fetch(ENDPOINT_CREATE_USER, {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ username: username, password: password }),
            credentials: "include",
        });
        console.log(res);
        push("/");
    }
</script>

<div class="p-8 container mx-auto">
    <p class="text-6xl pb-4">Create User</p>
    <form on:submit={handleCreate}>
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
            <button class="btn btn-primary lg:w-32 md:w-32" type="submit"
                >Create User</button
            >
        </div>
    </form>
</div>
