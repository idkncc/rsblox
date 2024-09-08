<script setup lang="ts">
    import { type GameDetails } from "$lib/typings";
    import * as Section from "@ui/section";

    export let gameDetails: GameDetails;
</script>

<Section.Root>
    <Section.Content>
        <section class="game-tab">
            <p style="white-space: pre-wrap;" class="w-full">
                {gameDetails.description}
            </p>
        </section>
    </Section.Content>
</Section.Root>
