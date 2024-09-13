<script lang="ts">
    import "./HomePage.scss";

    import { robloxApi } from "$lib/robloxApi";
    import chunk from "lodash.chunk";

    import Skeleton from "@ui/skeleton/skeleton.svelte";
    import * as Section from "@ui/section";

    import UserCard from "@components/Cards/UserCard.svelte";
    import UserCardSkeleton from "@components/Cards/UserCardSkeleton.svelte";
    import GameCard from "@components/Cards/GameCard.svelte";
    import GameCardSkeleton from "@components/Cards/GameCardSkeleton.svelte";

    import {
        TreatmentType,
        ThumbnailSize,
        ThumbnailType,
        type InternalUser,
        type RecommendationsTopic,
    } from "$lib/typings.js";
    import { PRESENCE_INDEXES } from "$lib/constants";
    import { trayApi } from "$lib/trayApi";

    let friends: InternalUser[] = [];
    let topics: RecommendationsTopic[] = [];
    let gameIcons: Record<number, string> = {};

    async function fetchFriends() {
        let friendsArray = await robloxApi.getFriendsList();

        friendsArray = friendsArray.sort(
            (a, b) =>
                PRESENCE_INDEXES[b.presence_type] -
                PRESENCE_INDEXES[a.presence_type],
        );

        const [friendsPresencesArray, friendsHeadshotsArray] =
            await Promise.all([
                robloxApi.getPresences(friendsArray.map((fr) => fr.user_id)),
                robloxApi.getThumbnailsUrls(
                    friendsArray.map((fr) => fr.user_id),
                    ThumbnailSize.S150x150,
                    ThumbnailType.AvatarHeadshot,
                ),
            ]);

        friends = friendsArray.map((info, i) => ({
            info,
            headshot: friendsHeadshotsArray[i],
            presence: friendsPresencesArray[i],
        }));
    }

    async function fetchRecommendations() {
        topics = await robloxApi
            .getRecommendations()
            .then((topics) =>
                topics
                    .filter(
                        (a) =>
                            a.treatment_type !== TreatmentType.FriendCarousel,
                    )
                    .map((a) => {
                        if (a.topic_id === 100000003) {
                            a.recommendation_list = a.recommendation_list.slice(
                                0,
                                8,
                            );
                        }

                        return a;
                    })
                    .sort((a, b) =>
                        a.topic_id === 100000003
                            ? -1
                            : b.topic_id === 100000003
                              ? 1
                              : 0,
                    ),
            )
            .catch((err) => {
                console.error(err);

                return [];
            });

        const iconsBatches = await Promise.all(
            topics.map(async (topic) => {
                if (topic.treatment_type === TreatmentType.Carousel) {
                    const thumbnails = (
                        await Promise.all(
                            chunk(
                                topic.recommendation_list.map(
                                    (r) => r.universe_id,
                                ),
                                9,
                            ).map((universeIdsChunk) =>
                                robloxApi.getThumbnailsUrls(
                                    universeIdsChunk,
                                    ThumbnailSize.S150x150,
                                    ThumbnailType.GameIcon,
                                ),
                            ),
                        )
                    ).flat(1);

                    return topic.recommendation_list.map((r, i) => [
                        r.universe_id,
                        thumbnails[i],
                    ]);
                } else {
                    const thumbnails = (
                        await Promise.all(
                            chunk(
                                topic.recommendation_list.map(
                                    (r) => r.root_place_id,
                                ),
                                9,
                            ).map((universeIdsChunk) =>
                                robloxApi.getThumbnailsUrls(
                                    universeIdsChunk,
                                    ThumbnailSize.S768x432,
                                    ThumbnailType.GameThumbnail,
                                ),
                            ),
                        )
                    ).flat(1);

                    return topic.recommendation_list.map((r, i) => [
                        r.universe_id,
                        thumbnails[i],
                    ]);
                }
            }),
        );

        gameIcons = Object.fromEntries(iconsBatches.flat(1));

        trayApi.setGames(
            topics
                .find((topic) => topic.topic_id === 100000003)!
                .recommendation_list.map((recommendation) => ({
                    id: recommendation.root_place_id,
                    title: recommendation.name,
                })),
        );
    }
</script>

<Section.Root class="friends-section">
    <Section.Title>Friends</Section.Title>
    <Section.Content>
        {#await fetchFriends()}
            {#each Array(8).map(() => 0) as _}
                <UserCardSkeleton />
            {/each}
        {:then}
            {#each friends as friend}
                <UserCard user={friend} />
            {/each}
        {/await}
    </Section.Content>
</Section.Root>

{#await fetchRecommendations()}
    <!-- Recommendations skeleton  -->
    {#each Array(8).map(() => 0) as _}
        <Section.Root class="topic-section">
            <Section.Title>
                <Skeleton class="w-40 h-4 mb-1 rounded-full" />
            </Section.Title>

            <Section.Content class={"overflow-scroll gap-2 flex"}>
                {#each Array(10).map(() => 0) as _}
                    <GameCardSkeleton />
                {/each}
            </Section.Content>
        </Section.Root>
    {/each}
{:then}
    {#each topics as topic}
        <Section.Root class="topic-section">
            <Section.Title>{topic.topic}</Section.Title>

            {#if topic.subtitle}
                <Section.Description>{topic.subtitle}</Section.Description>
            {/if}

            <Section.Content
                class={topic.treatment_type === TreatmentType.SortlessGrid
                    ? "sortless-grid" /* sortless-grid */
                    : "carousel"}
            >
                {#each topic.recommendation_list as game}
                    <GameCard
                        {game}
                        thumbnail={gameIcons[game.universe_id]}
                        treatmentType={topic.treatment_type}
                    />
                {/each}
            </Section.Content>
        </Section.Root>
    {/each}
{/await}
