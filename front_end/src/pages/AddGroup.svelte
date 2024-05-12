<script lang="ts">
    import { push } from "svelte-spa-router";
    import { ENDPOINT_CREATE_GROUP } from "../js/endpoints";
    import { getUser, getUsers } from "../js/api";

    const users = getUsers();
    const user = getUser();

    let name = "";
    let user_checked: Map<Number, boolean> = new Map();
    users.then((users) => {
        users.forEach((u) => {
            user_checked.set(u.id, false);
        });
        user.then((u) => user_checked.set(u.id, true));
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
        push("/expenses");
    }
</script>

<div class="flex flex-col p-8 justify-center">
    <p class="text-2xl lg:text-6xl pb-4">Add an expense</p>
    <form>
        <div class="flex flex-col">
            <div class="p-2">
                <input
                    id="name-field"
                    class="input-bordered"
                    bind:value={name}
                    placeholder="name"
                />
                {#await users}
                    <span class="loading loading-dots loading-lg"></span>
                {:then users}
                    {#each users as u}
                        <div class="row">
                            <label for={u.id.toString()}>{u.name}</label>
                            <input
                                id={u.id.toString()}
                                on:click={() =>
                                    user_checked.set(
                                        u.id,
                                        !user_checked.get(u.id),
                                    )}
                                type="checkbox"
                            />
                        </div>
                    {/each}
                {/await}
            </div>
        </div>
    </form>
    <div class="flex flex-col">
        <div class="p-2">
            <button class="btn btn-primary w-full" on:click={handleAdd}
                >Add</button
            >
        </div>
        <div class="p-2 grow">
            <button
                class="btn btn-warning w-full"
                on:click={() => {
                    push("/expenses");
                }}>Cancel</button
            >
        </div>
    </div>
</div>
