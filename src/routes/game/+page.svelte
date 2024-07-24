<script setup lang="ts">
    import { register } from "swiper/element/bundle";
    register();

    import GameDescriptionTab from "$lib/components/GameTabs/GameDescriptionTab.svelte";
    import GameStoreTab from "$lib/components/GameTabs/GameStoreTab.svelte";
    import GameServerTab from "$lib/components/GameTabs/GameServerTab.svelte";

    import { page } from "$app/stores";
    import { robloxApi } from "$lib/robloxApi";

    import {
        GameMediaType,
        ThumbnailSize,
        ThumbnailType,
        type GameDetails,
        type GameMedia,
    } from "$lib/typings";

    let currentTab: number = 0;

    let gameDetails: GameDetails;

    async function fetchGameDetails() {
        gameDetails = await robloxApi.getGameDetails(
            parseInt(($page.url.searchParams.get("id") ?? "0") as string),
        );
    }

    async function fetchGameMedia(): Promise<
        [GameMedia[], Record<number, string>]
    > {
        const gameMedia = await robloxApi.getGameMedia(gameDetails.universe_id);

        const gameMediaImages = gameMedia
            .filter((media) => media.image_id)
            .map((media) => media.image_id) as number[];

        const _gameMediaUrls = await robloxApi.getThumbnailsUrls(
            gameMediaImages,
            ThumbnailSize.S768x432,
            ThumbnailType.Asset,
        );

        const gameMediaUrls = Object.fromEntries(
            gameMediaImages.map((id, i) => [id, _gameMediaUrls[i]]),
        );

        return [gameMedia, gameMediaUrls];
    }

    function play() {
        if (!gameDetails) return;

        robloxApi.playPlace(gameDetails.root_place_id);
    }
</script>

{#await fetchGameDetails()}
    TOOD: skeleton elements
{:then _}
    <main class="game-page">
        <div class="game-header grid grid-cols-2 gap-4">
            <div class="game-images">
                {#await fetchGameMedia()}
                    TODO: skeleton elements
                {:then [gameMedia, gameMediaUrls]}
                    <swiper-container
                        navigation="true"
                        pagination="true"
                        space-between={10}
                    >
                        >
                        {#each gameMedia as media}
                            <swiper-slide>
                                {#if media.asset_type === GameMediaType.Image}
                                    <img
                                        src={gameMediaUrls[
                                            media.image_id ?? 0
                                        ] ??
                                            "https://placehold.co/1920x1080?text=Loading..."}
                                        alt={media.alt_text}
                                        class="rounded-lg w-[calc(100%-10px)]"
                                    />
                                {:else}
                                    <img
                                        src={"https://placehold.co/1920x1080?text=Youtube+Embeds+not+currently+supported"}
                                        alt={media.alt_text}
                                        class="rounded-lg w-[calc(100%-10px)]"
                                    />
                                {/if}
                            </swiper-slide>
                        {/each}
                    </swiper-container>
                {/await}
            </div>
            <div class="game-info">
                <div class="game-title">
                    <h3 class="text-2xl">{gameDetails.name}</h3>
                    <p>
                        by <b>{gameDetails.creator.name}</b>
                    </p>
                </div>

                <div class="game-play">
                    <button class="play-button" on:click={play}>
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

        <div class="game-sections">
            <div class="game-tabs">
                <button
                    class={currentTab === 0 ? "tab-active" : ""}
                    on:click={() => (currentTab = 0)}
                >
                    About
                </button>
                <button
                    class={currentTab === 1 ? "tab-active" : ""}
                    on:click={() => (currentTab = 1)}
                >
                    Store
                </button>
                <button
                    class={currentTab === 2 ? "tab-active" : ""}
                    on:click={() => (currentTab = 2)}
                >
                    Servers
                </button>
            </div>

            <GameDescriptionTab visible={currentTab === 0} {gameDetails} />
            <GameStoreTab visible={currentTab === 1} {gameDetails} />
            <GameServerTab visible={currentTab === 2} {gameDetails} />
        </div>
    </main>
{/await}

<style lang="scss">
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
                @apply grid grid-cols-3 gap-2 mb-2;

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
            }
        }
    }
</style>
