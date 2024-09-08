<script lang="ts">
    import "./user.scss";

    import { getContext } from "svelte";

    import { robloxApi } from "$lib/robloxApi";
    import { page } from "$app/stores";
    import { PRESENCE_INDEXES } from "$lib/constants";

    import UserStatus from "$lib/components/UserStatus.svelte";
    import UserCard from "$lib/components/Cards/UserCard.svelte";
    import UserCardSkeleton from "$lib/components/Cards/UserCardSkeleton.svelte";

    import * as Section from "@/components/ui/section";
    import * as Avatar from "@/components/ui/avatar";

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
    import Skeleton from "@/components/ui/skeleton/skeleton.svelte";

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

    let friendsCount = -1;
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

        friendsCount = friendsArray.length;

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
        <Section.Root class="user-details">
            <Section.Content>
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
                        <Avatar.Root class="rounded-lg w-32 h-32">
                            <Avatar.Image src={thumbnailUrl} alt="@shadcn" />
                            <Avatar.Fallback>CN</Avatar.Fallback>
                        </Avatar.Root>
                        <UserStatus presenceType={userPresence.presence_type} />
                    </div>
                {/await}

                <div class="user-info">
                    <div>
                        <p class="user-display-name">
                            {userDetails.display_name}
                        </p>
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
            </Section.Content>
        </Section.Root>
        <Section.Root class="user-description">
            <Section.Content
                class="whitespace-pre-wrap overflow-hidden text-sm"
            >
                {userDetails.description}
            </Section.Content>
        </Section.Root>

        <Section.Root class="user-friends">
            {#if friendsCount === -1}
                <!-- loading animation -->
                <Section.Title class="flex">
                    {`Friends (`}<Skeleton class="w-10 h-4 rounded-md" />{`)`}
                </Section.Title>
            {:else}
                <Section.Title>
                    Friends ({friendsCount})
                </Section.Title>
            {/if}

            <Section.Content>
                {#await fetchFriends()}
                    {#each Array(8).map(() => 0) as _}
                        <UserCardSkeleton />
                    {/each}
                {:then friends}
                    {#each friends as friend}
                        <UserCard user={friend} />
                    {/each}
                {/await}
            </Section.Content>
        </Section.Root>

        {#await fetchAvatar()}
            <Section.Root class="user-avatar">
                <Section.Title>Currenly wearing</Section.Title>
                <Section.Content>
                    <div class="avatar-container">
                        <Skeleton class="w-full h-full rounded-md" />
                    </div>
                    <div class="avatar-items-container">
                        <Skeleton class="w-full h-full rounded-md" />
                    </div>
                </Section.Content>
            </Section.Root>
        {:then [avatarURL]}
            <Section.Root class="user-avatar">
                <Section.Title>Currenly wearing</Section.Title>
                <Section.Content>
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
                </Section.Content>
            </Section.Root>
        {/await}
    </main>
{/await}
