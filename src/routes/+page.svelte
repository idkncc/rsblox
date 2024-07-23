<script lang="ts">
    import { robloxApi } from "$lib/robloxApi";
    import chunk from "lodash.chunk";

    import FriendCard from "$lib/components/Cards/FriendCard.svelte";
    import GameCard from "$lib/components/Cards/GameCard.svelte";

    import {
        TreatmentType,
        ThumbnailSize,
        ThumbnailType,
        type InternalFriend,
        type FriendUserInformation,
        type RecommendationsTopic,
        type UserPresence,
    } from "$lib/typings.js";

    let friends: InternalFriend[] = [];
    let topics: RecommendationsTopic[] = [];
    let gameIcons: Record<number, string> = {};

    const PRESENCE_INDEXES: Record<string, number> = {
        Invisible: 0,
        Offline: 0,
        Online: 1,
        InGame: 2,
        InStudio: 2,
    };

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
    }
</script>

<section class="friends">
    <p class="section-title">Friends</p>
    <div class="section-content">
        {#await fetchFriends()}
            skeleton elements
        {:then}
            {#each friends as friend}
                <FriendCard {friend} />
            {/each}
        {/await}
    </div>
</section>

{#await fetchRecommendations()}
    skeleton elements
{:then}
    {#each topics as topic}
        <section class="topic-section">
            <p class="section-title">{topic.topic}</p>
            {#if topic.subtitle}
                <p class="section-subtitle">
                    {topic.subtitle}
                </p>
            {/if}

            <div
                class={topic.treatment_type === TreatmentType.SortlessGrid
                    ? "section-content sortless-grid"
                    : "section-content carousel"}
            >
                {#each topic.recommendation_list as game}
                    <GameCard
                        {game}
                        thumbnail="gameIcons[game.universe_id]"
                        treatmentType={topic.treatment_type}
                    />
                {/each}
            </div>
        </section>
    {/each}
{/await}

<style lang="scss">
    section {
        .section-subtitle {
            @apply text-xs text-[#C0C0C0];
        }

        .section-content {
            @apply border border-[#787878] bg-[#121212];
            @apply p-2 rounded-lg;
        }
    }

    section.friends {
        .section-content {
            @apply flex gap-2;
            @apply overflow-scroll;
        }
    }

    section.topic-section {
        @apply my-2;

        .section-content {
            @apply gap-2;
            @apply overflow-scroll;

            &.carousel {
                @apply flex;
            }

            &.sortless-grid {
                @media screen and (max-width: 400px) {
                    @apply grid-cols-1;
                }

                @media screen and (max-width: 800px) {
                    @apply grid-cols-2;
                }

                @apply grid md:grid-cols-3 lg:grid-cols-4;
            }
        }
    }
</style>
