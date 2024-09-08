<script lang="ts">
    import "./GameCard.scss";

    import { Skeleton } from "@ui/skeleton";
</script>

<div class="game-card">
    <Skeleton class="game-image rounded-md w-[120px] h-[120px]" />

    <p class="game-title"><Skeleton class=" w-20 h-3 rounded-full" /></p>
    <p class="game-playing"><Skeleton class=" w-10 h-2 rounded-full" /></p>
</div>
