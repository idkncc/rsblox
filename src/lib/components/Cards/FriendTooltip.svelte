<script lang="ts">
    import { robloxApi } from "$lib/robloxApi";

    import type {
        FriendUserInformation,
        UserPresence,
        PlaceDetails,
    } from "$lib/typings.ts";

    export let friend: FriendUserInformation;
    export let friendPresence: UserPresence;
    export let placeDetails: PlaceDetails | undefined;
    export let placeThumbnail: string | undefined;

    function join() {
        if (friendPresence.place_id && friendPresence.game_id) {
            robloxApi.playServer(
                friendPresence.place_id,
                friendPresence.game_id,
            );
        }
    }
</script>

<div class="friend-tooltip" data-user-id={friend.user_id}>
    {#if friendPresence.place_id}
        <div class="friend-presence">
            <img src={placeThumbnail} alt={placeDetails?.name ?? "Thumbnail"} />

            <div class="friend-tooltip-footer">
                <p>{placeDetails?.name ?? "Unknown"}</p>
                <button class="join-button" on:click={join}>Join</button>
            </div>
        </div>
    {/if}

    <div class="friend-buttons">
        <p>@{friend.username}</p>
        <button class="friend-button">View profile</button>
    </div>
</div>

<style lang="scss">
    $width: 192px;
    $height: 108px;

    .friend-tooltip {
        @apply flex flex-col;

        @apply bg-black/50 backdrop-blur-lg;
        @apply overflow-hidden rounded-lg;

        max-width: $width;

        .friend-buttons {
            @apply p-2;

            & > * {
                @apply w-full px-2 py-1;
                @apply rounded-md;
                @apply text-center truncate;
            }

            & > .friend-button {
                @apply bg-[#525252];
            }
        }
    }

    .friend-presence {
        width: $width;
        height: $height;
        position: relative;

        img {
            @apply absolute top-0 left-0;
            @apply w-full;

            height: $height;
        }

        .friend-tooltip-footer {
            @apply absolute bottom-0 left-0;
            @apply px-2 py-3;
            width: $width;

            @apply bg-gradient-to-t from-black to-white/0;

            .join-button {
                @apply px-2 py-1 w-full;
                @apply bg-green-600 rounded-sm;
            }
        }
    }
</style>
