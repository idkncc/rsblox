<script lang="ts">
    import "./SettingsPage.scss";

    import * as Section from "@ui/section";
    import { page } from "$app/stores";
    import SettingsPageLink from "@components/Settings/SettingsPageLink.svelte";
</script>

<h1 class="text-2xl font-bold">Settings</h1>

<div class="settings-page">
    <Section.Root class="settings-sidebar">
        <Section.Content>
            <SettingsPageLink href="/settings" title="Home" />
            <SettingsPageLink href="/settings/theme" title="Theme" />
        </Section.Content>
    </Section.Root>

    <main class="settings-content">
        <slot></slot>
    </main>
</div>
