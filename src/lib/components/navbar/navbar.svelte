<script lang="ts">
    import { afterNavigate, goto } from "$app/navigation";
    import { page } from "$app/stores";

    import NavbarProfile from "$lib/components/NavbarProfile.svelte";
    import Input from "../ui/input/input.svelte";

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
</script>

<nav class="navbar px-4 p-2">
    <div class="navbar-links">
        <a class="navbar-link" href="/">Home</a>
    </div>

    <div class="navbar-search">
        <div class="input-container">
            <Input
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

<style lang="scss">
    nav.navbar {
        @apply text-lg select-none cursor-default;
        @apply grid grid-cols-[1fr_2fr_1fr] items-center gap-4;
        @apply sticky top-0;
        @apply z-40;

        @apply bg-black/60 backdrop-blur-2xl;

        .navbar-search {
            .input-container {
                @apply relative;

                input {
                    @media (screen and max-width: 400px) {
                        @apply hidden;
                    }

                    @apply w-full py-0.5;
                    @apply text-base;
                }

                .search-button {
                    @apply absolute top-0 right-0;
                    @apply h-full p-1;

                    .search-icon {
                        @apply transition-all;
                        @apply text-[#787878];
                        @apply h-full;
                    }

                    &:hover .search-icon {
                        @apply text-[#A4A4A4];
                    }

                    &:active .search-icon {
                        @apply text-[#646464];
                    }
                }
            }
        }

        .navbar-profile {
            @apply justify-self-end;
        }

        .navbar-link {
            @apply transition-all;

            &.router-link-active {
                @apply font-bold;
            }

            &:hover {
                @apply font-bold;
            }
        }
    }
</style>
