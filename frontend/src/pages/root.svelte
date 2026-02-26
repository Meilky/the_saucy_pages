<script lang="ts">
	import RecipesPage from "./recipes.svelte";
	import NewRecipePage from "./new-recipe.svelte";

	import IngredientsPage from "./ingredients.svelte";
	import NewIngredientPage from "./new-ingredient.svelte";
	import RecipeDetailsPage from "./recipe-details.svelte";

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

	router.addRoute(
		/^\/recipes\/[0-9a-f]{8}(?:\-[0-9a-f]{4}){3}-[0-9a-f]{12}\/?$/,
		() => {
			CurrentPage = RecipeDetailsPage;
		},
	);

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

<div id="app-container">
	<div id="navbar">
		<span id="title">The saucy pages</span>
		<a href="/recipes">Recipes</a>
		<a href="/ingredients">Ingredients</a>
	</div>
	<div id="page">
		<CurrentPage />
	</div>
</div>

<style>
	#title {
		font-weight: bold;
	}

	#app-container {
		display: grid;
		grid-template-columns: 10vw auto;
		height: 100vh;
	}

	#navbar {
		display: flex;
		flex-direction: column;
		padding: 1em;
		border-right: 1px darkgray solid;
		background-color: lightgray;
	}

	#page {
		background-color: whitesmoke;
	}
</style>
