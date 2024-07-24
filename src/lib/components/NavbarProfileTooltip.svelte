<script lang="ts">
    import { getContext } from "svelte";

    import { Store } from "tauri-plugin-store-api";
    import { STORE_PATH } from "$lib/constants";

    import { type ClientInfoWritable } from "$lib/typings";

    const clientInfo = getContext<ClientInfoWritable>("clientInfo");

    async function logOut() {
        const memoryStore = new Store(STORE_PATH);
        await memoryStore.delete("roblox-cookie");
        await memoryStore.save();
    }
</script>

<div id="navbar-profile-tooltip">
    <div class="tooltip-header">
        @{$clientInfo?.username ?? "loading"}
    </div>
    <div class="tooltip-body">
        <button on:click={logOut}>Logout</button>
    </div>
</div>

<style lang="scss">
    #navbar-profile-tooltip {
        @apply border border-[#787878];
        @apply overflow-hidden backdrop-blur-lg;
        @apply w-full rounded-lg;
        @apply z-50;

        .tooltip-header {
            @apply bg-[#121212]/75;
            @apply w-full p-2;
        }

        .tooltip-body > * {
            @apply bg-[#212121]/75;
            @apply w-full p-2;
        }
    }
</style>
