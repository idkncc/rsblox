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
        @apply border;
        @apply overflow-hidden backdrop-blur-xl;
        @apply w-full rounded-lg;
        @apply z-50;

        .tooltip-header {
            @apply bg-secondary/80 text-secondary-foreground;
            @apply w-full p-2;
        }

        .tooltip-body > * {
            @apply bg-secondary/60 text-secondary-foreground;
            @apply w-full p-2;
        }
    }
</style>
