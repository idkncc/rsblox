<script lang="ts">
    import "./UserTooltip.scss";

    import { goto } from "$app/navigation";
    import { robloxApi } from "$lib/robloxApi";
    import { writeText } from "@tauri-apps/api/clipboard";

    import type {
        UserInformation,
        UserPresence,
        PlaceDetails,
    } from "$lib/typings.ts";

    export let user: UserInformation;
    export let userPresence: UserPresence;
    export let placeDetails: PlaceDetails | undefined;
    export let placeThumbnail: string | undefined;

    function join() {
        if (userPresence.place_id && userPresence.game_id) {
            robloxApi.playServer(userPresence.place_id, userPresence.game_id);
        }
    }

    let copied = false;

    async function copyLink() {
        if (userPresence.place_id && userPresence.game_id) {
            await writeText(
                `roblox://experiences/start?placeId=${userPresence.place_id}&gameInstanceId=${userPresence.game_id}`,
            );

            copied = true;
            setTimeout(() => {
                copied = false;
            }, 2000);
        }
    }
</script>

<div class="user-tooltip" data-user-id={user.user_id}>
    {#if userPresence.place_id}
        <div class="user-presence">
            <img src={placeThumbnail} alt={placeDetails?.name ?? "Thumbnail"} />

            <div class="user-tooltip-footer">
                <p>{placeDetails?.name ?? "Unknown"}</p>
                <div class="join-button-container">
                    <button class="join-button" on:click={join}>Join</button>
                    <button class="copy-button" on:click={copyLink}>
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
            </div>
        </div>
    {/if}

    <div class="user-buttons">
        <p>@{user.username}</p>
        <button
            class="user-button"
            on:click={() => {
                goto(`/user?id=${user.user_id}`).then(() => location.reload());
            }}
        >
            View profile
        </button>
    </div>
</div>
