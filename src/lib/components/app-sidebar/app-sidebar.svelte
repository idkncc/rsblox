<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar";
    import * as Avatar from "$lib/components/ui/avatar";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";

    import { Badge } from "$lib/components/ui/badge";

    import ChevronUp from "@lucide/svelte/icons/chevron-up";
    import Profile from "./profile.svelte";
    import {
        House,
        SearchIcon,
        Shirt,
        Store,
        Telescope,
        Users,
    } from "@lucide/svelte";

    const ITEMS = [
        {
            title: "Home",
            icon: House,
            url: "/",
        },
        {
            title: "Discover",
            icon: Telescope,
            url: "/discover",
            wip: true,
        },
        {
            title: "Friends",
            icon: Users,
            url: "/friends",
            wip: true,
        },
        {
            title: "Marketplace",
            icon: Store,
            url: "/marketplace",
            wip: true,
        },
        {
            title: "Avatar",
            icon: Shirt,
            url: "/avatar",
            wip: true,
        },
    ];
</script>

<Sidebar.Root>
    <Sidebar.Header class="text-lg font-semibold">rsblox</Sidebar.Header>
    <Sidebar.Content>
        <Sidebar.Group>
            <Sidebar.GroupContent>
                <Sidebar.Menu>
                    {#each ITEMS as item (item.title)}
                        <Sidebar.MenuItem>
                            <Sidebar.MenuButton>
                                {#snippet child({ props })}
                                    <a
                                        href={item.url}
                                        aria-disabled={item.wip || null}
                                        {...props}
                                    >
                                        <item.icon />
                                        <span>{item.title}</span>

                                        {#if item.wip}
                                            <Badge
                                                variant="outline"
                                                class="ml-auto">WIP</Badge
                                            >
                                        {/if}
                                    </a>
                                {/snippet}
                            </Sidebar.MenuButton>
                        </Sidebar.MenuItem>
                    {/each}
                </Sidebar.Menu>
            </Sidebar.GroupContent>
        </Sidebar.Group>
        <Sidebar.Group />
    </Sidebar.Content>
    <Sidebar.Footer>
        <Profile />
    </Sidebar.Footer>
    <Sidebar.Rail />
</Sidebar.Root>
