<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { createPayed } from "../js/utils";
    import type { Expense, Group, GroupResponse } from "../js/types";
    import { push } from "svelte-spa-router";
    import { ENDPOINT_EXPENSES, ENDPOINT_GROUP } from "../js/endpoints";

    export let params: any = {};
    let amount: Number, description: String, who: String;

    async function getGroup(group_id: Number): Promise<GroupResponse> {
        let response = await fetch(ENDPOINT_GROUP + group_id + "/");
        let group: GroupResponse = await response.json();
        return group;
    }
    const group = getGroup(params.id);

    async function handleAdd() {
        // if we do not enter, the default who is empty which we do not do anything with
        console.log("Would have payed:" + amount);
        console.log("would descr:" + description);
        console.log("Who:" + who);
        let whosplit = who.split(" ");
        const d = new Date(Date.now());
        let data = {
            id: -1,
            payed_type: createPayed(whosplit[0], Number(whosplit[1])),
            amount: amount,
            name: description,
            time: d.toISOString(),
            expense_group_id: Number(params.id),
        };
        console.log(JSON.stringify(data));

        await fetch(ENDPOINT_EXPENSES + params.id + "/", {
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
                    id="amount-field"
                    class="input-bordered"
                    type="number"
                    placeholder="amount"
                    bind:value={amount}
                />
            </div>
            <div class="p-2">
                <input
                    class="input-bordered"
                    placeholder="description"
                    bind:value={description}
                />
            </div>
            {#await group}
                <span class="loading loading-dots loading-lg"></span>
            {:then g}
                <div class="p-2">
                    <select bind:value={who}>
                        {#each g.users as p, item}
                            <option value={"EvenSplit " + item}
                                >{p.name} payed, Split 50/50</option
                            >
                            <option value={"OwedTotal " + item}
                                >{p.name} is owed the full amount</option
                            >
                        {/each}
                    </select>
                </div>
            {/await}
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
