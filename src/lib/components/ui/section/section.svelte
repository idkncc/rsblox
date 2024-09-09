<script lang="ts">
    import type { HTMLAttributes } from "svelte/elements";
    import { cn } from "$lib/utils.js";

    type $$Props = HTMLAttributes<HTMLDivElement>;
    let className: $$Props["class"] = undefined;
    export { className as class };
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<section
    class={cn("mt-2", className)}
    {...$$restProps}
    on:click
    on:focusin
    on:focusout
    on:mouseenter
    on:mouseleave
>
    <slot />
</section>
