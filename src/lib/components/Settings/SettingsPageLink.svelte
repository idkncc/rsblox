<script lang="ts">
    import { page } from "$app/stores";
    import { Button } from "@ui/button";

    export let href: string;
    export let title: string;
</script>

<a class:settings-page-active={$page.route.id === href} {href}>
    <Button variant="ghost" class="w-full">
        {title}
    </Button>
</a>
