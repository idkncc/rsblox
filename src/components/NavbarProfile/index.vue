<script setup lang="ts">
import {ref, watch} from "vue";
import {useTippy} from "vue-tippy";
import NavbarProfileTooltip from "./NavbarProfileTooltip.vue";
import {ClientInfo, useRobloxApi} from "../../utils/robloxApi.ts";
import {invoke} from "@tauri-apps/api/tauri";

const userProfileRef = ref()

const props = defineProps<{
  clientInfo: ClientInfo | undefined
}>()

const robloxApi = useRobloxApi()

useTippy(userProfileRef, {
  content: NavbarProfileTooltip,
  placement: "bottom-end",

  interactive: true,
  duration: [0, 100],
  animateFill: true,
})


</script>

<template>
  <div class="flex gap-6 items-center">
    <p class="robux">
      <span class="robux-count">{{ props.clientInfo?.robux ?? "---" }}</span>
      <svg
          class="robux-icon aspect-square w-6"
          preserveAspectRatio="none" xml:space="preserve" viewBox="0 0 15 16.35"
          y="0px" x="0px" id="Layer_1" version="1.1">
        <path style="clip-rule:evenodd;fill:#FFFFFF;fill-rule:evenodd" id="path15"
              d="m 13.1,3.275 c 0.9,0.5 1.4,1.5 1.4,2.5 v 4.8 c 0,1 -0.5,2 -1.4,2.5 l -4.1,2.4 c -0.9,0.5 -2,0.5 -2.9,0 L 2,13.075 c -1,-0.5 -1.5,-1.5 -1.5,-2.5 v -4.8 c 0,-1 0.6,-2 1.4,-2.5 L 6,0.875 c 0.9,-0.5 2,-0.5 2.9,0 z m -6.6,-1.5 -4,2.4 c -0.6,0.3 -1,1 -1,1.6 v 4.7 c 0,0.7 0.4,1.4 1,1.7 l 4,2.4 c 0.6,0.3 1.3,0.3 1.9,0 l 4.1,-2.4 c 0.6,-0.3 1,-1 1,-1.7 v -4.7 c 0,-0.6 -0.4,-1.3 -1,-1.6 l -4,-2.4 c -0.6,-0.3 -1.4,-0.3 -2,0 z m 2,1.2 3,1.7 c 0.6,0.4 1,1.1 1,1.8 v 3.3 c 0,0.8 -0.4,1.4 -1,1.8 l -3,1.8 c -0.6,0.4 -1.4,0.4 -2.1,0 l -2.9,-1.7 c -0.6,-0.4 -1,-1.1 -1,-1.8 v -3.4 c 0,-0.7 0.4,-1.4 1,-1.8 l 3,-1.7 c 0.6,-0.3 1.4,-0.3 2,0 z m -3,7.2 h 4 v -4 h -4 z"
              class="st2"/>
      </svg>
    </p>

    <p ref="userProfileRef">{{ props.clientInfo?.display_name ?? "Loading..." }}</p>
  </div>
</template>

<style scoped>
.robux {
  @apply flex gap-2 items-center;
}
</style>