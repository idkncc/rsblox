import {defineConfig} from "vite";

import tsconfigPaths from "vite-tsconfig-paths";
import vue from "@vitejs/plugin-vue";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    plugins: [
        tsconfigPaths(),
        vue()
    ],

    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        watch: {
            // 3. tell vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
    },
}));
