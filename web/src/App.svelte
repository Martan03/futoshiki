<script lang="ts">
    import { onMount } from "svelte";
    import init, { WasmBoard } from "futoshiki-wasm";
    import Arrow from "./components/Arrow.svelte";

    let board: WasmBoard | null = null;
    let size = 0;

    $: gridTemplate = Array.from({ length: size * 2 - 1 }, (_, i) =>
        i % 2 === 0 ? "50px" : "30px",
    ).join(" ");

    console.log(gridTemplate);

    onMount(async () => {
        await init();
        let b = WasmBoard.generate(4);
        size = b.size();
        board = b;
    });

    function getHorRotation(cond: boolean | undefined): number | null {
        if (cond === true) return 0; // Points Right: >
        if (cond === false) return 180; // Points Left: <
        return null;
    }

    function getVerRotation(cond: boolean | undefined): number | null {
        if (cond === true) return 90; // Points Down: v
        if (cond === false) return -90; // Points Up: ^
        return null;
    }
</script>

<main>
    <h1>Futoshiki</h1>

    {#if board}
        <div
            class="board"
            style="grid-template-columns: {gridTemplate}; grid-template-rows: {gridTemplate};"
        >
            {#each { length: size * 2 - 1 } as _, row}
                {#each { length: size * 2 - 1 } as _, col}
                    {#if row % 2 === 0 && col % 2 === 0}
                        <div class="cell">
                            {board.get_value(col / 2, row / 2) || ""}
                        </div>
                    {:else if row % 2 === 0 && col % 2 !== 0}
                        {@const rot = getHorRotation(
                            board.get_hor_cond(Math.floor(col / 2), row / 2),
                        )}
                        <div class="cond hor">
                            {#if rot !== null}
                                <Arrow rotation={rot} />
                            {/if}
                        </div>
                    {:else if row % 2 !== 0 && col % 2 === 0}
                        {@const rot = getVerRotation(
                            board.get_ver_cond(col / 2, Math.floor(row / 2)),
                        )}
                        <div class="cond ver">
                            {#if rot !== null}
                                <Arrow rotation={rot} />
                            {/if}
                        </div>
                    {:else}
                        <div class="empty"></div>
                    {/if}
                {/each}
            {/each}
        </div>
    {:else}
        <p>Loading solver engine...</p>
    {/if}
</main>

<style>
    .board {
        display: grid;
        justify-content: center;
        margin-top: 2rem;
    }

    .cell {
        width: 100%;
        height: 100%;
        background: #2a2a2a;
        border: 2px solid #555;
        border-radius: 8px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 1.3rem;
        font-weight: bold;
        color: white;
    }

    .cond {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 1.2rem;
        font-weight: bold;
        color: #aaa;
    }

    .empty {
        width: 100%;
        height: 100%;
    }
</style>
