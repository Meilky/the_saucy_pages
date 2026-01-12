import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from "@tailwindcss/vite";
import path from "path";

export default defineConfig({
	plugins: [
		tailwindcss(),
		svelte()
	],
	server: {
		host: "0.0.0.0",
		port: 5054,
		strictPort: true,
		proxy: {
			"/api": "http://0.0.0.0:5055",
		}
	}, resolve: {
		alias: {
			$components: path.resolve(__dirname, "./src/components"),
			$api: path.resolve(__dirname, "./src/api")
		},
	},
})
