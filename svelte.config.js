import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),

    kit: {
        adapter: adapter(),
        alias: {
            "@/*": "./src/lib/*",
            "@ui/*": "./src/lib/components/ui/*",
        },
    },
    preprocess: vitePreprocess()
};

export default config;
