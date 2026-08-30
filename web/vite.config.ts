import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// https://vite.dev/config/
export default defineConfig({
    plugins: [svelte()],
    server: { fs: { allow: [".."] } },
    base: process.env.NODE_ENV === "production" ? "/futoshiki/" : "/",
});
