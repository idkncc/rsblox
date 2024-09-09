<script lang="ts">
    import { page } from "$app/stores";
    import { robloxApi } from "$lib/robloxApi";

    import GameCard from "@components/Cards/GameCard.svelte";
    import GameCardSkeleton from "@components/Cards/GameCardSkeleton.svelte";

    import {
        ThumbnailSize,
        ThumbnailType,
        TreatmentType,
        type SearchResult,
    } from "$lib/typings";

    const queryString = $page.url.searchParams.get("q")!;

    async function loadSearchResults(): Promise<[SearchResult[], string[]]> {
        const searchResult = await robloxApi.omniSearch(queryString);

        return [
            searchResult,
            await robloxApi.getThumbnailsUrlsChunked(
                searchResult.map((game) => game.universe_id),
                ThumbnailSize.S150x150,
                ThumbnailType.GameIcon,
            ),
        ];
    }
</script>

<main class="search-page">
    {#await loadSearchResults()}
        {#each Array(8).map(() => 0) as _}
            <GameCardSkeleton />
        {/each}
    {:then [results, thumbnails]}
        {#each results as game, i}
            <GameCard {game} thumbnail={thumbnails[i]} />
        {/each}
    {/await}
</main>

<style lang="scss">
    .search-page {
        @apply flex flex-wrap gap-2;
    }
</style>
