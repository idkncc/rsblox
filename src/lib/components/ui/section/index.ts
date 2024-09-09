import Root from "./section.svelte";
import Content from "./section-content.svelte";
import Description from "./section-description.svelte";
import Title from "./section-title.svelte";

export {
    Root,
    Content,
    Description,
    Title,
    //
    Root as Section,
    Content as SectionContent,
    Description as SectionDescription,
    Title as SectionTitle,
};

export type HeadingLevel = "h1" | "h2" | "h3" | "h4" | "h5" | "h6";
