<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { Expense, Group, GroupResponse, User } from "../js/types";
    import { push } from "svelte-spa-router";
    import {
        ENDPOINT_CREATE_GROUP,
        ENDPOINT_EXPENSES,
        ENDPOINT_GET_USERS,
        ENDPOINT_GROUP,
        ENDPOINT_TOTAL,
    } from "../js/endpoints";

    export const params: any = {};

    async function getUsers(): Promise<Array<User>> {
        let response = await fetch(ENDPOINT_GET_USERS);
        let users = await response.json();
        return users;
    }
    const users = getUsers();

    let name = "";
    let user_checked: Map<Number, boolean> = new Map();
    users.then((users) => {
        users.forEach((u) => {
            user_checked.set(u.id, false);
        });
    });

    async function handleAdd() {
        let selected_user_ids: Array<Number> = [];
        user_checked.forEach((value, key, _) => {
            if (value) {
                selected_user_ids.push(key);
            }
        });
        let data = { name: name, users: selected_user_ids };
        await fetch(ENDPOINT_CREATE_GROUP, {
            method: "POST",
            body: JSON.stringify(data),
            headers: {
                "content-type": "application/json",
            },
        });
    }
</script>

<div class="flex flex-col p-8 justify-center">
    <h1 class="underline">Add an expense</h1>
    <form class="shadow-md rounded px-8 pt-6 pb-8 mb-4">
        <div class="flex flex-col p8">FIX THIS TO ADD A GROUP!!!</div>
        <label for="name-field">Name</label>
        <input id="name-field" class="input-bordered" bind:value={name} />
        {#await users then users}
            {#each users as u}
                <div class="row">
                    <label>{u.name}</label>
                    <input
                        id={u.id.toString()}
                        on:click={() =>
                            user_checked.set(u.id, !user_checked.get(u.id))}
                        type="checkbox"
                    />
                </div>
            {/each}
        {/await}
    </form>
    <button on:click={handleAdd}>Add</button>
</div>
