<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let inputSize: number;
    const dispatch = createEventDispatcher();
</script>

<div class="controls">
    <label for="sizeInput">Size: </label>
    <input
        id="sizeInput"
        type="number"
        min="2"
        max="25"
        bind:value={inputSize}
    />

    <div class="spacer"></div>

    <button class="action-btn" on:click={() => dispatch("newGame")}>New</button>
    <button class="action-btn" on:click={() => dispatch("solve")}>Solve</button>
</div>

<style>
    .controls {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 0.5rem;
        margin-top: 1rem;
        background: transparent;
    }

    .controls label {
        font-size: 1rem;
        color: var(--text-h);
    }

    .controls input {
        width: 60px;
        height: 35px;
        font-size: 1rem;
        font-weight: bold;
        text-align: center;
        background: var(--code-bg);
        color: var(--text-h);
        border: 2px solid var(--border);
        border-radius: 8px;
        outline: none;
        transition: border-color 0.2s ease;
    }

    .controls input:focus {
        border-color: var(--accent-border);
    }

    .controls input::-webkit-outer-spin-button,
    .controls input::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }

    .controls input[type="number"] {
        appearance: textfield;
        -moz-appearance: textfield;
    }

    .action-btn {
        height: 35px;
        padding: 0 0.8rem;
        font-size: 1rem;
        background-color: var(--code-bg);
        color: var(--text-h);
        border: 2px solid var(--border);
        border-radius: 8px;
        cursor: pointer;
        transition:
            border-color 0.2 ease,
            background-color 0.2s ease;
    }

    .action-btn:hover {
        background-color: var(--accent-bg);
        border-color: var(--accent-border);
    }
</style>
