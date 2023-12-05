<script>
    import { createEventDispatcher } from "svelte";
    import "../app.css";
    let isProduction = import.meta.env.MODE === "production";
    let group_id = 1;
    async function getGroup() {
        let response = await fetch("/api/group/" + group_id + "/");
        let group = await response.json();
        return group;
    }

    async function getExpenses() {
        let response = await fetch("/api/expense/" + group_id + "/");
        let expenses = await response.json();
        return expenses;
    }
    const expense = getExpenses();
    const group = getGroup();
</script>

<div class="container mx-auto px-4">
    <h1 class="underline">Main Expense Overview</h1>

    <button class="btn btn-primary">Add</button>

    {#await expense}
        <p>Loading expenses</p>
    {:then expenses}
        <div>
            {#await group then group}
                <h2>{group.name}</h2>
                {#each group.people as p}
                    <div>{p}</div>
                {/each}
            {/await}
        </div>
        <table class="table">
            {#each expenses as exp}
                <tr>
                    <td
                        >{#if exp["payed"] == 0}
                            "You are owed"
                        {:else}
                            "You owe"
                        {/if}
                    </td>
                    <!-- <td>{exp["amount"] / exp["people"].length}YEN</td>-->
                </tr>
            {/each}
        </table>
    {/await}
</div>
