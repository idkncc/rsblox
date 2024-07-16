<script setup lang="ts">
import 'tippy.js/animations/shift-away.css'
import 'tippy.js/animations/shift-away-subtle.css'

import {h, ref} from "vue";
import {useTippy} from 'vue-tippy'

import FriendTooltip from "./FriendTooltip.vue";

import type {FriendUserInformation} from "../utils/typings.ts";

const props = defineProps<{
  headshot: string
  friend: FriendUserInformation
  friendPresence: FriendUserInformation
}>()

const btn = ref()

useTippy(btn, {
  content: h(FriendTooltip, { message: 'Hello' }),
  placement: "bottom",

  duration: [0, 100],
  animateFill: true
})

</script>

<template>
  <div class="friend-card" ref="btn">
    <div class="friend-image">
      <img
          class="rounded-md"

          :src="headshot"
          :alt="props.friend.display_name"

          width="90"
          height="90"
      >

      <span
          v-if="props.friend.presence_type === 'Online'"
          class="bg-blue-600 user-status"
      >
        <svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg"
             width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
          <path fill-rule="evenodd"
                d="M12 4a4 4 0 1 0 0 8 4 4 0 0 0 0-8Zm-2 9a4 4 0 0 0-4 4v1a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-1a4 4 0 0 0-4-4h-4Z"
                clip-rule="evenodd"/>
        </svg>

      </span>

      <span
          v-if="props.friend.presence_type === 'InGame'"
          class="bg-green-600 user-status"
      >
       <svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
        <path fill-rule="evenodd" d="M5 5a2 2 0 0 0-2 2v3a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1V7a2 2 0 0 0-2-2H5Zm9 2a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H14Zm3 0a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H17ZM3 17v-3a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2Zm11-2a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H14Zm3 0a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H17Z" clip-rule="evenodd"/>
       </svg>
      </span>


      <span
          v-if="props.friend.presence_type === 'InStudio'"
          class="bg-yellow-600 user-status"
      >
       <svg  aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
        <path fill-rule="evenodd" d="M14 4.182A4.136 4.136 0 0 1 16.9 3c1.087 0 2.13.425 2.899 1.182A4.01 4.01 0 0 1 21 7.037c0 1.068-.43 2.092-1.194 2.849L18.5 11.214l-5.8-5.71 1.287-1.31.012-.012Zm-2.717 2.763L6.186 12.13l2.175 2.141 5.063-5.218-2.141-2.108Zm-6.25 6.886-1.98 5.849a.992.992 0 0 0 .245 1.026 1.03 1.03 0 0 0 1.043.242L10.282 19l-5.25-5.168Zm6.954 4.01 5.096-5.186-2.218-2.183-5.063 5.218 2.185 2.15Z" clip-rule="evenodd"/>
       </svg>
      </span>
    </div>

    <div class="friend-username">
      <p class="user-name">{{ props.friend.display_name }}</p>
    </div>
  </div>
</template>

<style scoped>
.friend-card {
  --size: 100px;

  @media screen and (max-width: 920px) {
    --size: 80px
  }

  @apply flex flex-col;

  .friend-image {
    min-width: var(--size);
    min-height: var(--size);

    @apply aspect-square;
    @apply bg-[#787878] rounded-full relative;

    img {
      @apply w-full rounded-full;
    }

    .user-status {
      @apply absolute right-0 bottom-0;
      @apply w-6 h-6 p-0.5;
      @apply text-center;
      @apply rounded-full;

      svg {
        @apply w-full h-full;
      }
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