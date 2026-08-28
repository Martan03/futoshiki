<script lang="ts">
    export let rotation: number;
</script>

<svg viewBox="0 0 24 24" style="transform: rotate({rotation}deg);">
    <polyline points="8 6 16 12 8 18" />
</svg>

<style>
    svg {
        width: 24px;
        height: 24px;
        fill: none;
        stroke: var(--text);
        stroke-width: 3px;
        stroke-linecap: round;
        stroke-linejoin: round;
        transition: transform 0.2s ease;
    }
</style>
