<script lang="ts">
    import { push } from "svelte-spa-router";
    import { ENDPOINT_EXPENSES, ENDPOINT_TOTAL } from "../js/endpoints";
    import type { Expense, ExpenseAdjusted, Group } from "../js/types";
    import { adjusted_expense } from "../js/utils";

    export let group_id: Promise<Number>;
    export let group: Promise<Group>;

    async function getTotal(group_id: Number): Promise<Number> {
        let response = await fetch(ENDPOINT_TOTAL + group_id + "/");
        let total = await response.json();
        return total;
    }
    $: total = group_id.then((gs) => {
        return getTotal(gs);
    });
</script>

<div class="row-span-4 pb-2">
    {#await group_id then gid}
        <button
            class="btn btn-primary w-64"
            on:click={async () => {
                push("/AddExpense/" + gid);
            }}>Add</button
        >
    {/await}
    {#await group then g}
        <div class="flex">
            <h2 class="w-14 h-14">Group:</h2>
            {#each g.users as u}
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
