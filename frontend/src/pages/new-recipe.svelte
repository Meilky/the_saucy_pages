<script lang="ts">
	import CreateRecipeForm from "../components/recipe/create-form.svelte";

	function onSubmited() {
		navigation.navigate("/recipes")
	}
</script>

<CreateRecipeForm {onSubmited} />
