<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { createPayed } from "../js/types";
    import type { Expense, Group, GroupResponse } from "../js/types";
    import { push } from "svelte-spa-router";
    import { ENDPOINT_EXPENSES, ENDPOINT_GROUP } from "../js/endpoints";

    export const params: any = {};

    async function handleAdd() {}
</script>

<div class="flex flex-col p-8 justify-center">
    <h1 class="underline">Add an expense</h1>
    <form class="shadow-md rounded px-8 pt-6 pb-8 mb-4">
        <div class="flex flex-col p8">FIX THIS TO ADD A GROUP!!!</div>
    </form>
    <button on:click={handleAdd}>Add</button>
</div>
