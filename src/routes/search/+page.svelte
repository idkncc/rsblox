<script lang="ts">
    import { page } from "$app/stores";
    import GameCard from "$lib/components/Cards/GameCard.svelte";
    import { robloxApi } from "$lib/robloxApi";
    import { ThumbnailSize, ThumbnailType, TreatmentType } from "$lib/typings";

    const queryString = $page.url.searchParams.get("q")!;

    async function loadSearchResults() {
        const searchResult = await robloxApi.omniSearch(queryString);

        console.log(searchResult);

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
        <!--  -->
    {:then [results, thumbnails]}
        {#each results as game, i}
            <GameCard
                {game}
                thumbnail={thumbnails[i]}
                treatmentType={TreatmentType.Carousel}
            />
        {/each}
    {/await}
</main>

<style lang="scss">
    .search-page {
        @apply flex flex-wrap gap-2;
    }
</style>
