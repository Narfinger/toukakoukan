<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { createPayed } from "../js/utils";
    import type { Expense, Group, GroupResponse } from "../js/types";
    import { push } from "svelte-spa-router";
    import {
        ENDPOINT_EXPENSES,
        ENDPOINT_GET_EXPENSE,
        ENDPOINT_GROUP,
    } from "../js/endpoints";

    export let params: any = {};
    async function getGroup(group_id: Number): Promise<GroupResponse> {
        let response = await fetch(ENDPOINT_GROUP + group_id + "/");
        let group: GroupResponse = await response.json();
        return group;
    }
    const group = getGroup(params.id);

    async function getExpense(expense_id): Promise<Expense> {
        let res = await fetch(ENDPOINT_GET_EXPENSE);
        let exp = res.json();
        return exp;
    }
    let amount: Promise<Number>,
        description: Promise<String>,
        who: Promise<Number>;
    $: amount = getExpense(params.id).then((e) => e.amount);
    $: description = getExpense(params.id).then((e) => e.name);

    async function handleAdd() {}
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
