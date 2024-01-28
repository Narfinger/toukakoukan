<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { push, pop, replace } from "svelte-spa-router";
    import {
        createPayed,
        type Expense,
        type Group,
        type GroupResponse,
    } from "../js/types";
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

    async function getExpenses(group_id: Number): Promise<Array<Expense>> {
        let response = await fetch(ENDPOINT_EXPENSES + group_id + "/");
        let expenses: Array<Expense> = await response.json();

        console.log("change time towards objects here");
        return expenses;
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

<div class="flex flex-col p-8 justify-center">
    <h1 class="underline">Main Expense Overview</h1>

    <div role="tablist" class="tabs tabs-bordered">
        {#await groups then groups}
            {#each groups as g, index}
                <a
                    role="tab"
                    class="tab tab-active"
                    tabindex={index}
                    class:tab-active={active_tab == index}
                    on:click={() => (active_tab = index)}>{g.name}</a
                >
            {/each}
            <a
                role="tab"
                class="tab tab-active"
                tabindex={groups.length}
                class:tab-active={active_tab == groups.length}
                on:click={() => {
                    push("/addgroup/");
                }}>Add Group</a
            >
        {/await}
    </div>
    <div>
        <button
            class="btn btn-primary"
            on:click={async () => {
                const gid = await group_id;
                push("/AddExpense/" + gid);
            }}>Add</button
        >
    </div>
    <div>
        {#await groups then g}
            <div class="flex">
                <h2 class="w-14 h-14">Group:</h2>
                {#each g[active_tab].users as u}
                    <div class="shrink w-14 h-14">{u}</div>
                {/each}
            </div>
        {/await}
    </div>
    <div>
        {#await total then t}
            <div class="flex">
                <h2 class="w-14 h-14">Total:</h2>
                <div class="shrink w-14 h-14">{t}</div>
            </div>
        {/await}
    </div>
    <div>
        {#await expense}
            <p>Loading expenses</p>
        {:then expenses}
            <table class="table-fixed border table-zebra">
                <thead>
                    <th>Name</th>
                    <th>Who Payed</th>
                    <th>Amount</th>
                </thead>
                <tbody>
                    {#each expenses as exp}
                        <tr>
                            <td>{exp["name"]}</td>
                            <td
                                >{#if exp["payed"] == 0}
                                    "You are owedTHISISWRONG"
                                {:else}
                                    "You oweTHISISWRONG"
                                {/if}
                            </td>
                            <td>{exp["amount"]}</td>
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
