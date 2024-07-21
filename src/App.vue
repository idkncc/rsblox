<script setup lang="ts">
import { onMounted, ref } from "vue";

import { Store } from "tauri-plugin-store-api";
import { useIntervalFn } from "@vueuse/core";

import { ClientInfo, useRobloxApi } from "./utils/robloxApi.ts";
import { STORE_PATH } from "./constants.ts";

import Login from "./pages/login.vue";
import NavbarProfile from "./components/NavbarProfile/index.vue";

const robloxApi = useRobloxApi();
const clientInfo = ref<ClientInfo | undefined>();

/* This function gets some information about account */
async function processCookie(cookie: string) {
    try {
        robloxApi.auth(cookie)

        clientInfo.value = await robloxApi.getMe()
        robloxApi.loadClientInfo(clientInfo.value);
    } catch (e) {
        console.error("Cookie is invalid!");
        console.error(e);
    }
}

onMounted(async () => {
    const memoryStore = new Store(STORE_PATH);

    const value = await memoryStore.get<string>("roblox-cookie");

    if (value) {
        robloxApi.loadCookie(value);
        await processCookie(value);
    }

    await memoryStore.onChange<string>(async (key, value) => {
        if (key !== "roblox-cookie") return;

        if (value) {
            robloxApi.loadCookie(value);
            await processCookie(value);
        } else {
            robloxApi.$patch({ cookie: undefined, clientInfo: undefined });
        }
    });
});

useIntervalFn(() => {
    robloxApi.updatePresence()
}, 12_000)

</script>

<template>
    <nav class="navbar px-4 p-2 bg-black">
        <RouterLink class="navbar-link" to="/">Home</RouterLink>

        <span class="flex-grow" />

        <NavbarProfile v-if="robloxApi.isLoggedIn" :clientInfo="clientInfo" />
    </nav>

    <main class="main-app">
        <RouterView v-if="robloxApi.isLoggedIn" />
        <Login v-else />
    </main>
</template>
