<script lang="ts">
    import "./NavbarProfileTooltip.scss";

    import { getContext } from "svelte";

    import { Store } from "tauri-plugin-store-api";
    import { STORE_PATH } from "$lib/constants";

    import { Button } from "@ui/button";

    import { type ClientInfoWritable } from "$lib/typings";
    import { goto } from "$app/navigation";

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
        <a href="/settings">
            <Button variant="ghost" class="w-full">Settings</Button>
        </a>
        <Button variant="ghost" on:click={logOut}>Logout</Button>
    </div>
</div>
