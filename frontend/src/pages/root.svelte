<script lang="ts">
	import RecipesPage from "./recipes.svelte";
	import NewRecipePage from "./new-recipe.svelte";

	import IngredientsPage from "./ingredients.svelte";
	import NewIngredientPage from "./new-ingredient.svelte";

	import NotFound from "./404.svelte";

	import { Router } from "../libs/router";
	import type { Component } from "svelte";

	let CurrentPage: Component = $state(NotFound);

	const router = new Router();

	router.addRoute(/^\/recipes\/?$/, () => {
		CurrentPage = RecipesPage;
	});

	router.addRoute(/^\/recipes\/new\/?$/, () => {
		CurrentPage = NewRecipePage;
	});

	router.addRoute(/^\/ingredients\/?$/, () => {
		CurrentPage = IngredientsPage;
	});

	router.addRoute(/^\/ingredients\/new\/?$/, () => {
		CurrentPage = NewIngredientPage;
	});

	router.addRoute(/.*/, () => {
		CurrentPage = NotFound;
	});

	function callback(event: any) {
		const url = new URL(event.destination.url);

		const handler = router.getHandlerByMatch(url.pathname);

		if (!handler) return;

		event.intercept({
			handler,
		});
	}

	$effect(() => {
		navigation.addEventListener("navigate", callback);

		const pathname = window.location.pathname;

		const handler = router.getHandlerByMatch(pathname);

		if (pathname == "/") {
			navigation.navigate("/recipes");
		} else if (handler) {
			handler();
		}

		return () => {
			navigation.removeEventListener("navigate", callback);
		};
	});
</script>

<a href="/recipes">Recipes</a>
<a href="/recipes/new">New recipe</a>
<a href="/ingredients">Ingredients</a>
<a href="/ingredients/new">New ingredient</a>

<CurrentPage />
