<script lang="ts">
    import type { Expense, GroupResponse, PayedTypeSelect } from "../js/types";
    import { push } from "svelte-spa-router";
    import { ENDPOINT_EXPENSES, ENDPOINT_GET_EXPENSE } from "../js/endpoints";
    import { getGroup } from "../js/api";
    import { onMount } from "svelte";
    import { EvenSplit, payed_type_list } from "../js/utils";

    type EditExpenseParam = {
        id: number;
    };

    export let params: EditExpenseParam;
    async function getExpense(expense_id: number): Promise<Expense> {
        let res = await fetch(ENDPOINT_GET_EXPENSE + expense_id + "/", {
            credentials: "include",
        });
        let exp = res.json();
        return exp;
    }
    let expense: Expense;
    let amount: number = 0;
    let description: string = "";
    let who: number;
    let expense_types: PayedTypeSelect = [];
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

    async function handleEdit() {
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
            credentials: "include",
        });
        push("/expenses");
    }
    onMount(() => {
        setDetailsOnLoad();
    });
</script>

<div class="flex flex-col p-8 justify-center">
    <p class="text-2xl lg:text-6xl pb-4">Add an expense</p>
    <form on:submit|preventDefault={handleEdit}>
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
                    {#await expense_types}
                    <span class="loading loading-dots loading-lg"
                    ></span>
                    {:then exp_type}
                    <select bind:value={who}>
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
                        </select>
                        {/await}
                </div>
            {/await}
        </div>
    </form>
    <div class="flex flex-col">
        <div class="p-2">
            <button class="btn btn-primary w-full">Modify</button>
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
