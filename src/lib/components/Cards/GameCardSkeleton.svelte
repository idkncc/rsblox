<script lang="ts">
    import type { Recommendation, TreatmentType } from "$lib/typings.ts";
</script>

<div class="game-card w-full" data-treatment-type={"Carousel"}>
    <div class="game-image rounded-md placeholder" />

    <p class="game-title"><span class="placeholder w-20" /></p>
    <p class="game-playing"><span class="placeholder w-12" /></p>
</div>

<style lang="scss">
    .game-card {
        @apply min-w-[120px];
        @apply cursor-pointer;

        .game-title {
            @apply max-w-[120px] max-h-[52px] text-sm truncate;
        }

        .game-playing {
            @apply text-[#C0C0C0] text-xs;
        }
    }

    .game-card[data-treatment-type="Carousel"] {
        .game-image {
            @apply w-[120px] h-[120px];
        }
    }

    .game-card[data-treatment-type="SortlessGrid"] {
        .game-image {
            @apply min-w-[120px] min-h-[120px];

            width: 100%;
            aspect-ratio: 16 / 9;
        }
    }
</style>
