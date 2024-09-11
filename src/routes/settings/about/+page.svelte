<script lang="ts">
    import * as Card from "@ui/card";
    import * as Section from "@ui/section";
    import * as Select from "@ui/select";

    import { userPrefersMode, setMode, resetMode } from "mode-watcher";
</script>

<Section.Root>
    <Section.Title>About rsblox</Section.Title>
    <Section.Content>
        <p>rsblox is created as replacement of using Roblox website</p>
        <p>
            project is still in development and gets new features with each
            version
        </p>
        <Section.Root>
            <Section.Content>
                <ul class="flex flex-row gap-4">
                    <li>
                        <a
                            class="text-blue-600 dark:text-blue-400 underline hover:font-medium"
                            href="https://github.com/idkncc/rsblox"
                            target="_blank"
                        >
                            Github
                        </a>
                    </li>
                    <li>
                        <a
                            class="text-blue-600 dark:text-blue-400 underline hover:font-medium"
                            href="https://github.com/idkncc/rsblox/issues"
                            target="_blank"
                        >
                            Issues
                        </a>
                    </li>
                    <li>
                        <a
                            class="text-blue-600 dark:text-blue-400 underline hover:font-medium"
                            href="https://github.com/orgs/idkncc/projects/1"
                            target="_blank"
                        >
                            TODO project
                        </a>
                    </li>
                </ul>
            </Section.Content>
        </Section.Root>
    </Section.Content>
</Section.Root>
