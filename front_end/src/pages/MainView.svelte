<script lang="ts">
    import { push } from "svelte-spa-router";
    import ExpensesWidget from "../widgets/ExpensesWidget.svelte";
    import ExpensesInfoWidget from "../widgets/ExpensesInfoWidget.svelte";
    import { onMount } from "svelte";
    import { getUser, getGroups, prefetchGroups, logout } from "../js/api";

    const groups = getGroups();
    const user = getUser();
    let active_tab = 0;

    $: group_id = groups.then((g) => {
        if (g.length == 0) {
            return -1;
        } else {
            return g[active_tab].id;
        }
    });
    $: number_of_group_members = groups.then((g) => {
        if (g.length == 0) {
            return 0;
        } else {
            return g[active_tab].users.length;
        }
    });

    groups.then((g) => prefetchGroups(g));
    onMount(() => {
        getUser();
    });
</script>

<div class="grid lg:grid-cols-4 md:grid-cols-2 gap-4 p-4">
    <p class="lg:col-span-3 text-2xl pb-4">
        Main Expense Overview for
        {#await user then user}
            {user.name}
        {/await}
    </p>
    <div>
        <button class="btn btn-neutral w-64" on:click={() => logout()}
            >Logout</button
        >
    </div>
    <div role="tablist" class="tabs tabs-bordered lg:col-span-4 pb-6">
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
    {#await groups then g}
        <div>
            {#if g.length != 0}
                <ExpensesInfoWidget
                    {group_id}
                    group={groups.then((g) => g[active_tab])}
                />
            {/if}
        </div>
        <div class="lg:col-start-2 lg:col-span-2">
            {#if g.length != 0}
                {#await user then user}
                    <ExpensesWidget
                        {group_id}
                        {number_of_group_members}
                        user_id={user.id}
                    />
                {/await}
            {/if}
        </div>
    {/await}
</div>
