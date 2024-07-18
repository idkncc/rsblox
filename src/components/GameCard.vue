<script setup lang="ts">
import { useRouter } from "vue-router";

import { Recommendation, TreatmentType } from "../utils/typings.ts";

const router = useRouter()

const { game, thumbnail } = defineProps<{
    game: Recommendation
    thumbnail?: string
    treatmentType: TreatmentType
}>()

</script>

<template>
    <div class="game-card w-full" @click="router.push(`/games/${game.universe_id}`)" :data-treatment-type="treatmentType">
        <img class="game-image rounded-md" :src="thumbnail ?? 'https://placehold.co/512'" alt="Game" :height="120">
        <p class="game-title">{{ game.name }}</p>
        <p class="game-playing">Playing {{ game.player_count }}</p>
    </div>
</template>

<style scoped lang="scss">
.game-card {
    @apply min-w-[120px];
    @apply cursor-pointer;

    .game-image {
        @apply min-w-[120px] min-h-[120px];
    }

    .game-title {
        @apply max-h-[52px] text-sm truncate;
    }

    .game-playing {
        @apply text-[#C0C0C0] text-xs;
    }
}

.game-card[data-treatment-type="SortlessGrid"] {
    .game-image {
        width: 100%;
        aspect-ratio: 16 / 9;
    }
}
</style>
