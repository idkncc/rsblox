<script setup lang="ts">
    import { type GameDetails } from "$lib/typings";

    export let visible: boolean;
    export let gameDetails: GameDetails;
</script>

<section class="game-tab" class:hidden={!visible}>
    <p style="white-space: pre-wrap;" class="w-full">
        {gameDetails.description}
    </p>
</section>
