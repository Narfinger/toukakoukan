<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { push, pop, replace } from "svelte-spa-router";
    import {
        type Expense,
        type ExpenseAdjusted,
        type Group,
        type GroupResponse,
    } from "../js/types";
    import { adjusted_expense, createPayed } from "../js/utils";
    import {
        ENDPOINT_EXPENSES,
        ENDPOINT_GROUP,
        ENDPOINT_GROUPS,
        ENDPOINT_TOTAL,
    } from "../js/endpoints";
    async function getGroups(): Promise<Array<Group>> {
        let response = await fetch(ENDPOINT_GROUPS);
        let groups = await response.json();
        return groups;
    }

    async function getGroup(group_id: Number): Promise<GroupResponse> {
        let response = await fetch(ENDPOINT_GROUP + group_id + "/");
        let group: GroupResponse = await response.json();
        return group;
    }

    async function getExpenses(
        group_id: Number,
    ): Promise<Array<ExpenseAdjusted>> {
        let response = await fetch(ENDPOINT_EXPENSES + group_id + "/");
        let expenses: Array<Expense> = await response.json();

        expenses.map((val) => {
            let expense: Expense = val;
            (val as ExpenseAdjusted).amount_adjusted = adjusted_expense(val);
            expense;
        });
        return expenses as Array<ExpenseAdjusted>;
    }
    async function getTotal(group_id: Number): Promise<Number> {
        let response = await fetch(ENDPOINT_TOTAL + group_id + "/");
        let total = await response.json();
        return total;
    }

    const groups = getGroups().then((groups) => groups);
    let active_tab = 0;
    $: expense = groups.then((gs) => {
        return getExpenses(gs[active_tab].id);
    });
    $: group = groups.then((gs) => {
        return getExpenses(gs[active_tab].id);
    });
    $: total = groups.then((gs) => {
        return getTotal(gs[active_tab].id);
    });
    $: group_id = groups.then((g) => g[active_tab].id);
</script>

<div class="grid grid-cols-4 p-4">
    <p class="text-6xl pb-8 col-span-4">Main Expense Overview</p>

    <div role="tablist" class="tabs tabs-bordered col-span-4 pb-8">
        {#await groups then groups}
            {#each groups as g, index}
                <button
                    role="tab"
                    class="tab tab-active"
                    tabindex={index}
                    class:tab-active={active_tab == index}
                    on:click={() => (active_tab = index)}>{g.name}</button
                >
            {/each}
            <button
                role="tab"
                class="tab tab-active"
                tabindex={groups.length}
                class:tab-active={active_tab == groups.length}
                on:click={() => {
                    push("/addgroup/");
                }}>Add Group</button
            >
        {/await}
    </div>

    <div class="row-span-4 pb-2">
        {#await group_id then gid}
            <button
                class="btn btn-primary w-64"
                on:click={async () => {
                    push("/AddExpense/" + gid);
                }}>Add</button
            >
        {/await}
        {#await groups then g}
            <div class="flex">
                <h2 class="w-14 h-14">Group:</h2>
                {#each g[active_tab].users as u}
                    <div class="shrink w-14 h-14">{u.name}</div>
                {/each}
            </div>
        {/await}
        {#await total then t}
            <div class="flex">
                <h2 class="w-14 h-14">Total:</h2>
                <div class="shrink w-14 h-14">{t}</div>
            </div>
        {/await}
    </div>
    <div class="col-start-2 col-span-3">
        {#await expense}
            <span class="loading loading-dots loading-lg"></span>
        {:then expenses}
            <table class="table-auto border table-zebra border-separate p-2">
                <thead>
                    <th>Name</th>
                    <th>Amount</th>
                    <th>Date</th>
                </thead>
                <tbody>
                    {#each expenses as exp}
                        <tr>
                            <td>{exp["name"]}</td>
                            <td>{exp["amount_adjusted"]}</td>
                            <td
                                >{new Date(exp["time"]).toLocaleTimeString()} - {new Date(
                                    exp["time"],
                                ).toDateString()}</td
                            >
                        </tr>
                    {/each}
                </tbody>
            </table>
        {/await}
    </div>
</div>
