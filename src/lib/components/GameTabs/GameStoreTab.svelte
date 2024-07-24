<script setup lang="ts">
    import { type GameDetails } from "$lib/typings";

    export let visible: boolean;
    export let gameDetails: GameDetails;
</script>

<section class="game-tab" class:hidden={!visible}>
    <div class="alert warning">I'm too broke to debug this menu, sooo</div>
</section>
