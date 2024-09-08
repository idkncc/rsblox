<script lang="ts">
    import "./GameServerItem.scss";

    import { robloxApi } from "$lib/robloxApi";
    import type { GameServer } from "$lib/typings";
    import { writeText } from "@tauri-apps/api/clipboard";
    import * as Card from "@ui/card";
    import * as Avatar from "@ui/avatar";
    import { Description } from "../ui/section";

    export let gameServer: GameServer;
    export let avatarUrls: Record<string, string>;
    export let placeId: number;

    function join() {
        robloxApi.playServer(placeId, gameServer.id);
    }

    let copied = false;

    async function copyLink() {
        await writeText(
            `roblox://experiences/start?placeId=${placeId}&gameInstanceId=${gameServer.id}`,
        );

        copied = true;
        setTimeout(() => {
            copied = false;
        }, 2000);
    }
</script>

<Card.Root class="game-server">
    <Card.Content class="p-3">
        <div class="game-server-avatars">
            {#each gameServer.player_tokens as playerToken}
                <Avatar.Root class="avatar-image">
                    <Avatar.Image
                        src={avatarUrls[playerToken]}
                        alt="Player Headshot"
                    />
                    <Avatar.Fallback>???</Avatar.Fallback>
                </Avatar.Root>
            {/each}

            {#if gameServer.playing > 5}
                <Avatar.Root class="avatar-image">
                    <Avatar.Fallback class="text-xl font-bold"
                        >+{gameServer.playing - 5}</Avatar.Fallback
                    >
                </Avatar.Root>
            {/if}
        </div>
        <p>{gameServer.playing} of {gameServer.max_players}</p>

        <div class="game-server-join-container">
            <button class="game-server-join" on:click={join}>Join</button>

            <button class="game-server-copy" on:click={copyLink}>
                {#if copied}
                    <svg
                        class="w-6 h-6"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        fill="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M9 2a1 1 0 0 0-1 1H6a2 2 0 0 0-2 2v15a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2h-2a1 1 0 0 0-1-1H9Zm1 2h4v2h1a1 1 0 1 1 0 2H9a1 1 0 0 1 0-2h1V4Zm5.707 8.707a1 1 0 0 0-1.414-1.414L11 14.586l-1.293-1.293a1 1 0 0 0-1.414 1.414l2 2a1 1 0 0 0 1.414 0l4-4Z"
                            clip-rule="evenodd"
                        />
                    </svg>
                {:else}
                    <svg
                        class="w-6 h-6"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        fill="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M8 3a1 1 0 0 1 1-1h6a1 1 0 0 1 1 1h2a2 2 0 0 1 2 2v15a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h2Zm6 1h-4v2H9a1 1 0 0 0 0 2h6a1 1 0 1 0 0-2h-1V4Zm-6 8a1 1 0 0 1 1-1h6a1 1 0 1 1 0 2H9a1 1 0 0 1-1-1Zm1 3a1 1 0 1 0 0 2h6a1 1 0 1 0 0-2H9Z"
                            clip-rule="evenodd"
                        />
                    </svg>
                {/if}
            </button>
        </div>
    </Card.Content>
</Card.Root>

<style lang="scss">
</style>
