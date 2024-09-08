<script lang="ts">
    import "tippy.js/animations/shift-away.css";
    import "tippy.js/animations/shift-away-subtle.css";

    import "./UserCard.scss";

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
    import * as Avatar from "$lib/components/ui/avatar";

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
                <Avatar.Root class="friend-image-root">
                    <Avatar.Image
                        src={user.headshot}
                        alt={`@${user.info.display_name}`}
                    />
                    <Avatar.Fallback
                        >{user.info.display_name
                            .slice(0, 2)
                            .toUpperCase()}</Avatar.Fallback
                    >
                </Avatar.Root>

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
