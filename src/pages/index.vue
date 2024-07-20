<script setup lang="ts">
import { onMounted, ref } from "vue";

import { useRobloxApi } from "../utils/robloxApi";
import chunk from "lodash.chunk";

import FriendCard from "../components/FriendCard.vue";
import GameCard from "../components/GameCard.vue";

import {
    FriendUserInformation,
    RecommendationsTopic,
    UserPresence,
    TreatmentType,
    ThumbnailSize,
    ThumbnailType
} from "../utils/typings.ts";

const robloxApi = useRobloxApi();

const friends = ref<Array<FriendUserInformation>>([]);
const friendsHeadshots = ref<Array<string>>([]);
const friendsPresences = ref<Record<number, UserPresence>>({});
const topics = ref<RecommendationsTopic[]>([]);

const gameIcons = ref<Record<number, string>>({});

const PRESENCE_INDEXES: Record<string, number> = {
    Invisible: 0,
    Offline: 0,
    Online: 1,
    InGame: 2,
    InStudio: 2,
};

onMounted(async () => {
    let friendsArray = await robloxApi.getFriendsList();

    friendsArray = friendsArray
        .sort((a, b) => PRESENCE_INDEXES[b.presence_type] - PRESENCE_INDEXES[a.presence_type])

    const [friendsPresencesArray, friendsHeadshotsArray] = await Promise.all([
        robloxApi.getPresences(friendsArray.map((fr) => fr.user_id)),
        robloxApi.getThumbnailsUrls(
            friendsArray.map((fr) => fr.user_id),
            ThumbnailSize.S150x150,
            ThumbnailType.AvatarHeadshot
        ),
    ]);

    friends.value = friendsArray;
    friendsHeadshots.value = friendsHeadshotsArray;
    friendsPresences.value = Object.fromEntries(
        friendsPresencesArray.map((presence) => [presence.user_id, presence])
    );
});

onMounted(async () => {
    topics.value = await robloxApi
        .getRecommendations()
        .then((topics) =>
            topics
                .filter((a) => a.treatment_type !== TreatmentType.FriendCarousel)
                .map((a) => {
                    if (a.topic_id === 100000003) {
                        a.recommendation_list = a.recommendation_list.slice(0, 8)
                    }

                    return a
                })
                .sort((a, b) =>
                    a.topic_id === 100000003 ? -1 : b.topic_id === 100000003 ? 1 : 0
                )
        )
        .catch((err) => {
            console.error(err);

            return [];
        });

    const iconsBatches = await Promise.all(
        topics.value.map(async (topic) => {
            if (topic.treatment_type === TreatmentType.Carousel) {
                const thumbnails = (await Promise.all(
                    chunk(topic.recommendation_list.map((r) => r.universe_id), 9)
                        .map((universeIdsChunk) => robloxApi.getThumbnailsUrls(
                            universeIdsChunk,
                            ThumbnailSize.S150x150,
                            ThumbnailType.GameIcon
                        ))
                )).flat(1);


                return topic.recommendation_list.map((r, i) => [
                    r.universe_id,
                    thumbnails[i],
                ]);
            } else {
                const thumbnails = (await Promise.all(
                    chunk(topic.recommendation_list.map((r) => r.root_place_id), 9)
                        .map((universeIdsChunk) => robloxApi.getThumbnailsUrls(
                            universeIdsChunk,
                            ThumbnailSize.S768x432,
                            ThumbnailType.GameThumbnail
                        ))
                )).flat(1);

                return topic.recommendation_list.map((r, i) => [
                    r.universe_id,
                    thumbnails[i],
                ]);
            }
        })
    );

    gameIcons.value = Object.fromEntries(iconsBatches.flat(1));
});
</script>

<template>
    <section class="friends">
        <p class="section-title">Friends</p>
        <div class="section-content">
            <FriendCard v-for="(friend, i) in friends" :headshot="friendsHeadshots[i]" :friend="friend"
                :friendPresence="friendsPresences[friend.user_id]" />
        </div>
    </section>

    <section v-for="topic in topics" class="topic-section">
        <p class="section-title">{{ topic.topic }}</p>
        <p v-if="topic.subtitle" class="section-subtitle">
            {{ topic.subtitle }}
        </p>

        <div :class="[
            'section-content',
            topic.treatment_type === TreatmentType.SortlessGrid
                ? 'sortless-grid'
                : 'carousel',
        ]">
            <GameCard v-for="game in topic.recommendation_list" :game="game" :thumbnail="gameIcons[game.universe_id]"
                :treatmentType="topic.treatment_type" />
        </div>
    </section>
</template>

<style scoped lang="scss">
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
