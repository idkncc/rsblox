<script lang="ts">
    import "./Navbar.scss";

    import { afterNavigate, goto } from "$app/navigation";
    import { page } from "$app/stores";

    import NavbarProfile from "$lib/components/NavbarProfile.svelte";
    import Input from "../ui/input/input.svelte";

    let searchQuery = "";

    afterNavigate(() => {
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
                    class="search-icon"
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
