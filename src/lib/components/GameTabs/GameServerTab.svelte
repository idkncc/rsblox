<script setup lang="ts">
    import chunk from "lodash.chunk";

    import { robloxApi } from "$lib/robloxApi";

    import * as Section from "@ui/section";

    import GameServerItem from "./GameServerItem.svelte";

    import {
        ThumbnailSize,
        ThumbnailType,
        type GameDetails,
        type GameServer,
    } from "$lib/typings";

    export let gameDetails: GameDetails;

    let friendsServers: {
        servers: GameServer[];
        nextCursor: string | null;
    } = { servers: [], nextCursor: null };
    let publicServers: {
        servers: GameServer[];
        nextCursor: string | null;
    } = { servers: [], nextCursor: null };
    let loadMoreAvatars = true;

    $: if (loadMoreAvatars) {
        loadMoreAvatars = false;

        loadAvatars([
            ...friendsServers.servers.map((s) => s.player_tokens),
            ...publicServers.servers.map((s) => s.player_tokens),
        ]).then((newAvatarUrls) => {
            avatarUrls = Object.assign(avatarUrls, newAvatarUrls);
        });
    }

    let avatarUrls: Record<string, string> = {};

    async function loadAvatars(
        serversPlayerTokens: string[][],
    ): Promise<Record<string, string>> {
        const alreadyLoadedTokens = Object.keys(avatarUrls);
        const newAvatarTokens: string[] = serversPlayerTokens
            .flat(1)
            .filter((token) => !alreadyLoadedTokens.includes(token));

        const newAvatarUrls = await Promise.all(
            chunk(newAvatarTokens, 15).map((chunkedAvatarTokens) =>
                robloxApi.getTokensThumbnailsUrls(
                    newAvatarTokens,
                    ThumbnailSize.S150x150,
                    ThumbnailType.AvatarHeadshot,
                ),
            ),
        ).then((res) => res.flat(1));

        return Object.fromEntries(
            newAvatarTokens.map((token, i) => [token, newAvatarUrls[i]]),
        );
    }

    async function fetchFriendsServers() {
        const [_friendsGameServers, _friendsNextCursor] =
            await robloxApi.getGameServers(
                gameDetails.root_place_id,
                "Friends",
            );

        friendsServers = {
            servers: _friendsGameServers,
            nextCursor: _friendsNextCursor,
        };
        loadMoreAvatars = true;
    }
    async function fetchPublicServers() {
        const [_publicGameServers, _publicNextCursor] =
            await robloxApi.getGameServers(gameDetails.root_place_id, "Public");

        publicServers = {
            servers: _publicGameServers,
            nextCursor: _publicNextCursor,
        };
        loadMoreAvatars = true;
    }

    async function fetchNextFriendsServers() {
        if (!friendsServers.nextCursor) return;

        const [_friendsGameServers, _friendsNextCursor] =
            await robloxApi.getGameServers(
                gameDetails.root_place_id,
                "Friends",
                friendsServers.nextCursor,
            );

        friendsServers = {
            servers: [...friendsServers.servers, ..._friendsGameServers],
            nextCursor: _friendsNextCursor,
        };
        loadMoreAvatars = true;
    }

    async function fetchNextPublicServers() {
        if (!publicServers.nextCursor) return;

        const [_publicGameServers, _publicNextCursor] =
            await robloxApi.getGameServers(
                gameDetails.root_place_id,
                "Public",
                publicServers.nextCursor,
            );

        publicServers = {
            servers: [...publicServers.servers, ..._publicGameServers],
            nextCursor: _publicNextCursor,
        };
        loadMoreAvatars = true;
    }

    // function refresh(servers_type: "Public" | "Friends") {
    //     if (servers_type === "Friends") {
    //         friendsServers.value = {
    //             servers: [],
    //             nextCursor: null
    //         }
    //     }
    // }
</script>

<Section.Root>
    <!-- <button>Refresh</button> -->
    <Section.Title>Friends Servers:</Section.Title>
    <Section.Content>
        {#await fetchFriendsServers()}
            TODO: skeleton elements
        {:then _}
            <div class="game-servers">
                {#each friendsServers.servers as gameServer}
                    <GameServerItem
                        {gameServer}
                        {avatarUrls}
                        placeId={gameDetails.root_place_id}
                    />
                {/each}
            </div>
            {#if friendsServers.nextCursor !== null}
                <button on:click={fetchNextFriendsServers}>
                    Load next servers
                </button>
            {/if}
        {/await}
    </Section.Content>
</Section.Root>

<Section.Root>
    <Section.Title>Public Servers:</Section.Title>
    <Section.Content>
        {#await fetchPublicServers()}
            TODO: skeleton elements
        {:then _}
            <!-- TODO: filters (sortOrder, excludeFullServers)  -->

            <div class="game-servers">
                {#each publicServers.servers as gameServer}
                    <GameServerItem
                        {gameServer}
                        {avatarUrls}
                        placeId={gameDetails.root_place_id}
                    />
                {/each}
            </div>

            {#if publicServers.nextCursor !== null}
                <!-- TODO: Pagination   -->
                <button on:click={fetchNextPublicServers}>
                    Load next servers
                </button>
            {/if}
        {/await}
    </Section.Content>
</Section.Root>

<style scoped lang="scss">
    .game-servers {
        @apply grid grid-cols-3 gap-2;
    }

    .game-servers-type {
        .game-servers-title {
            @apply flex justify-between;

            button {
                @apply underline font-medium;
            }
        }
    }
</style>
