<script setup lang="ts">
import { useRobloxApi } from "../../utils/robloxApi"
import { GameServer } from "../../utils/typings"

const robloxApi = useRobloxApi()
const props = defineProps<{
    gameServer: GameServer,
    avatarUrls: Record<string, string>,
    placeId: number
}>()

function join() {
    robloxApi.playServer(props.placeId, props.gameServer.id)
}

</script>

<template>
    <div class="game-server">
        <div class="game-server-avatars">
            <img v-for="playerToken in gameServer.player_tokens"
                :src="avatarUrls[playerToken] ?? 'https://placehold.co/512'" class="bg-[#282828]">

            <div v-if="gameServer.playing > 5" class="w-full h-full flex justify-center items-center">
                <p class="text-2xl font-bold">+{{ gameServer.playing - 5 }}</p>
            </div>
        </div>
        <p>{{ gameServer.playing }} of {{ gameServer.max_players }}</p>

        <div class="game-server-join">
            <button @click="join">Join</button>
        </div>
    </div>
</template>

<style scoped lang="scss">
.game-server {
    @apply p-2;
    @apply bg-[#181818] rounded-lg;

    .game-server-avatars {
        @apply grid grid-cols-3 justify-items-center gap-2;
        @apply mb-2;

        img {
            @apply rounded-md;
        }
    }

    .game-server-join {
        button {
            @apply px-2 py-1 w-full;
            @apply bg-green-600 rounded-md;
        }
    }
}
</style>
