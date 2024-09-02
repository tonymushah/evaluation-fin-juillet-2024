<script lang="ts">
	import {
		Heading,
		Fileupload,
		Label,
		Helper,
		Button,
		P,
		Select,
		type SelectOptionType,
		Input
	} from 'flowbite-svelte';
	import { CheckCircleSolid, DownloadSolid, UploadSolid } from 'flowbite-svelte-icons';
	import type { ActionData, PageServerData } from './$types';
	import { enhance } from '$app/forms';
	export let form: ActionData;
	export let data: PageServerData;
	$: matieres = data.matieres.map<SelectOptionType<string>>((m) => {
		return {
			name: m,
			value: m
		};
	});
	$: promotions = data.promotions.map<SelectOptionType<string>>((m) => {
		return {
			name: m,
			value: m
		};
	});
</script>

<svelte:head>
	<title>Insertion par promotion</title>
</svelte:head>

<div class="flex w-screen justify-center">
	<div class="flex w-2/3 flex-col gap-4">
		<Heading>Insertion par promotion</Heading>
		{#if form?.errors}
			<P color="red">{form.errors}</P>
		{/if}
		<form method="post" class="grid gap-3" use:enhance>
			<div class="grid gap-1">
				<Label class="text-xl">Matiere</Label>
				<Select name="matiere" items={matieres} />
			</div>
			<div class="grid gap-1">
				<Label class="text-xl">Promotion</Label>
				<Select name="promotion" items={promotions} />
			</div>
			<div class="grid gap-1">
				<Label class="text-xl">Note</Label>
				<Input type="number" name="note" />
			</div>
			<div class="mt-2 flex justify-end">
				<Button type="submit">
					<div class="flex flex-row items-center justify-center gap-2">
						<UploadSolid />
						<P>Importer</P>
					</div>
				</Button>
			</div>
		</form>
	</div>
</div>
