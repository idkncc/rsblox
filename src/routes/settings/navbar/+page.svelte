<script lang="ts">
    import { Button } from "@ui/button";
    import * as Section from "@ui/section";
    import * as Select from "@ui/select";
    import * as Dialog from "@ui/dialog";

    import ShortcutsDialog from "@components/Settings/Navbar/ShortcutsDialog.svelte";

    import { userPrefersMode, setMode, resetMode } from "mode-watcher";
    import { Input } from "@ui/input";
</script>

<Section.Root>
    <Section.Title>Navbar</Section.Title>
    <Section.Description
        >Navbar – is line on the top with your robux, your profile and links at
        the left</Section.Description
    >
    <Section.Content>
        <Dialog.Root>
            <Dialog.Trigger>
                <Button class="w-full">Select shortcuts</Button>
            </Dialog.Trigger>
            <Dialog.Content>
                <Dialog.Header>
                    <Dialog.Title>Shortcuts</Dialog.Title>
                    <Dialog.Description>
                        Drag&Drop shortcuts, that you like to see
                    </Dialog.Description>

                    <ShortcutsDialog />
                </Dialog.Header>
            </Dialog.Content>
        </Dialog.Root>
    </Section.Content>
</Section.Root>
