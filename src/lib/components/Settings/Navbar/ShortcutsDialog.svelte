<script>
    import "./ShortcutsDialog.scss";

    import * as Section from "@ui/section";
    import { Input } from "@ui/input";
    import { Root } from "@ui/dialog";
</script>

<div class="settings-navbar">
    <div class="shortcuts">
        <p>Home</p>
        <p>Friends</p>
        <p>Discovery</p>
    </div>

    <div class="navbar-search">
        <div class="input-container">
            <Input
                class="control-input"
                type="search"
                placeholder="Search..."
            />
            <button class="search-button">
                <svg
                    class="search-icon"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-width="2"
                        d="m21 21-3.5-3.5M17 10a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z"
                    />
                </svg>
            </button>
        </div>
    </div>

    <p>John Doe</p>
</div>

<br />

<Section.Root>
    <Section.Title>Available elements</Section.Title>
    <Section.Content>
        <p class="cursor-default">Home</p>
        <p class="cursor-default">Friends</p>
        <p class="cursor-default">Discovery</p>
    </Section.Content>
</Section.Root>
