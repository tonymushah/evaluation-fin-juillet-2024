<script lang="ts">
	import { Label, Input, Button, Helper } from 'flowbite-svelte';
	import type { ActionData } from './$types';
	import { enhance } from '$app/forms';
	export let form: ActionData;
</script>

<div class="flex h-screen w-screen items-center justify-center">
	<div class="grid h-2/3 w-2/3 grid-cols-2 rounded-md bg-slate-300 p-4 dark:bg-slate-700">
		<div class="flex h-full w-full items-center justify-center">
			<h1 class="text-3xl font-extrabold">ITU</h1>
		</div>
		<div class="flex h-full w-full flex-col items-center justify-center gap-5">
			<h2 class="text-2xl font-bold dark:text-white">Se Connecter</h2>
			{#if form?.error}
				<Helper color="red">{form.error}</Helper>
			{/if}
			<form method="post" class=" flex w-11/12 flex-col gap-5" use:enhance>
				<div>
					<Label class="mb-2" for="numero_etudiant">Numero etudiant</Label>
					<Input
						name="numero_etudiant"
						type="text"
						id="numero_etudiant"
						placeholder="1844"
						required
					/>
				</div>
				<Button type="submit">Se Connecter</Button>
			</form>
		</div>
	</div>
</div>
