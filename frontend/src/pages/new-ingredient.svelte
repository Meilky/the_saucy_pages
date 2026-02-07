<script lang="ts">
	import CreateIngredientForm from "../components/ingredient/create-form.svelte";

	function onSubmited() {
		navigation.navigate("/ingredients");
	}
</script>

<CreateIngredientForm {onSubmited} />
