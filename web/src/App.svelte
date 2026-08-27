<script lang="ts">
    import { onMount } from "svelte";
    import init, { WasmBoard } from "futoshiki-wasm";

    let board: WasmBoard | null = null;
    let boardSize = 4;

    onMount(async () => {
        await init();

        board = WasmBoard.generate(boardSize);
    });
</script>

<main>
    <h1>Futoshiki</h1>

    {#if board}
        <p>Board loaded! Size: {board.size()}</p>
    {:else}
        <p>Loading solver engine...</p>
    {/if}
</main>
