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
    <h1 class="underline">Add an expense</h1>
    <form class="shadow-md rounded px-8 pt-6 pb-8 mb-4">
        <div class="flex flex-col p8">
            <label for="amount-field">Amount</label>
            <input
                id="amount-field"
                class="input-bordered"
                type="number"
                bind:value={amount}
            />
            <input
                class="input-bordered"
                placeholder="description"
                bind:value={description}
            />
            {#await group then g}
                <select bind:value={who}>
                    {#each g.users as p, item}
                        <option value={"EvenSplit " + item}
                            >{p} payed, Split 50/50</option
                        >
                        <option value={"OwedTotal " + item}
                            >{p} is owed the full amount</option
                        >
                    {/each}
                </select>
            {/await}
        </div>
    </form>
    <button class="btn btn-primary" on:click={handleAdd}>Add</button>
    <button
        class="btn btn-warning"
        on:click={() => {
            push("/expenses");
        }}>Cancel</button
    >
</div>
