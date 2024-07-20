<script setup lang="ts">
import { onMounted, ref } from "vue";

import { useRobloxApi } from "../../utils/robloxApi";

import {
    GameDetails,
    GameServer,
    ThumbnailSize,
    ThumbnailType
} from "../../utils/typings";

import GameServerItem from "./GameServerItem.vue";

const robloxApi = useRobloxApi()

const friendsServers = ref<{
    servers: GameServer[]
    nextCursor: string | null
}>({ servers: [], nextCursor: null })
const publicServers = ref<{
    servers: GameServer[]
    nextCursor: string | null
}>({ servers: [], nextCursor: null })

const avatarUrls = ref<Record<string, string>>({})

const props = defineProps<{
    gameDetails: GameDetails
}>()

async function loadAvatars(serversPlayerTokens: string[][]): Promise<Record<string, string>> {
    return Object.fromEntries(
        (await Promise.all(
            serversPlayerTokens
                .map((avatarTokens) => (
                    robloxApi.getTokensThumbnailsUrls(
                        avatarTokens,
                        ThumbnailSize.S150x150,
                        ThumbnailType.AvatarHeadshot
                    ).then((urls) => [avatarTokens, urls])
                ))
        ))
            .map(([tokens, urls]) => tokens.map((token, i) => [token, urls[i]]))
            .flat(1)
    )
}

onMounted(async () => {
    const [_friendsGameServers, _friendsNextCursor] = await robloxApi.getGameServers(props.gameDetails.root_place_id, "Friends")
    const [_publicGameServers, _publicNextCursor] = await robloxApi.getGameServers(props.gameDetails.root_place_id, "Public")

    const _avatarUrls = await loadAvatars([
        ..._friendsGameServers.map((s) => s.player_tokens),
        ..._publicGameServers.map((s) => s.player_tokens)
    ])

    friendsServers.value = {
        servers: _friendsGameServers,
        nextCursor: _friendsNextCursor
    }

    publicServers.value = {
        servers: _publicGameServers,
        nextCursor: _publicNextCursor
    }

    avatarUrls.value = _avatarUrls
})

async function loadNextFriendsServers() {
    if (!friendsServers.value.nextCursor) return;

    const [_friendsGameServers, _friendsNextCursor] = await robloxApi.getGameServers(
        props.gameDetails.root_place_id,
        "Friends",
        friendsServers.value.nextCursor
    )

    const _avatarUrls = await loadAvatars(_friendsGameServers.map((s) => s.player_tokens))

    friendsServers.value = {
        servers: [...friendsServers.value.servers, ..._friendsGameServers],
        nextCursor: _friendsNextCursor
    }

    avatarUrls.value = Object.assign(avatarUrls.value, _avatarUrls)
}

async function loadNextPublicServers() {
    if (!publicServers.value.nextCursor) return;

    const [_publicGameServers, _publicNextCursor] = await robloxApi.getGameServers(
        props.gameDetails.root_place_id,
        "Public",
        publicServers.value.nextCursor
    )

    const _avatarUrls = await loadAvatars(_publicGameServers.map((s) => s.player_tokens))

    publicServers.value = {
        servers: [...publicServers.value.servers, ..._publicGameServers],
        nextCursor: _publicNextCursor
    }

    avatarUrls.value = Object.assign(avatarUrls.value, _avatarUrls)
}

function refresh(servers_type: "Public" | "Friends") {
    if (servers_type === "Friends") {
        friendsServers.value = {
            servers: [],
            nextCursor: null
        }
    }
}
</script>

<template>
    <section class="game-tab">
        <section class="game-servers-type">
            <div class="game-servers-title">
                <p>Friends:</p>
                <!-- <button>Refresh</button> -->
            </div>
            <div class="game-servers">
                <GameServerItem v-for="gameServer in friendsServers.servers" :gameServer="gameServer"
                    :avatarUrls="avatarUrls" :placeId="gameDetails.root_place_id" />
            </div>
            <button v-if="friendsServers.nextCursor !== null" @click="loadNextFriendsServers">Load next servers</button>
        </section>

        <section class="game-servers-type">
            <!-- TODO: filters (sortOrder, excludeFullServers)  -->
            <div class="game-servers-title">
                <p>Public Servers:</p>
                <!-- <button>Refresh</button> -->
            </div>
            <div class="game-servers">
                <GameServerItem v-for="gameServer in publicServers.servers" :gameServer="gameServer"
                    :avatarUrls="avatarUrls" :placeId="gameDetails.root_place_id" />
            </div>
            <button v-if="publicServers.nextCursor !== null" @click="loadNextPublicServers">Load next servers</button>
        </section>
    </section>
</template>

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
