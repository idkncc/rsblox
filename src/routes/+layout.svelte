<script lang="ts">
    import "tippy.js/animations/shift-away-subtle.css";
    import "../css/main.scss";

    import { setContext } from "svelte";
    import { writable } from "svelte/store";
    import { robloxApi } from "$lib/robloxApi";

    import { Store } from "tauri-plugin-store-api";
    import { STORE_PATH } from "$lib/constants";

    import Login from "@components/Login.svelte";
    import Navbar from "@components/navbar/navbar.svelte";

    import { ModeWatcher } from "mode-watcher";

    import type { ClientInfo } from "$lib/typings";

    export const clientInfo = writable<ClientInfo | undefined>();
    export const isLoggedIn = writable(false);

    async function loadAuthorization() {
        async function processCookie() {
            clientInfo.set(await robloxApi.getMe());
        }

        const store = new Store(STORE_PATH);
        let isAlreadyAuthed = await robloxApi.isAuthed();
        const storedCookie = (await store.get<string>("roblox-cookie")) ?? "";

        if (storedCookie.length !== 0 && !isAlreadyAuthed) {
            await robloxApi.auth(storedCookie);
            isAlreadyAuthed = true;
        }

        isLoggedIn.set(isAlreadyAuthed);
        if (isAlreadyAuthed) {
            await processCookie();
        }

        return store.onChange<string>(async (key, value) => {
            if (key !== "roblox-cookie") return;

            const willBeLoggedIn = (value ?? "") !== "";

            isLoggedIn.set(willBeLoggedIn);
            if (willBeLoggedIn) {
                await robloxApi.auth(value!);
                await processCookie();
            } else {
                await robloxApi.auth("");
            }
        }) as Promise<() => void>;
    }

    setContext("clientInfo", clientInfo);
</script>

<ModeWatcher />
<Navbar />

<main class="main-app">
    {#await loadAuthorization()}
        <!-- <p>Getting information about account...</p> -->
    {:then}
        {#if $isLoggedIn}
            <slot></slot>
        {:else}
            <Login />
        {/if}
    {/await}
</main>

<style lang="scss">
    .main-app {
        @apply px-2;
    }
</style>
