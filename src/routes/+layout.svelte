<script lang="ts">
    import "victormono";
    import "tippy.js/animations/shift-away-subtle.css";
    import "../css/main.scss";

    import { onMount, setContext } from "svelte";
    import { writable, readable } from "svelte/store";
    import { robloxApi } from "$lib/robloxApi";

    import { Store } from "tauri-plugin-store-api";
    import { STORE_PATH } from "$lib/constants";

    import Login from "$lib/components/Login.svelte";
    import NavbarProfile from "$lib/components/NavbarProfile.svelte";

    import type { ClientInfo } from "$lib/typings";
    import { afterNavigate, goto } from "$app/navigation";
    import { page } from "$app/stores";

    export const clientInfo = writable<ClientInfo | undefined>();
    export const isLoggedIn = writable(false);

    async function loadAuthorization() {
        // await ((n) => new Promise((res) => setTimeout(res, n)))(5e3);

        async function processCookie() {
            console.log("Procesin");
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

    let searchQuery = "";
    // $: searchQuery = $page.url.searchParams.get("q") ?? "";
    //

    afterNavigate(() => {
        console.log("movin");
        if (location.pathname.startsWith("/search")) {
            searchQuery = $page.url.searchParams.get("q") ?? "";
        } else {
            searchQuery = "";
        }
    });

    async function search() {
        goto(`/search?q=${encodeURIComponent(searchQuery)}`).then(() => {
            location.reload();
        });
    }

    setContext("clientInfo", clientInfo);
</script>

<nav class="navbar px-4 p-2">
    <div class="navbar-links">
        <a class="navbar-link" href="/">Home</a>
    </div>

    <div class="navbar-search">
        <div class="input-container">
            <input
                class="control-input"
                type="search"
                placeholder="Search..."
                bind:value={searchQuery}
                on:keypress={(key) => {
                    if (key.key === "Enter") search();
                }}
            />
            <button class="search-button" on:click={search}>
                <svg
                    class="search-icon text-white"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-width="2"
                        d="m21 21-3.5-3.5M17 10a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z"
                    />
                </svg>
            </button>
        </div>
    </div>

    <div class="navbar-profile">
        <NavbarProfile />
    </div>

    <!-- <NavbarProfile v-if="robloxApi.isLoggedIn" :clientInfo="clientInfo" /> -->
</nav>

<main class="main-app">
    {#await loadAuthorization()}
        <p>Getting information about account...</p>
    {:then}
        {#if $isLoggedIn}
            <slot></slot>
        {:else}
            <Login />
        {/if}
    {/await}
</main>
