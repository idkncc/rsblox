<script setup lang="ts">
import {GameDetails} from "../utils/typings.ts";

const props = defineProps<{
  game: GameDetails
}>()

</script>

<template>
  <div class="friend-tooltip">
    <img src="https://placehold.co/384x216" alt="">

    <div class="friend-tooltip-footer">
      <p>Nigos negbots</p>
      <button class="join-button">Join</button>
    </div>
  </div>
</template>

<style scoped lang="scss">
.friend-tooltip {
  $width: 192px;
  $height: 108px;

  width: $width;
  height: $height;
  position: relative;

  @apply overflow-hidden;
  @apply rounded-lg;

  img {
    @apply absolute top-0 left-0;
    @apply w-full;

    height: $height;
  }

  .friend-tooltip-footer {
    @apply absolute bottom-0 left-0;
    @apply px-2 py-3;
    width: $width;

    @apply bg-gradient-to-t from-black to-white/0;

    .join-button {
      @apply px-2 py-1 w-full;
      @apply bg-green-600 rounded-sm
    }
  }
}
</style>