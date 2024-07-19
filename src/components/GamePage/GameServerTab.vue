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

const gameServers = ref<GameServer[]>([])
const avatarUrls = ref<Record<string, string>>({})
const nextCursor = ref<string | undefined>()

const props = defineProps<{
    gameDetails: GameDetails
}>()

async function loadAvatars(serversPlayerTokens: string[][]) {
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
    gameServers.value = []
    nextCursor.value = undefined

    const [_gameServers, _nextCursor] = await robloxApi.getGameServers(props.gameDetails.root_place_id)
    const _avatarUrls = await loadAvatars(_gameServers.map((s) => s.player_tokens))

    console.log(_nextCursor)
    gameServers.value = _gameServers
    nextCursor.value = _nextCursor
    avatarUrls.value = _avatarUrls
})

async function loadNextServers() {
    console.log("usin", nextCursor.value)
    const [_gameServers, _nextCursor] = await robloxApi.getGameServers(props.gameDetails.root_place_id, nextCursor.value)
    const _avatarUrls = await loadAvatars(_gameServers.map((s) => s.player_tokens))

    gameServers.value = [...gameServers.value, ..._gameServers]
    nextCursor.value = _nextCursor
    avatarUrls.value = Object.assign(avatarUrls.value, _avatarUrls)
}
</script>

<template>
    <section class="game-tab game-servers">
        <GameServerItem v-for="gameServer in gameServers" :gameServer="gameServer" :avatarUrls="avatarUrls"
            :placeId="gameDetails.root_place_id" />
    </section>
    <button @click="loadNextServers">Load next servers</button>
</template>

<style scoped lang="scss">
.game-servers {
    @apply grid grid-cols-3 gap-2;
}
</style>
