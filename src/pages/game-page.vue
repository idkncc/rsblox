<script setup lang="ts">
import '../assets/css/splide.css';

import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { useRobloxApi } from "../utils/robloxApi";

import GameDescriptionTab from "../components/GamePage/GameDescriptionTab.vue";

import { GameDetails, GameMedia, GameMediaType } from "../utils/typings.ts";

const robloxApi = useRobloxApi();
const route = useRoute();

const currentTab = ref<number>(0);

const gameDetails = ref<GameDetails>();
const gameMedia = ref<GameMedia[]>([]);
const gameMediaUrls = ref<Record<number, string>>({});

onMounted(async () => {
    gameDetails.value = await robloxApi.getGameDetails(
        parseInt(route.params.id as string),
    );

    gameMedia.value = await robloxApi.getGameMedia(gameDetails.value.universe_id)
    const gameMediaImages = gameMedia.value
        .filter((media) => media.image_id)
        .map((media) => media.image_id) as number[]

    const _gameMediaUrls = await robloxApi.getMediaUrls(gameMediaImages)

    gameMediaUrls.value = Object.fromEntries(
        gameMediaImages.map((id, i) => [id, _gameMediaUrls[i]])
    )
});

function play() {
    if (!gameDetails.value) return;

    robloxApi.playPlace(gameDetails.value.root_place_id);
}
</script>

<template>
    <main class="game-page" v-if="gameDetails">
        <div class="game-header grid grid-cols-2 gap-4">
            <div class="game-images">
                <Splide :options="{ rewind: true }" aria-label="My Favorite Images">
                    <SplideSlide v-for="media in gameMedia">
                        <img v-if="media.asset_type === GameMediaType.Image"
                            :src="gameMediaUrls[media.image_id ?? 0] ?? 'https://placehold.co/1920x1080?text=Loading...'"
                            class="rounded-lg" />
                        <img v-else src="https://placehold.co/1920x1080?text=Youtube+Embeds+not+currently+supported"
                            class="rounded-lg" />
                    </SplideSlide>
                </Splide>
            </div>
            <div class="game-info">
                <div class="game-title">
                    <h3 class="text-2xl">{{ gameDetails.name }}</h3>
                    <p>
                        by <b>{{ gameDetails.creator.name }}</b>
                    </p>
                </div>

                <div class="game-play">
                    <button class="play-button" @click="play">
                        <svg class="play-button-icon" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24"
                            height="24" fill="currentColor" viewBox="0 0 24 24">
                            <path fill-rule="evenodd"
                                d="M10.271 5.575C8.967 4.501 7 5.43 7 7.12v9.762c0 1.69 1.967 2.618 3.271 1.544l5.927-4.881a2 2 0 0 0 0-3.088l-5.927-4.88Z"
                                clip-rule="evenodd" />
                        </svg>
                    </button>
                </div>
            </div>
        </div>

        <div class="game-sections" v-if="gameDetails">
            <div class="game-tabs">
                <button :class="currentTab === 0 ? 'tab-active' : ''" @click="currentTab = 0">About</button>
                <button :class="currentTab === 1 ? 'tab-active' : ''" @click="currentTab = 1">Store</button>
                <button :class="currentTab === 2 ? 'tab-active' : ''" @click="currentTab = 2">Servers</button>
            </div>

            <GameDescriptionTab v-if="currentTab === 0" :gameDetails="gameDetails" />
            <div v-if="currentTab === 1" class="game-tab">
                <div class="alert warning">
                    I'm too broke to debug this menu, sooo
                </div>
            </div>

            <div v-if="currentTab === 2" class="game-tab">
                <div class="alert warning">
                    WIP
                </div>
            </div>
        </div>
    </main>
</template>

<style scoped lang="postcss">
.game-page {
    .game-header {
        @apply bg-[#121212] rounded-lg;
        @apply p-3;

        .game-info {
            @apply flex flex-col justify-between;

            .game-play .play-button {
                @apply rounded-lg;
                @apply px-3 py-2 w-full;
                @apply bg-green-600;

                @apply font-bold;

                @apply flex justify-center;

                .play-button-icon {
                    @apply w-10 h-10;
                    @apply text-white;
                }

                &:hover {
                    @apply bg-green-500;
                }

                &:active {
                    @apply bg-green-700;
                }
            }
        }
    }

    .game-sections {
        @apply mt-2 rounded-lg;
        @apply p-2 bg-[#121212];

        .game-tabs {
            @apply grid grid-cols-3 gap-2;

            button {
                @apply w-full p-2;
                @apply rounded-sm;

                &.tab-active {
                    @apply bg-[#242424];
                }

                &:hover {
                    @apply bg-[#242424];
                }

                &:active {
                    @apply bg-[#101010];
                }
            }
        }

        .game-tab {
            @apply p-2;
        }
    }
}
</style>
