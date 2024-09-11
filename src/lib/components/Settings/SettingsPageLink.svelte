<script lang="ts">
    import { page } from "$app/stores";
    import { Badge } from "@ui/badge";
    import { Button } from "@ui/button";

    export let href: string;
    export let title: string;
    export let badge: "wip" | "beta" | "new" | undefined = undefined;
</script>

<a class:settings-page-active={$page.route.id === href} {href}>
    <Button variant="ghost" class="w-full flex">
        {title}

        <span class="flex-grow" />

        {#if badge !== undefined}
            {#if badge === "wip"}
                <Badge class="ml-2" variant="warning">WIP</Badge>
            {:else if badge === "beta"}
                <Badge class="ml-2" variant="destructive">BETA</Badge>
            {:else if badge === "new"}
                <Badge class="ml-2" variant="info">NEW</Badge>
            {/if}
        {/if}
    </Button>
</a>
