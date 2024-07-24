<script lang="ts">
    import type { Recommendation, TreatmentType } from "$lib/typings.ts";

    export let game: Recommendation;
    export let thumbnail: string;
    export let treatmentType: TreatmentType;
</script>

<a href="/game?id={game.universe_id}">
    <div class="game-card w-full" data-treatment-type={treatmentType}>
        <img
            class="game-image rounded-md"
            src={thumbnail}
            alt="Game"
            height="120"
        />

        <p class="game-title">{game.name}</p>
        <p class="game-playing">Playing {game.player_count}</p>
    </div>
</a>

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
