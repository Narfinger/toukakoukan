<script lang="ts">
    import { push } from "svelte-spa-router";
    import { ENDPOINT_EXPENSES } from "../js/endpoints";
    import type { Expense, ExpenseAdjusted } from "../js/types";
    import { adjusted_expense } from "../js/utils";
    import { formatDistance } from "date-fns";

    export let group_id: Promise<number>;
    export let user_id: number;
    export let number_of_group_members: Promise<number>;

    async function getExpenses(
        group_id: number,
    ): Promise<Array<ExpenseAdjusted>> {
        let response = await fetch(ENDPOINT_EXPENSES + group_id + "/");
        let expenses: Array<Expense> = await response.json();
        const uig = user_id;
        const nogm = await number_of_group_members;

        expenses.map((val) => {
            let expense: Expense = val;
            (val as ExpenseAdjusted).amount_adjusted = adjusted_expense(
                uig,
                nogm,
                val,
            );
            expense;
        });
        return expenses as Array<ExpenseAdjusted>;
    }
    $: expense = group_id.then((gs) => {
        return getExpenses(gs);
    });
</script>

{#await expense}
    <span class="loading loading-dots loading-lg"></span>
{:then expenses}
    <table
        class="table-auto border table-zebra p-2 border-separate border-spacing-y-2"
    >
        <thead>
            <th class="p-2 w-52">Name</th>
            <th class="p-2 w-52">Amount Owed</th>
            <th class="p-2 w-52">Date</th>
        </thead>
        <tbody>
            {#each expenses as exp}
                <tr class="group">
                    <td
                        >{exp["name"]}
                        <button
                            class="hidden group-hover:center-right group-hover:btn group-hover:btn-sm"
                            on:click={() => push("/edit/" + exp["id"])}
                            >Edit</button
                        ></td
                    >
                    <td>{exp["amount_adjusted"]}</td>
                    <td>{formatDistance(new Date(exp["time"]), Date.now())}</td>
                </tr>
            {/each}
        </tbody>
    </table>
{/await}
