<script lang="ts">
    import { Store } from "tauri-plugin-store-api";
    import { STORE_PATH } from "$lib/constants";
    import * as Alert from "@ui/alert";
    import Input from "./ui/input/input.svelte";
    import Button from "./ui/button/button.svelte";

    let userCookie = "";

    async function loginCookie() {
        const store = new Store(STORE_PATH);
        await store.set("roblox-cookie", userCookie);
        await store.save();
    }
</script>

<h1 class="text-3xl font-bold mb-2">Log in</h1>

<Alert.Root variant="warning">
    <Alert.Title>Warning</Alert.Title>
    <Alert.Description>
        <p>Currently, you can log in only with cookie</p>
        <p>
            Maybe in the future, I could make with Quick Login or
            Username&Password
        </p>
    </Alert.Description>
</Alert.Root>

<div class="flex w-full gap-2 mt-2">
    <Input
        type="text"
        class="control-input flex-grow"
        placeholder="_|WARNING:-DO-NOT-SHARE-THIS.--Sharing-this-will-allow-someone-to-log-in-as-you-and-to-steal-your-ROBUX-and-items.|_..."
        bind:value={userCookie}
    />

    <Button class="h-auto" on:click={loginCookie}>Login</Button>
</div>

<style scoped>
</style>
