<script lang="ts">
    import "tippy.js/animations/shift-away.css";
    import "tippy.js/animations/shift-away-subtle.css";

    import UserTooltip from "./UserTooltip.svelte";

    import {
        ThumbnailSize,
        ThumbnailType,
        type InternalUser,
        type PlaceDetails,
    } from "$lib/typings";
    import { robloxApi } from "$lib/robloxApi";
    import { tippy } from "$lib/tippy";
    import UserStatus from "../UserStatus.svelte";

    export let user: InternalUser;

    let tippyTooltip: HTMLDivElement;
    // onMounted(async () => {
    async function fetchDataForTooltip() {
        let placeDetails: PlaceDetails | undefined;
        let placeThumbnail: string = "https://placehold.co/512";
        if (
            user.presence.presence_type === "InGame" &&
            user.presence.place_id
        ) {
            const currentPlaceDetails = await robloxApi.getPlaceDetails(
                user.presence.place_id,
            );

            placeDetails = await robloxApi.getPlaceDetails(
                currentPlaceDetails.universe_root_place_id,
            );

            placeThumbnail = await robloxApi
                .getThumbnailsUrls(
                    [placeDetails.universe_root_place_id],
                    ThumbnailSize.S768x432,
                    ThumbnailType.GameThumbnail,
                )
                .then((res) => res[0]);
        }

        setTimeout(() => {
            tippyTooltip = document.querySelector(
                `.user-tooltip[data-user-id="${user.info.user_id}"]`,
            )!;
        }, 20);

        return {
            user: user.info,
            userPresence: user.presence,
            placeDetails,
            placeThumbnail,
        };
    }
</script>

<div
    class="friend-card-container"
    use:tippy={{ content: tippyTooltip, placement: "bottom" }}
>
    <a data-sveltekit-reload href="/user?id={user.info.user_id}">
        <div class="friend-card">
            <div class="friend-image">
                <img
                    class="rounded-md"
                    src={user.headshot}
                    alt={user.info.display_name}
                    width="90"
                    height="90"
                />

                <UserStatus presenceType={user.presence.presence_type} />
            </div>

            <div class="friend-username">
                <p class="user-name">{user.info.display_name}</p>
            </div>
        </div>
    </a>

    {#await fetchDataForTooltip() then props}
        <UserTooltip {...props} />
    {/await}
</div>

<style lang="scss">
    .friend-card {
        --size: 100px;

        @media screen and (max-width: 920px) {
            --size: 80px;
        }

        @apply flex flex-col;

        .friend-image {
            width: var(--size);
            height: var(--size);

            @apply aspect-square;
            @apply bg-[#787878] rounded-full relative;

            img {
                @apply w-full rounded-full;
            }
        }

        div.friend-username {
            @apply text-xs text-center;
            @apply flex gap-1 justify-center;

            width: var(--size);

            .user-name {
                max-width: calc(var(--size) - 10);
                @apply truncate;
            }

            .user-status {
                @apply w-fit;
            }
        }
    }
</style>
