<script lang="ts">
    import {assert} from "$lib/util"

    const queue: string[] = $state(["1540", "3636"])
    let client_team : string = $state("")

    function join_queue() {
        queue.push(client_team)
    }

    function remove_from_queue(i: number) {
        assert(i >= 0 && queue.length > i, `Expected usize within bounds, found: ${i} for length ${queue.length}`)

        queue.splice(i)
    }

    const in_queue = $derived(queue.includes(client_team))
</script>

<h1>BAQ: Bunnybots Automated Queue</h1>
{#if queue.length == 0}
    <div>Queue Empty</div>
{/if}
{#each queue as team, i}
    <div class="flex flex-row gap-2">
        <div class="bg-gunmetal p-2 rounded">{team}</div>
        {#if team === client_team}
            <button class="outline p-2" onclick={() => remove_from_queue(i)}>X</button>
        {/if}
    </div>
{/each}

<input type="text" placeholder="Your Team" bind:value={client_team}>
{#if !in_queue}
    <button onclick={join_queue} class="outline p-2">Join Queue</button>
{/if}
