<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { route_addexpense } from "../js/consts";

    type GroupResponse = {
        name: String;
        people: Array<String>;
    };

    type Group = {
        id: number;
        name: String;
        users: Array<String>;
    };

    let isProduction = import.meta.env.MODE === "production";
    async function getGroups(): Promise<Array<Group>> {
        if (!isProduction) {
            return Promise.resolve([
                { id: 1, name: "Testgroup1", users: [] },
                { id: 2, name: "Testgroup2", users: [] },
            ]);
        } else {
            let response = await fetch("/api/groups/");
            let groups = await response.json();
            return groups;
        }
    }
    async function getGroup(group_id): Promise<GroupResponse> {
        if (!isProduction) {
            return Promise.resolve({
                name: "Test1",
                people: ["PTest1", "PTest2"],
            });
        }
        let response = await fetch("/api/group/" + group_id + "/");
        let group = await response.json();
        return group;
    }

    async function getExpenses(group_id) {
        if (!isProduction) {
            return Promise.resolve([
                {
                    id: 1,
                    amount: 250,
                    time: "2023-01-01 10:01",
                    payed: 0,
                    name: "Test1",
                },
                {
                    id: 2,
                    amount: 250,
                    time: "2023-01-02 10:00",
                    payed: 1,
                    name: "Test2",
                },
            ]);
        }
        let response = await fetch("/api/expense/" + group_id + "/");
        let expenses = await response.json();
        return expenses;
    }
    const groups = getGroups().then((groups) => groups.entries());
    let active_tab = 0;
    $: group_id = groups.then((groups) => {
        groups[active_tab].id;
    });

    $: expense = groups.then((groups) => {
        return getExpenses(group_id);
    });
    $: group = groups.then((groups) => {
        return getExpenses(group_id);
    });
    function switchTab(index) {
        active_tab = index;
    }
</script>

<div class="flex flex-col p-8 justify-center">
    <h1 class="underline">Main Expense Overview</h1>

    <div role="tablist" class="tabs tabs-bordered">
        {#await groups then groups}
            {#each groups as g}
                {#if g.id == active_tab}
                    <a
                        role="tab"
                        class="tab tab-active"
                        tabindex={0}
                        on:click={() => {
                            switchTab(0);
                        }}>{g.name}</a
                    >
                {:else}
                    <a
                        role="tab"
                        class="tab"
                        tabindex={1}
                        on:click={() => switchTab(1)}>{g.name}</a
                    >
                {/if}
            {/each}
        {/await}
    </div>
    <div>
        <button
            class="btn btn-primary"
            on:click={() => (window.location.hash = "#addexpenses")}>Add</button
        >
    </div>
    <div>
        {#await group then group}
            <h2>Group: {group.name}</h2>
            {#each group.people as p}
                <div>{p}</div>
            {/each}
        {/await}
    </div>
    <div>
        {#await expense}
            <p>Loading expenses</p>
        {:then expenses}
            <table class="table-fixed border">
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
                            <td>{exp["time"]}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        {/await}
    </div>
</div>
