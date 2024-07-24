<div class="friend-card">
    <div class="friend-image">
        <div
            class="placeholder rounded-lg"
            style="width: 90px; height: 90px;"
        />
    </div>

    <div class="friend-username">
        <p class="user-name text-center">
            <span class="placeholder w-full rounded-lg" style="width: 70px" />
        </p>
    </div>
</div>

<style lang="scss">
    .friend-card {
        --size: 100px;

        @media screen and (max-width: 920px) {
            --size: 80px;
        }

        @apply flex flex-col;

        .friend-image {
            min-width: var(--size);
            min-height: var(--size);

            @apply aspect-square;
            @apply rounded-full relative;

            img {
                @apply w-full rounded-full;
            }
        }

        div.friend-username {
            @apply text-xs text-center;
            @apply flex gap-1 justify-center;

            width: var(--size);

            .user-name {
                max-width: calc(var(--size) - 10px);
                @apply truncate;
            }

            .user-status {
                @apply w-fit;
            }
        }
    }
</style>
