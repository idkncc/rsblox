<script lang="ts">
    import { robloxApi } from "$lib/robloxApi";
    import { page } from "$app/stores";
    import { PRESENCE_INDEXES } from "$lib/constants";

    import UserStatus from "$lib/components/UserStatus.svelte";
    import FriendCard from "$lib/components/Cards/FriendCard.svelte";
    import FriendCardSkeleton from "$lib/components/Cards/FriendCardSkeleton.svelte";

    import {
        ThumbnailSize,
        ThumbnailType,
        type InternalFriend,
        type UserDetails,
        type UserPresence,
    } from "$lib/typings";

    const userId = parseInt($page.url.searchParams.get("id") ?? "1");

    async function fetchUserDetails(): Promise<[UserDetails, UserPresence]> {
        return Promise.all([
            robloxApi.getUserDetails(userId),
            robloxApi.getPresences([userId]).then((res) => res[0]),
        ]);
    }

    async function fetchUserThumbnail(): Promise<string> {
        return robloxApi
            .getThumbnailsUrls(
                [userId],
                ThumbnailSize.S150x150,
                ThumbnailType.AvatarHeadshot,
            )
            .then((res) => res[0] ?? "https://placehold.co/512?text=Not+Found");
    }

    async function fetchFriends(): Promise<InternalFriend[]> {
        let friendsArray = await robloxApi.getUsersFriendsList(userId);

        friendsArray = friendsArray.sort(
            (a, b) =>
                PRESENCE_INDEXES[b.presence_type] -
                PRESENCE_INDEXES[a.presence_type],
        );

        const [friendsPresencesArray, friendsHeadshotsArray] =
            await Promise.all([
                robloxApi.getPresences(friendsArray.map((fr) => fr.user_id)),
                robloxApi.getThumbnailsUrlsChunked(
                    friendsArray.map((fr) => fr.user_id),
                    ThumbnailSize.S150x150,
                    ThumbnailType.AvatarHeadshot,
                ),
            ]);

        return friendsArray.map((info, i) => ({
            info,
            headshot: friendsHeadshotsArray[i],
            presence: friendsPresencesArray[i],
        }));
    }
</script>

{#await fetchUserDetails()}
    <br />
{:then [userDetails, userPresence]}
    <main class="user-page">
        <section class="section-content user-details">
            {#await fetchUserThumbnail()}
                <div class="user-image-container">
                    <img
                        class="user-image"
                        src="https://placehold.co/512"
                        alt="Loading..."
                    />
                </div>
            {:then thumbnailUrl}
                <div class="user-image-container">
                    <img
                        class="user-image"
                        src={thumbnailUrl}
                        alt={userDetails.display_name}
                    />
                    <UserStatus presenceType={userPresence.presence_type} />
                </div>
            {/await}
            <div class="user-info">
                <div>
                    <p class="user-display-name">{userDetails.display_name}</p>
                    <p class="user-name">@{userDetails.username}</p>
                </div>

                <div class="user-bar">
                    <div class="user-stats">
                        <p>TODO: stats</p>
                    </div>
                    <div class="user-actions">
                        <button class="user-action unfriend">Unfriend</button>
                    </div>
                </div>
            </div>
        </section>

        <section class="section-content user-description">
            <p class="whitespace-pre-wrap text-sm">
                {userDetails.description}
            </p>
        </section>

        <section class="section-content user-friends">
            {#await fetchFriends()}
                {#each Array(8).map(() => 0) as _}
                    <FriendCardSkeleton />
                {/each}
            {:then friends}
                {#each friends as friend}
                    <FriendCard {friend} />
                {/each}
            {/await}
        </section>
    </main>
{/await}

<style lang="scss">
    .user-page {
        @apply flex flex-col gap-2;

        .section-content {
            @apply border border-[#787878] bg-[#121212];
            @apply p-2 rounded-lg;
        }

        .user-details {
            @apply flex gap-2;

            .user-image-container {
                @apply relative;

                .user-image {
                    @apply w-24 rounded-md;
                }
            }

            .user-info {
                @apply flex flex-col justify-between;
                @apply flex-grow;

                div {
                    .user-display-name {
                        @apply font-semibold text-2xl;
                    }
                }

                .user-bar {
                    @apply flex flex-row justify-between items-center;
                    @apply w-full;

                    .user-stats {
                        /* @apply font-semibold text-2xl; */
                    }

                    .user-actions {
                        .user-action {
                            @apply px-2 py-1;
                            @apply rounded-md;

                            &.unfriend {
                                @apply bg-red-900;
                            }

                            &.friend {
                                @apply bg-green-900;
                            }
                        }
                    }
                }
            }
        }

        .use-description {
            @apply overflow-x-scroll;
        }

        .user-friends {
            @apply flex gap-2 overflow-x-scroll;
        }
    }
</style>
