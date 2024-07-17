<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useRoute } from "vue-router";
import { GameDetails } from "../utils/typings.ts";
import { useRobloxApi } from "../utils/robloxApi";

const robloxApi = useRobloxApi();
const route = useRoute();

const gameDetails = ref<GameDetails>();

onMounted(async () => {
    gameDetails.value = await robloxApi.getGameDetails(
        parseInt(route.params.id as string),
    );
});

function play() {
    if (!gameDetails.value) return;

    robloxApi;

    invoke<GameDetails>("plugin:roblox-api|open_place", {
        placeId: gameDetails.value.root_place_id,
    }).then(() => console.info("play"));
}
</script>

<template>
    <main class="game-page" v-if="gameDetails">
        <div class="game-header grid grid-cols-2 gap-4">
            <div class="game-images">
                <img src="https://placehold.co/1920x1080" class="rounded-lg" />
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
                        <svg
                            class="play-button-icon"
                            aria-hidden="true"
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            fill="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M10.271 5.575C8.967 4.501 7 5.43 7 7.12v9.762c0 1.69 1.967 2.618 3.271 1.544l5.927-4.881a2 2 0 0 0 0-3.088l-5.927-4.88Z"
                                clip-rule="evenodd"
                            />
                        </svg>
                    </button>
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
}
</style>
