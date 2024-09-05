<div class="user-card">
    <div class="user-image">
        <div
            class="placeholder rounded-lg"
            style="width: 90px; height: 90px;"
        />
    </div>

    <div class="user-username">
        <p class="user-name text-center">
            <span class="placeholder w-full rounded-lg" style="width: 70px" />
        </p>
    </div>
</div>

<style lang="scss">
    .user-card {
        --size: 100px;

        @media screen and (max-width: 920px) {
            --size: 80px;
        }

        @apply flex flex-col;

        .user-image {
            min-width: var(--size);
            min-height: var(--size);

            @apply aspect-square;
            @apply rounded-full relative;

            img {
                @apply w-full rounded-full;
            }
        }

        div.user-username {
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
