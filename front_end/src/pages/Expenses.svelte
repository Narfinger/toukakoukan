<script lang="ts">
    import { push } from "svelte-spa-router";
    import { type Group, type GroupResponse } from "../js/types";
    import { ENDPOINT_GROUP, ENDPOINT_GROUPS } from "../js/endpoints";
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

<div class="grid lg:grid-cols-4 md:grid-cols-2 p-4">
    <p class="text-2xl lg:text-6xl pb-4 lg:col-span-4">Main Expense Overview</p>

    <div role="tablist" class="tabs tabs-bordered lg:col-span-4 pb-8">
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
    <div class="pb-2 lg:col-span-4">
        {#await group_id then gid}
            <button
                class="btn btn-primary w-64"
                on:click={async () => {
                    push("/AddExpense/" + gid);
                }}>Add</button
            >
        {/await}
    </div>
    <div>
        <ExpensesInfoWidget
            {group_id}
            group={groups.then((g) => g[active_tab])}
        />
    </div>
    <div class="lg:col-start-2 lg:col-span-2">
        <ExpensesWidget {group_id} />
    </div>
</div>
