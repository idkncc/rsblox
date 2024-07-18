<script setup lang="ts">
import { useRobloxApi } from "../../utils/robloxApi.ts";
import { Store } from "tauri-plugin-store-api";
import { STORE_PATH } from "../../constants.ts";

const robloxApi = useRobloxApi()

async function logOut() {
    const memoryStore = new Store(STORE_PATH);
    await memoryStore.delete("roblox-cookie")
    await memoryStore.save()

    robloxApi.$reset()
}
</script>

<template>
    <div class="navbar-profile-tooltip">
        <div class="tooltip-header">
            @{{ robloxApi.clientInfo?.username ?? "loading" }}
        </div>
        <div class="tooltip-body">
            <button @click="logOut">Logout</button>
        </div>
    </div>
</template>

<style scoped>
.navbar-profile-tooltip {
    @apply border border-[#787878];
    @apply overflow-hidden backdrop-blur-lg;
    @apply w-full rounded-lg;
    @apply z-50;

    .tooltip-header {
        @apply bg-[#121212]/75;
        @apply w-full p-2;
    }

    .tooltip-body>* {
        @apply bg-[#212121]/75;
        @apply w-full p-2;
    }
}
</style>
