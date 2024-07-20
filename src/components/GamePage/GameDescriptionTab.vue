<script setup lang="ts">
import { GameDetails } from "../../utils/typings";

defineProps<{
    gameDetails: GameDetails
}>()
</script>

<template>
    <section class="game-tab">
        <p style="white-space: pre;">{{ gameDetails.description }}</p>
    </section>
</template>
