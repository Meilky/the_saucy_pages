import { mount } from "svelte";

import App from "./components/app.svelte";

const app = mount(App, {
	target: document.getElementById("app")!,
});

window.addEventListener("navigate", (event) => {
	console.log(event)
})

export default app;
