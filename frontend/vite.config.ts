import path from "path";

import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";

export default defineConfig({
	plugins: [svelte()],
	server: {
		host: "0.0.0.0",
		port: 5054,
		strictPort: true,
		proxy: {
			"/api": "http://0.0.0.0:5055",
		},
	},
	resolve: {
		alias: {
			$components: path.resolve(__dirname, "./src/components"),
			$api: path.resolve(__dirname, "./src/api"),
		},
	},
});
