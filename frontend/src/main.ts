import "./app.css";
import { mount } from "svelte";

import RootPage from "./pages/root.svelte";

const rootPage = mount(RootPage, {
	target: document.body,
});

export default rootPage;
