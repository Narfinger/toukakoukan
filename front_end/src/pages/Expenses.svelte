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
    import Expenses from "../widgets/ExpensesWidget.svelte";
    import ExpensesWidget from "../widgets/ExpensesWidget.svelte";
    import ExpensesInfoWidget from "../widgets/ExpensesInfoWidget.svelte";
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

    const groups = getGroups().then((groups) => groups);
    let active_tab = 0;

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
    <ExpensesInfoWidget {group_id} group={groups.then((g) => g[active_tab])} />
    <div class="col-start-2 col-span-3">
        <ExpensesWidget {group_id} />
    </div>
</div>
