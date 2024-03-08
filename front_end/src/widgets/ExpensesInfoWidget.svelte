<script lang="ts">
    import { ENDPOINT_TOTAL } from "../js/endpoints";
    import type { Group } from "../js/types";

    export let group_id: Promise<Number>;
    export let group: Promise<Group>;

    async function getTotal(group_id: Number): Promise<Number> {
        let response = await fetch(ENDPOINT_TOTAL + group_id + "/");
        let total = await response.json();
        return total;
    }
    $: total = group_id.then((gs) => {
        return getTotal(gs);
    });
</script>

{#await group}
    <span class="loading loading-dots loading-lg"></span>
{:then g}
    <div class="flex flex-row">
        <p class="pr-2">Group:</p>
        {#each g.users as u}
            <p class="pr-2">{u.name}</p>
        {/each}
    </div>
{/await}
{#await total}
    <span class="loading loading-dots loading-lg"></span>
{:then t}
    <div class="flex flex-row">
        <p class="pr-2">Total:</p>
        <p>{t}</p>
    </div>
{/await}
