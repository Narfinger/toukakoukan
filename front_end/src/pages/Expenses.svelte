<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { push, pop, replace } from "svelte-spa-router";
    import {
        createPayed,
        type Expense,
        type Group,
        type GroupResponse,
    } from "../js/types";
    let isProduction = import.meta.env.MODE === "production";
    async function getGroups(): Promise<Array<Group>> {
        if (!isProduction) {
            return Promise.resolve([
                { id: 1, name: "Testgroup1", users: ["N1", "N2"] },
                { id: 2, name: "Testgroup2", users: ["N1", "N2"] },
            ]);
        }
        let response = await fetch("/api/groups/");
        let groups = await response.json();
        return groups;
    }

    async function getGroup(group_id: Number): Promise<GroupResponse> {
        if (!isProduction) {
            return Promise.resolve({
                name: "Test1",
                people: ["PTest1", "PTest2"],
            });
        }
        let response = await fetch("/api/group/" + group_id + "/");
        let group: GroupResponse = await response.json();
        return group;
    }

    async function getExpenses(group_id: Number): Promise<Array<Expense>> {
        if (!isProduction) {
            return Promise.resolve([
                {
                    id: 1,
                    payed_type: createPayed("EvenSplit", 0),
                    amount: 250,
                    name: "Test1",
                    time: new Date("2023-01-01 10:01"),
                    expense_group_id: 1,
                },
                {
                    id: 2,
                    payed_type: createPayed("EvenSplit", 1),
                    amount: 250,
                    time: new Date("2023-01-02 10:00"),
                    name: "Test2",
                    expense_group_id: 1,
                },
            ]);
        }
        let response = await fetch("/api/expense/" + group_id + "/");
        let expenses: Array<Expense> = await response.json();

        console.log("change time towards objects here");
        return expenses;
    }
    async function getTotal(group_id: Number): Promise<Number> {
        if (!isProduction) {
            return Promise.resolve(500);
        }
        let response = await fetch("/api/total/" + group_id + "/");
        let total = await response.json();
    }

    const groups = getGroups().then((groups) => groups);
    let active_tab = 0;
    $: expense = groups.then((groups) => {
        return getExpenses(groups[active_tab].id);
    });
    $: group = groups.then((groups) => {
        return getExpenses(groups[active_tab].id);
    });
    $: total = groups.then((groups) => {
        return getTotal(groups[active_tab].id);
    });
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
        {/await}
    </div>
    <div>
        <button
            class="btn btn-primary"
            on:click={() => push("/AddExpense/" + groups[active_tab].id)}
            >Add</button
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
                                    "You are owed"
                                {:else}
                                    "You owe"
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
