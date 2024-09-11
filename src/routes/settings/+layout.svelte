<script lang="ts">
    import "./SettingsPage.scss";

    import * as Section from "@ui/section";
    import { Separator } from "@ui/separator";

    import SettingsPageLink from "@components/Settings/SettingsPageLink.svelte";
</script>

<!-- <h1 class="text-2xl font-bold">Settings</h1> -->

<div class="settings-page">
    <nav class="settings-sidebar">
        <!-- <Section.Root>
            <Section.Title>Roblox settings</Section.Title>
            <Section.Content>
                <SettingsPageLink
                    href="/settings/account-info"
                    title="Account Info"
                />
            </Section.Content>
        </Section.Root> -->
        <Section.Root class="settings-sidebar">
            <Section.Title class="">rsblox settings</Section.Title>
            <Section.Content>
                <SettingsPageLink
                    href="/settings/theme"
                    title="Theme"
                    badge="new"
                />
                <SettingsPageLink
                    href="/settings/tray"
                    title="Tray"
                    badge="wip"
                />
                <SettingsPageLink
                    href="/settings/navbar"
                    title="Navbar"
                    badge="wip"
                />
                <Separator />
                <SettingsPageLink href="/settings/about" title="About" />
                <!-- <SettingsPageLink href="/settings/theme" title="Tray" /> -->
            </Section.Content>
        </Section.Root>
    </nav>

    <main class="settings-content">
        <slot></slot>
    </main>
</div>
