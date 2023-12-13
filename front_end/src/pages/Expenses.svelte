<script>
    import { createEventDispatcher } from "svelte";
    import { route_addexpense } from "../js/consts";

    let isProduction = import.meta.env.MODE === "production";
    let group_id = 1;
    async function getGroup() {
        if (!isProduction) {
            return Promise.resolve({
                name: "Test1",
                people: ["Test1", "Test2"],
            });
        }
        let response = await fetch("/api/group/" + group_id + "/");
        let group = await response.json();
        return group;
    }

    async function getExpenses() {
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
    const expense = getExpenses();
    const group = getGroup();
</script>

<div class="flex flex-col p-8 justify-center">
    <h1 class="underline">Main Expense Overview</h1>
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
