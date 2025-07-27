<script lang="ts">
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import * as Sidebar from "$lib/components/ui/sidebar";
    import * as Avatar from "$lib/components/ui/avatar";
    import * as Dialog from "$lib/components/ui/dialog";
    import * as Alert from "$lib/components/ui/alert";
    import * as Tabs from "$lib/components/ui/tabs";

    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import { Badge } from "$lib/components/ui/badge";
    import { Button } from "../ui/button";

    import { ChevronUp, ShieldQuestion, SignalHigh } from "@lucide/svelte";
</script>

{#if true}
    <Sidebar.MenuItem>
        <Sidebar.MenuButton>
            {#snippet child({ props })}
                <Dialog.Root>
                    <Dialog.Trigger {...props}>
                        <span class="w-full text-center"> Sign in </span>
                    </Dialog.Trigger>
                    <Dialog.Content class="gap-4">
                        <Dialog.Header>
                            <Dialog.Title>Authorization</Dialog.Title>
                        </Dialog.Header>

                        <Alert.Root>
                            <Alert.Title class="font-semibold">
                                <span class="flex">
                                    <ShieldQuestion />
                                    Why authorization is only with cookies?
                                </span>
                            </Alert.Title>
                            <Alert.Description>
                                <span>
                                    Unfortunately, it's very difficult to sign
                                    in using password/one-time-code/etc.
                                </span>
                                <span>
                                    Fastest and easiest way is by using
                                    <code>.ROBLOSECURITY</code> cookie.
                                </span>
                            </Alert.Description>
                        </Alert.Root>

                        <Tabs.Root value="cookie">
                            <Tabs.List class="w-full">
                                <Tabs.Trigger value="cookie">
                                    Cookie
                                </Tabs.Trigger>
                                <Tabs.Trigger value="wip" disabled>
                                    Login/Password
                                </Tabs.Trigger>
                                <Tabs.Trigger value="wip" disabled>
                                    One-time Code
                                </Tabs.Trigger>
                            </Tabs.List>
                            <Tabs.Content value="cookie">
                                <div class="grid gap-2 py-4">
                                    <div
                                        class="grid grid-cols-4 items-center gap-4"
                                    >
                                        <Label for="name" class="text-right">
                                            Cookie
                                        </Label>
                                        <Input
                                            id="name"
                                            placeholder="|WARNING:-DO-NOT-SHARE-THIS.--Sharing..."
                                            class="col-span-3"
                                        />
                                    </div>
                                </div>
                            </Tabs.Content>
                        </Tabs.Root>

                        <Button>Sign in</Button>
                    </Dialog.Content>
                </Dialog.Root>
            {/snippet}
        </Sidebar.MenuButton>
    </Sidebar.MenuItem>
{:else}
    <Sidebar.Menu>
        <Sidebar.MenuItem>
            <DropdownMenu.Root>
                <DropdownMenu.Trigger>
                    {#snippet child({ props })}
                        <Sidebar.MenuButton
                            {...props}
                            class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                        >
                            <Avatar.Root>
                                <Avatar.Image
                                    src="https://github.com/Mon4ik.png"
                                    alt="@idkncc"
                                />
                                <Avatar.Fallback>CN</Avatar.Fallback>
                            </Avatar.Root>
                            idkncc

                            <span class="text-muted-foreground"
                                >(@idk_ncc240px)</span
                            >

                            <ChevronUp class="ml-auto" />
                        </Sidebar.MenuButton>
                    {/snippet}
                </DropdownMenu.Trigger>
                <DropdownMenu.Content
                    side="top"
                    class="w-(--bits-dropdown-menu-anchor-width)"
                >
                    <DropdownMenu.Item>
                        <a href="/">View Profile</a>
                    </DropdownMenu.Item>
                    <DropdownMenu.Item>
                        <span>Settings</span>
                    </DropdownMenu.Item>
                    <DropdownMenu.Separator />
                    <DropdownMenu.Item variant="destructive" disabled>
                        <span>Switch Account</span>
                        <Badge variant="outline">TODO</Badge>
                    </DropdownMenu.Item>
                    <DropdownMenu.Item variant="destructive">
                        <span>Sign out</span>
                    </DropdownMenu.Item>
                </DropdownMenu.Content>
            </DropdownMenu.Root>
        </Sidebar.MenuItem>
    </Sidebar.Menu>
{/if}
