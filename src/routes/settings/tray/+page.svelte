<script lang="ts">
    import * as Section from "@ui/section";
    import * as Select from "@ui/select";
    import { Checkbox } from "@ui/checkbox";

    import { type Selected } from "bits-ui";

    import { userPrefersMode, setMode, resetMode } from "mode-watcher";

    const availableContentCategories = [
        {
            value: "continue",
            label: "Continue",
        },
        {
            value: "friends",
            label: "Friends",
        },
    ];

    let selectedContentCategory: Selected<string> | undefined;
</script>

<Section.Root>
    <Section.Title>Tray</Section.Title>
    <Section.Content>
        <div class="settings-field">
            <h3>Enabled</h3>

            <div class="settings-field-content">
                <Checkbox />
            </div>
        </div>
        <div class="settings-field">
            <h3>Content category</h3>

            <div class="settings-field-content">
                <Select.Root
                    selected={selectedContentCategory}
                    onSelectedChange={(selected) => {
                        selectedContentCategory = selected;
                    }}
                >
                    <Select.Trigger class="w-full">
                        <Select.Value placeholder="Select tray category" />
                    </Select.Trigger>
                    <Select.Content>
                        {#each availableContentCategories as category}
                            <Select.Item value={category.value}>
                                {category.label}
                            </Select.Item>
                        {/each}
                    </Select.Content>
                </Select.Root>
            </div>
        </div>
    </Section.Content>
</Section.Root>
