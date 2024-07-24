<script lang="ts">
	import { robloxApi } from "$lib/robloxApi";
	import type { GameServer } from "$lib/typings";

	export let gameServer: GameServer;
	export let avatarUrls: Record<string, string>;
	export let placeId: number;

	function join() {
		robloxApi.playServer(placeId, gameServer.id);
	}
</script>

<div class="game-server">
	<div class="game-server-avatars">
		{#each gameServer.player_tokens as playerToken}
			{#if avatarUrls[playerToken]}
				<img
					class="avatar-image"
					src={avatarUrls[playerToken]}
					alt="Player Headshot"
				/>
			{:else}
				<div
					class="placeholder avatar-image w-full h-full aspect-square"
				/>
			{/if}
		{/each}

		{#if gameServer.playing > 5}
			<div class="w-full h-full flex justify-center items-center">
				<p class="text-2xl font-bold">+{gameServer.playing - 5}</p>
			</div>
		{/if}
	</div>
	<p>{gameServer.playing} of {gameServer.max_players}</p>

	<div class="game-server-join">
		<button on:click={join}>Join</button>
	</div>
</div>

<style lang="scss">
	.game-server {
		@apply p-2;
		@apply bg-[#181818] rounded-lg;

		.game-server-avatars {
			@apply grid grid-cols-3 justify-items-center gap-2;
			@apply mb-2;

			.avatar-image {
				@apply rounded-md;
			}

			img.avatar-image {
				@apply bg-[#282828];
			}
		}

		.game-server-join {
			button {
				@apply px-2 py-1 w-full;
				@apply bg-green-600 rounded-md;
			}
		}
	}
</style>
