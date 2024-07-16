<script setup lang="ts">

import {ref} from "vue";
import {Store} from "tauri-plugin-store-api";
import {STORE_PATH} from "../constants.ts";

const cookie = ref("")

async function loginCookie() {
  const store = new Store(STORE_PATH);
  await store.set("roblox-cookie", cookie.value);
  await store.save()
}

</script>

<template>
  <h1>Log in</h1>
  <div class="alert warning my-2">
    <p>Currently only with cookie</p>
  </div>

  <div class="flex w-full gap-2">
    <input
        type="text"
        class="control-input flex-grow"
        placeholder="_|WARNING:-DO-NOT-SHARE-THIS.--Sharing-this-will-allow-someone-to-log-in-as-you-and-to-steal-your-ROBUX-and-items.|_..."

        v-model="cookie"
    >

    <button class="control-button" @click="loginCookie">Login</button>
  </div>
</template>

<style scoped>

</style>