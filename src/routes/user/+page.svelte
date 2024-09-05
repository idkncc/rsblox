<script lang="ts">
    import { getContext } from "svelte";

    import { robloxApi } from "$lib/robloxApi";
    import { page } from "$app/stores";
    import { PRESENCE_INDEXES } from "$lib/constants";

    import UserStatus from "$lib/components/UserStatus.svelte";
    import UserCard from "$lib/components/Cards/UserCard.svelte";
    import UserCardSkeleton from "$lib/components/Cards/UserCardSkeleton.svelte";

    import {
        FriendStatus,
        PresenceType,
        ThumbnailSize,
        ThumbnailType,
        type ClientInfoWritable,
        type InternalUser,
        type UserDetails,
        type UserPresence,
        type UserProfileStats,
    } from "$lib/typings";

    const userId = parseInt($page.url.searchParams.get("id") ?? "1");

    const clientInfo = getContext<ClientInfoWritable>("clientInfo");

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

    async function fetchUserProfileStats(): Promise<UserProfileStats> {
        return robloxApi.getUserStats(userId);
    }

    async function fetchFriends(): Promise<InternalUser[]> {
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

    function isMe() {
        return ($clientInfo?.user_id ?? -1) === userId;
    }

    async function fetchFriendStatus() {
        return robloxApi.getFriendStatus(userId);
    }

    async function fetchAvatar() {
        const avatarImagePromise = robloxApi
            .getThumbnailsUrls(
                [userId],
                ThumbnailSize.S420x420,
                ThumbnailType.Avatar,
            )
            .then((res) => res[0]);

        return Promise.all([avatarImagePromise]);
    }

    // Actions

    async function join(friendPresence: UserPresence) {
        if (friendPresence.place_id && friendPresence.game_id) {
            robloxApi.playServer(
                friendPresence.place_id,
                friendPresence.game_id,
            );
        }
    }

    async function friend() {
        await robloxApi.friend(userId);
        location.reload();
    }

    async function unfriend() {
        await robloxApi.unfriend(userId);
        location.reload();
    }

    async function acceptFriendRequest() {
        await robloxApi.acceptFriendRequest(userId);
        location.reload();
    }

    async function declineFriendRequest() {
        await robloxApi.declineFriendRequest(userId);
        location.reload();
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
                    {#await fetchUserProfileStats()}
                        <div class="user-stats">
                            <span
                                class="placeholder w-full rounded-lg"
                                style="width: 200px"
                            />
                        </div>
                    {:then stats}
                        <div class="user-stats">
                            <p class="user-stat">
                                Friends: <span>{stats.friends}</span>
                            </p>
                            <p class="user-stat">
                                Followers: <span>{stats.followers}</span>
                            </p>
                            <p class="user-stat">
                                Following: <span>{stats.followings}</span>
                            </p>
                        </div>
                    {/await}

                    <div class="user-actions">
                        {#if !isMe()}
                            {#if userPresence.universe_id !== null}
                                <button
                                    class="user-action join"
                                    on:click={() => join(userPresence)}
                                >
                                    Join
                                </button>
                            {/if}

                            {#await fetchFriendStatus()}
                                <!--  -->
                            {:then friendStatus}
                                {#if friendStatus === FriendStatus.NotFriends}
                                    <button
                                        class="user-action friend"
                                        on:click={friend}
                                    >
                                        Friend
                                    </button>
                                {:else if friendStatus === FriendStatus.Friends}
                                    <button
                                        class="user-action unfriend"
                                        on:click={unfriend}
                                    >
                                        Unfriend
                                    </button>
                                {:else if friendStatus === FriendStatus.RequestSent}
                                    <button class="user-action pending">
                                        Pending
                                    </button>
                                {:else if friendStatus === FriendStatus.RequestReceived}
                                    <button
                                        class="user-action request-accept"
                                        on:click={acceptFriendRequest}
                                    >
                                        Accept
                                    </button>
                                    <button
                                        class="user-action request-decline"
                                        on:click={declineFriendRequest}
                                    >
                                        Decline
                                    </button>
                                {/if}
                            {/await}
                        {/if}
                    </div>
                </div>
            </div>
        </section>

        <section class="section-content user-description">
            <p class="whitespace-pre-wrap overflow-hidden text-sm">
                {userDetails.description}
            </p>
        </section>

        <section class="section-content user-friends">
            {#await fetchFriends()}
                {#each Array(8).map(() => 0) as _}
                    <UserCardSkeleton />
                {/each}
            {:then friends}
                {#each friends as friend}
                    <UserCard user={friend} />
                {/each}
            {/await}
        </section>

        {#await fetchAvatar()}
            <section class="section-content user-avatar">
                <div class="avatar-container">
                    <span class="placeholder w-full h-full rounded-md" />
                </div>
                <div class="avatar-items-container">
                    <span class="placeholder w-full h-full rounded-md" />
                </div>
            </section>
        {:then [avatarURL]}
            <section class="section-content user-avatar">
                <div class="avatar-container">
                    <img
                        class="avatar"
                        src={avatarURL}
                        alt={userDetails.display_name}
                    />
                </div>
                <div class="avatar-items-container">
                    <div class="alert warning w-full h-full">
                        <p class="font-bold">
                            There's will be "currently wearing" items
                        </p>
                        <p>work in progress...</p>
                    </div>
                </div>
            </section>
        {/await}
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
                        @apply flex gap-2 items-center;
                        @apply text-sm;

                        .user-stat span {
                            @apply font-semibold;
                        }
                    }

                    .user-actions {
                        .user-action {
                            @apply px-2 py-1;
                            @apply rounded-md;

                            &.join {
                                @apply bg-green-800 mr-2;
                            }

                            &.unfriend {
                                @apply bg-red-900;
                            }

                            &.friend {
                                @apply bg-green-800;
                            }

                            &.pending {
                                @apply bg-[#242424] cursor-default;
                            }

                            &.request- {
                                &accept {
                                    @apply bg-green-800;
                                }
                                &decline {
                                    @apply bg-red-900;
                                }
                            }
                        }
                    }
                }
            }
        }

        .user-description {
            @apply overflow-x-scroll;
        }

        .user-friends {
            @apply flex gap-2 overflow-x-scroll;
        }

        .user-avatar {
            @apply grid grid-cols-2 h-80 gap-2;

            .avatar-container {
                @apply flex justify-center items-center;
                @apply overflow-hidden;
                @apply w-full h-full;

                .avatar {
                    @apply h-[125%];
                }
            }

            .avatar-items-container {
                @apply w-full h-full;
                @apply bg-[#242424] rounded-md;
            }
            /* @apply flex gap-2 overflow-x-scroll; */
        }
    }
</style>
