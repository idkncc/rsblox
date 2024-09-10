<script lang="ts">
    import * as Section from "@ui/section";
    import * as Select from "@ui/select";

    import { userPrefersMode, setMode, resetMode } from "mode-watcher";
</script>

<Section.Root>
    <Section.Title>Color scheme</Section.Title>
    <Section.Content>
        <Select.Root
            selected={{
                value: $userPrefersMode,
                label:
                    /* aka capitalize -v */
                    $userPrefersMode.charAt(0).toUpperCase() +
                    $userPrefersMode.slice(1),
            }}
            onSelectedChange={(selected) => {
                userPrefersMode.set(selected?.value ?? "system");
            }}
        >
            <Select.Trigger class="w-[180px]">
                <Select.Value placeholder="Color scheme" />
            </Select.Trigger>
            <Select.Content>
                <Select.Item value="light">Light</Select.Item>
                <Select.Item value="dark">Dark</Select.Item>
                <Select.Item value="system">System</Select.Item>
            </Select.Content>
        </Select.Root>
    </Section.Content>
</Section.Root>
