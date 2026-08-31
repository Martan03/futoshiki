<script lang="ts">
    import type { WasmBoard } from "futoshiki-wasm";
    import { createEventDispatcher } from "svelte";
    import Arrow from "./Arrow.svelte";

    export let board: WasmBoard;
    export let size: number;
    export let hasWon: boolean;
    export let selected: { x: number; y: number } | null;
    export let fixedCells: Set<string>;

    const dispatch = createEventDispatcher();

    $: gridTemplate = Array.from({ length: size * 2 - 1 }, (_, i) =>
        i % 2 === 0 ? "50px" : "30px",
    ).join(" ");

    function getHorRotation(cond: boolean | undefined): number | null {
        if (cond === true) return 0;
        if (cond === false) return 180;
        return null;
    }

    function getVerRotation(cond: boolean | undefined): number | null {
        if (cond === true) return 90;
        if (cond === false) return -90;
        return null;
    }
</script>

<div class="board-container">
    <div
        class="board"
        style="grid-template-columns: {gridTemplate}; grid-template-rows: {gridTemplate};"
    >
        {#each { length: size * 2 - 1 } as _, row}
            {#each { length: size * 2 - 1 } as _, col}
                <!-- Number cell -->
                {#if row % 2 === 0 && col % 2 === 0}
                    {@const x = col / 2}
                    {@const y = row / 2}
                    <button
                        class="cell"
                        aria-label="Row {y + 1}, Column {x +
                            1}. {board.get_value(x, y)
                            ? `Value is ${board.get_value(x, y)}`
                            : 'Empty'}"
                        class:fixed={fixedCells.has(`${x},${y}`)}
                        class:selected={selected?.x === x && selected?.y === y}
                        on:click={() => dispatch("select", { x, y })}
                    >
                        {board.get_value(x, y) || ""}
                    </button>
                    <!-- Horizontal condition -->
                {:else if row % 2 === 0 && col % 2 !== 0}
                    {@const rot = getHorRotation(
                        board.get_hor_cond(Math.floor(col / 2), row / 2),
                    )}
                    <div class="cond hor">
                        {#if rot !== null}
                            <Arrow rotation={rot} />
                        {/if}
                    </div>
                    <!-- Vertical condition -->
                {:else if row % 2 !== 0 && col % 2 === 0}
                    {@const rot = getVerRotation(
                        board.get_ver_cond(col / 2, Math.floor(row / 2)),
                    )}
                    <div class="cond ver">
                        {#if rot !== null}
                            <Arrow rotation={rot} />
                        {/if}
                    </div>
                    <!-- Empty part of the grid -->
                {:else}
                    <div class="empty"></div>
                {/if}
            {/each}
        {/each}
    </div>

    {#if hasWon}
        <div class="win-overlay">
            <h2>Puzzle Solved!</h2>
            <button on:click={() => dispatch("playAgain")}>Play Again</button>
        </div>
    {/if}
</div>

<style>
    .board-container {
        position: relative;
        display: inline-block;
        margin-top: 2rem;
    }

    .board {
        display: grid;
        justify-content: center;
    }

    .cell {
        width: 100%;
        height: 100%;
        background: var(--code-bg);
        border: 2px solid var(--border);
        border-radius: 8px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 1.3rem;
        font-weight: bold;
        color: var(--text-h);
        cursor: pointer;
        user-select: none;
    }

    .cell.fixed {
        color: var(--text);
        background: var(--bg);
        cursor: not-allowed;
    }

    .cell.selected {
        border-color: var(--accent-border);
        background: var(--accent-bg);
    }

    .cond {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 1.2rem;
        font-weight: bold;
    }

    .empty {
        width: 100%;
        height: 100%;
    }

    .win-overlay {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.6);
        backdrop-filter: blur(4px);
        border-radius: 8px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        z-index: 10;
        animation: fadeIn 0.4s ease-out forwards;
    }

    .win-overlay button {
        background: var(--accent-bg);
        color: var(--text-h);
        border: 2px solid var(--accent-border);
        padding: 0.65rem 1.5rem;
        font-size: 1rem;
        font-weight: bold;
        border-radius: 8px;
        cursor: pointer;
        opacity: 1;
        transition: opacity 0.2s;
    }

    .win-overlay button:hover {
        opacity: 0.8;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }
</style>
