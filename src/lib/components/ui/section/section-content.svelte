<script lang="ts">
    import type { HTMLAttributes } from "svelte/elements";
    import { cn } from "$lib/utils.js";

    type $$Props = HTMLAttributes<HTMLDivElement>;

    let className: $$Props["class"] = undefined;
    export { className as class };
</script>

<div
    {...$$restProps}
    class={cn(
        "section-content bg-card text-card-foreground rounded-xl border shadow p-2 w-full ",
        className,
    )}
>
    <slot />
</div>
