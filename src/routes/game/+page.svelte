<script setup lang="ts">
    import "./GamePage.scss";

    import * as Carousel from "@ui/carousel";
    import * as Section from "@ui/section";
    import * as Tabs from "@ui/tabs";

    import GameDescriptionTab from "@components/GameTabs/GameDescriptionTab.svelte";
    import GameStoreTab from "@components/GameTabs/GameStoreTab.svelte";
    import GameServerTab from "@components/GameTabs/GameServerTab.svelte";

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

{#await fetchGameDetails() then _}
    <main class="game-page">
        <Section.Root>
            <Section.Content class="game-header">
                <div class="game-images">
                    {#await fetchGameMedia()}
                        <div
                            class="placeholder rounded-lg w-full aspect-video"
                        />
                    {:then [gameMedia, gameMediaUrls]}
                        <Carousel.Root>
                            <Carousel.Content>
                                {#each gameMedia as media}
                                    <Carousel.Item>
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
                                    </Carousel.Item>
                                {/each}
                            </Carousel.Content>
                            <Carousel.Previous />
                            <Carousel.Next />
                        </Carousel.Root>
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
            </Section.Content>
        </Section.Root>

        <Tabs.Root value="about" class="game-sections">
            <Tabs.List class="grid grid-cols-3">
                <Tabs.Trigger value="about">About</Tabs.Trigger>
                <Tabs.Trigger value="store">Store</Tabs.Trigger>
                <Tabs.Trigger value="servers">Servers</Tabs.Trigger>
            </Tabs.List>

            <Tabs.Content value="about">
                <GameDescriptionTab {gameDetails} />
            </Tabs.Content>
            <Tabs.Content value="store">
                <GameStoreTab {gameDetails} />
            </Tabs.Content>
            <Tabs.Content value="servers">
                <GameServerTab {gameDetails} />
            </Tabs.Content>
        </Tabs.Root>
    </main>
{/await}
