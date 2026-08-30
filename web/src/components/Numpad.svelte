<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let size: number;
    export let disabled: boolean = false;

    const dispatch = createEventDispatcher();
</script>

<div class="numpad" class:disabled>
    {#each Array.from({ length: size }, (_, i) => i + 1) as num}
        <button on:click={() => dispatch("input", num)}>{num}</button>
    {/each}
    <button on:click={() => dispatch("input", 0)}>X</button>
</div>

<style>
    .numpad {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        gap: 10px;
        margin: 2.5rem auto 1rem auto;
        transition:
            opacity 0.2s ease,
            filter 0.2 ease;
    }

    .numpad.disabled {
        opacity: 0.4;
        filter: grayscale(100%);
        pointer-events: none;
    }

    .numpad button {
        width: 50px;
        height: 50px;
        font-size: 1.5rem;
        font-weight: bold;
        background: var(--code-bg);
        color: var(--text);
        border: 2px solid var(--border);
        border-radius: 8px;
        cursor: pointer;
        user-select: none;
    }

    .numpad button:hover {
        opacity: 0.8;
    }
</style>
