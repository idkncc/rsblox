<script lang="ts">
    import "./GameCard.scss";

    import {
        TreatmentType,
        type Recommendation,
        type SearchResult,
    } from "$lib/typings.js";
    import * as Avatar from "../ui/avatar";

    export let game: Recommendation | SearchResult;
    export let thumbnail: string;
    export let treatmentType: TreatmentType | undefined =
        TreatmentType.Carousel;
</script>

<a href="/game?id={game.universe_id}">
    <div class="game-card w-full" data-treatment-type={treatmentType}>
        <Avatar.Root class="rounded-md game-image">
            <Avatar.Image src={thumbnail} alt={game.name} />
            <Avatar.Fallback>{game.name.slice(0, 4)}</Avatar.Fallback>
        </Avatar.Root>

        <p class="game-title">{game.name}</p>
        <p class="game-playing">Playing {game.player_count}</p>
    </div>
</a>
