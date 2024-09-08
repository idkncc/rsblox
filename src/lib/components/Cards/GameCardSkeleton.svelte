<script lang="ts">
    import type { Recommendation, TreatmentType } from "$lib/typings.ts";
    import Skeleton from "../ui/skeleton/skeleton.svelte";
</script>

<div class="game-card">
    <Skeleton class="game-image rounded-md w-[120px] h-[120px]" />

    <p class="game-title"><Skeleton class=" w-20 h-3 rounded-full" /></p>
    <p class="game-playing"><Skeleton class=" w-10 h-2 rounded-full" /></p>
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
