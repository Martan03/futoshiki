<script lang="ts">
    import init, { check_win, WasmBoard } from "futoshiki-wasm";
    import { onMount } from "svelte";
    import Controls from "./components/Controls.svelte";
    import BoardGrid from "./components/BoardGrid.svelte";
    import Numpad from "./components/Numpad.svelte";
    import Footer from "./components/Footer.svelte";
    import Rules from "./components/Rules.svelte";

    let board: WasmBoard | null = null;
    let hasWon = false;
    let isRevealed = false;

    let size = 0;
    let inputSize = 4;
    let debounceTimer: ReturnType<typeof setTimeout>;

    let selected: { x: number; y: number } | null = null;
    let fixedCells: Set<string> = new Set();

    $: if (inputSize > 1 && inputSize <= 25) {
        clearTimeout(debounceTimer);
        debounceTimer = setTimeout(() => {
            if (inputSize != size) newGame(inputSize);
        }, 500);
    }

    async function newGame(newSize: number) {
        hasWon = false;
        isRevealed = false;
        selected = null;

        let b = WasmBoard.generate(newSize);
        size = b.size();

        fixedCells.clear();
        for (let y = 0; y < size; y++) {
            for (let x = 0; x < size; x++) {
                if (b.get_value(x, y) !== 0) {
                    fixedCells.add(`${x},${y}`);
                }
            }
        }

        fixedCells = fixedCells;
        board = b;
    }

    function solveBoard() {
        if (!board || hasWon || isRevealed) return;

        for (let y = 0; y < size; y++) {
            for (let x = 0; x < size; x++) {
                if (!fixedCells.has(`${x},${y}`)) {
                    board.set_value(x, y, 0);
                }
            }
        }

        const success = board.solve();
        if (success) {
            board = board;
            isRevealed = true;
            selected = null;
        } else {
            alert("This board cannot be solved...");
        }
    }

    onMount(async () => {
        await init();
        newGame(4);
    });

    function selectCell(e: CustomEvent<{ x: number; y: number }>) {
        const { x, y } = e.detail;
        if (!fixedCells.has(`${x},${y}`)) {
            selected = { x, y };
        }
    }

    function writeVal(val: number) {
        if (selected && board && !hasWon && !isRevealed) {
            board.set_value(selected.x, selected.y, val);
            board = board;

            if (check_win(board)) {
                hasWon = true;
                selected = null;
            }
        }
    }

    function handleKey(e: KeyboardEvent) {
        if (!selected || hasWon || isRevealed) return;

        const num = parseInt(e.key);
        if (num >= 0 && num <= size) {
            writeVal(num);
        } else if (e.key === "Backspace" || e.key === "Delete") {
            writeVal(0);
        } else if (e.key === "ArrowLeft" && selected.x > 0) {
            selected.x -= 1;
        } else if (e.key === "ArrowRight" && selected.x < size - 1) {
            selected.x += 1;
        } else if (e.key === "ArrowUp" && selected.y > 0) {
            selected.y -= 1;
        } else if (e.key === "ArrowDown" && selected.y < size - 1) {
            selected.y += 1;
        }
    }
</script>

<svelte:window on:keydown={handleKey} />
<main>
    <h1>Futoshiki</h1>

    <Controls
        bind:inputSize
        on:newGame={() => newGame(size)}
        on:solve={solveBoard}
    />

    {#if board}
        <BoardGrid
            {board}
            {size}
            {hasWon}
            {selected}
            {fixedCells}
            on:select={selectCell}
            on:playAgain={() => newGame(size)}
        />

        {#if !hasWon && !isRevealed}
            <Numpad
                {size}
                disabled={!selected}
                on:input={(e) => writeVal(e.detail)}
            />
        {/if}
    {:else}
        <p>Loading solver engine...</p>
    {/if}

    <Rules {size} />

    <Footer />
</main>

<style>
    main {
        text-align: center;
        min-height: 100vh;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>
