<script lang="ts">
    import { EvenSplit, payed_type_list } from "../js/utils";
    import { push } from "svelte-spa-router";
    import { ENDPOINT_EXPENSES } from "../js/endpoints";
    import { getGroup } from "../js/api";
    import type { PayedType, PayedTypeSelect } from "../js/types";

    async function payed_type_select_to_payedtype(
        p: Promise<PayedTypeSelect>,
        index: number,
    ): Promise<PayedType> {
        const pts = await p;
        const cur = pts[index];
        console.log(cur);
        return cur[1];
    }

    type AddExpenseParam = {
        id: number;
    };

    export let params: AddExpenseParam;
    let amount: number,
        description: string,
        who: number = 1;

    const group = getGroup(params.id);

    async function handleAdd() {
        const payed_type = await payed_type_select_to_payedtype(
            expense_types,
            who,
        );
        const d = new Date(Date.now());
        let data = {
            id: -1,
            payed_type: payed_type,
            amount: amount,
            name: description,
            time: d.toISOString(),
            expense_group_id: Number(params.id),
        };
        console.log("WE ARE DOING SOMETHING WRONG HERE");
        await fetch(ENDPOINT_EXPENSES + params.id + "/", {
            method: "POST",
            body: JSON.stringify(data),
            headers: {
                "content-type": "application/json",
            },
            credentials: "include",
        });
        push("/expenses");
    }
    const expense_types: Promise<PayedTypeSelect> = group.then((g) =>
        payed_type_list(g.users),
    );
</script>

<div class="flex flex-col p-8 justify-center">
    <p class="text-2xl lg:text-6xl pb-4">Add an expense</p>
    <form on:submit|preventDefault={handleAdd}>
        <div class="flex flex-col">
            <div class="p-2">
                <input
                    id="amount-field"
                    class="input input-primary input-bordered"
                    type="number"
                    placeholder="amount"
                    bind:value={amount}
                    required
                />
            </div>
            <div class="p-2">
                <input
                    class="input input-secondary input-bordered"
                    placeholder="description"
                    bind:value={description}
                    required
                />
            </div>
            {#await group}
                <span class="loading loading-dots loading-lg"></span>
            {:then g}
                <div class="p-2">
                    {#await expense_types}
                    <span class="loading loading-dots loading-lg"></span>
                    {:then exp_type}
                    <select class="select select-primary" bind:value={who}>
                            {#each exp_type as e}
                                <option value={e[0]}>
                                    {g.users[e[1].c].name} payed
                                    {#if e[1].t == EvenSplit}
                                        Split 50/50
                                    {:else}
                                        owed in full
                                    {/if}</option
                                >
                            {/each}
                        </select>
                        {/await}
                </div>
            {/await}
        </div>
        <div class="flex flex-col">
            <div class="p-2">
                <button class="btn btn-primary w-full" type="submit">Add</button
                >
            </div>
            <div class="p-2 grow">
                <button
                    class="btn btn-warning w-full"
                    on:click={() => {
                        push("/expenses");
                    }}>Cancel</button
                >
            </div>
        </div>
    </form>
</div>
