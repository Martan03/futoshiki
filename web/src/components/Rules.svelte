<script lang="ts">
    export let size: number;
</script>

<details class="rules">
    <summary>How to play</summary>
    <div class="rules-content">
        <p>
            Fill the board with numbers from
            <strong>1 to {size || 4}</strong>.
        </p>
        <ul>
            <li>
                Every row and column must contain exactly one of each number.
            </li>
            <li>
                Numbers must satisfy the greater-than (<strong>&gt;</strong>)
                and less-than (<strong>&lt;</strong>) signs between the cells.
            </li>
        </ul>
    </div>
</details>

<style>
    .rules {
        max-width: 400px;
        width: 100%;
        margin: 2rem auto 1.5rem auto;
        background: var(--code-bg);
        border: 1px solid var(--border);
        border-radius: 8px;
        text-align: left;
    }

    .rules summary {
        padding: 0.75rem 1rem;
        font-weight: bold;
        color: var(--text-h);
        cursor: pointer;
        user-select: none;
        list-style-position: inside;
    }

    .rules summary:hover {
        background: var(--accent-bg);
    }

    .rules-content {
        padding: 0.5rem 1rem 1rem 1rem;
        color: var(--text);
        font-size: 0.9rem;
        line-height: 1.4;
    }

    .rules-content ul {
        margin-top: 0.5rem;
        padding-left: 1.5rem;
    }
</style>
