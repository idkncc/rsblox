<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

import FriendCard from "../components/FriendCard.vue";
import {FriendUserInformation, RecommendationsTopic, TreatmentType} from "../utils/typings.ts";
import GameCard from "../components/GameCard.vue";

const friends = ref<Array<FriendUserInformation>>([])
const friendsHeadshots = ref<Array<string>>([])
const topics = ref<RecommendationsTopic[]>([])

const gameIcons = ref<Record<number, string>>({})

const PRESENCE_INDEXES: Record<string, number> = {
  Invisible: 0,
  Offline: 0,
  Online: 1,
  InGame: 2,
  InStudio: 2,
}


onMounted(async () => {
  invoke<FriendUserInformation[]>("plugin:roblox-api|friends_list")
      .then((v) => {
        console.log(v)

        const friendsArray = v
            .sort((a, b) => a.display_name.localeCompare(b.display_name))
            .sort((a, b) => PRESENCE_INDEXES[b.presence_type] - PRESENCE_INDEXES[a.presence_type])

        console.log(v)

        invoke<string[]>("plugin:roblox-api|get_headshots", {avatarIds: friendsArray.map((fr) => fr.user_id)})
            .then((friendsHeadshotsArray) => {
              friends.value = friendsArray
              friendsHeadshots.value = friendsHeadshotsArray
            })
            .catch(console.error)
      })
      .catch(console.error)
})

onMounted(async () => {
  topics.value = await invoke<RecommendationsTopic[]>("plugin:roblox-api|recommendations")
      .then((topics) => {
        return topics
            .filter((a) => a.treatment_type !== TreatmentType.FriendCarousel)
            .map((a) => ({
              ...a,
              recommendation_list: a.recommendation_list.slice(0, 24)
            }))
            .sort((a, b) =>
                a.topic_id === 100000003
                    ? -1
                    : (b.topic_id === 100000003
                        ? 1
                        : 0)
            )
      })
      .catch((err) => {
        console.error(err)

        return []
      })

  const iconsBatches = await Promise.all(topics.value.map(async (topic) => {
    if (topic.treatment_type === TreatmentType.Carousel) {
      const thumbnails = await invoke<RecommendationsTopic[]>(
          "plugin:roblox-api|get_icons",
          {universeIds: topic.recommendation_list.map((r) => r.universe_id)}
      )

      return topic.recommendation_list.map((r, i) => [r.universe_id, thumbnails[i]])
    } else {
      const thumbnails = await invoke<RecommendationsTopic[]>(
          "plugin:roblox-api|get_head_thumbnails",
          {placeIds: topic.recommendation_list.map((r) => r.root_place_id)}
      )

      return topic.recommendation_list.map((r, i) => [r.universe_id, thumbnails[i]])
    }
  }))

  gameIcons.value = Object.fromEntries(iconsBatches.flat(1))
})
</script>

<template>
  <section class="friends">
    <p class="section-title">Friends</p>
    <div class="section-content">
      <FriendCard
          v-for="(friend, i) in friends"

          :headshot="friendsHeadshots[i]"
          :friend="friend"
      />
    </div>
  </section>

  <section
      v-for="topic in topics"
      class="topic-section"
  >
    <p class="section-title">{{ topic.topic }}</p>
    <p
        v-if="topic.subtitle"
        class="section-subtitle"
    >
      {{ topic.subtitle }}
    </p>

    <div :class="['section-content', topic.treatment_type === TreatmentType.SortlessGrid ? 'sortless-grid' : 'carousel']">
      <GameCard
          v-for="game in topic.recommendation_list"
          :game="game"
          :thumbnail="gameIcons[game.universe_id]"
          :treatmentType="topic.treatment_type"
      />
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
        @apply grid-cols-1
      }

      @media screen and (max-width: 800px) {
        @apply grid-cols-2
      }

      @apply grid md:grid-cols-3 lg:grid-cols-4;
    }
  }
}
</style>