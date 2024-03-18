<script lang="ts">
    import type { Expense, Group, GroupResponse, PayedType } from "../js/types";
    import { push } from "svelte-spa-router";
    import { ENDPOINT_EXPENSES, ENDPOINT_GET_EXPENSE } from "../js/endpoints";
    import { getGroup } from "../js/api";
    import { onMount } from "svelte";
    import { EvenSplit, payed_type_list } from "../js/utils";

    export let params: any = {};
    async function getExpense(expense_id): Promise<Expense> {
        let res = await fetch(ENDPOINT_GET_EXPENSE + expense_id + "/");
        let exp = res.json();
        return exp;
    }
    let expense: Expense;
    let amount: number = 0;
    let description: string = "";
    let who: number;
    let expense_types = [];
    let group: GroupResponse = { name: "", users: [], querying_user_is: 0 };
    async function setDetailsOnLoad() {
        expense = await getExpense(params.id);
        amount = expense.amount;
        description = expense.name;
        group = await getGroup(expense.expense_group_id);
        expense_types = payed_type_list(group.users);
        who = expense_types.findIndex(
            (val) =>
                val[1].c == expense.payed_type.c &&
                val[1].t == expense.payed_type.t,
        );
    }

    async function handleAdd() {
        let e = await expense;
        e.amount = amount;
        e.name = description;
        e.payed_type = expense_types[who][1];
        console.log(e);
        await fetch(ENDPOINT_EXPENSES, {
            method: "PUT",
            body: JSON.stringify(e),
            headers: {
                "content-type": "application/json",
            },
        });
        push("/expenses");
    }
    onMount(() => {
        setDetailsOnLoad();
    });
</script>

<div class="flex flex-col p-8 justify-center">
    <p class="text-2xl lg:text-6xl pb-4">Add an expense</p>
    <form>
        <div class="flex flex-col">
            <div class="p-2">
                <input
                    id="amount-field"
                    class="input input-primary input-bordered"
                    type="number"
                    placeholder="amount"
                    bind:value={amount}
                />
            </div>
            <div class="p-2">
                <input
                    class="input input-secondary input-bordered"
                    placeholder="description"
                    bind:value={description}
                />
            </div>
            {#await group}
                <span class="loading loading-dots loading-lg"></span>
            {:then g}
                <div class="p-2">
                    <select bind:value={who}>
                        {#await expense_types}
                            <span class="loading loading-dots loading-lg"
                            ></span>
                        {:then exp_type}
                            {#each exp_type as e}
                                <option
                                    class="select select-primary"
                                    value={e[0]}
                                >
                                    {g.users[e[1].c].name} payed
                                    {#if e[1].t == EvenSplit}
                                        Split 50/50
                                    {:else}
                                        owed in full
                                    {/if}</option
                                >
                            {/each}
                        {/await}
                    </select>
                </div>
            {/await}
        </div>
    </form>
    <div class="flex flex-col">
        <div class="p-2">
            <button class="btn btn-primary w-full" on:click={handleAdd}
                >Modify</button
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
