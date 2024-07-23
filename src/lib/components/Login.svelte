<script lang="ts">
    import { Store } from "tauri-plugin-store-api";
    import { STORE_PATH } from "$lib/constants";

    let userCookie = "";

    async function loginCookie() {
        const store = new Store(STORE_PATH);
        await store.set("roblox-cookie", userCookie);
        await store.save();
    }
</script>

<h1>Log in</h1>
<div class="alert warning my-2">
    <p>Currently only with cookie</p>
</div>

<div class="flex w-full gap-2">
    <input
        type="text"
        class="control-input flex-grow"
        placeholder="_|WARNING:-DO-NOT-SHARE-THIS.--Sharing-this-will-allow-someone-to-log-in-as-you-and-to-steal-your-ROBUX-and-items.|_..."
        bind:value={userCookie}
    />

    <button class="control-button" on:click={loginCookie}>Login</button>
</div>

<style scoped>
</style>
