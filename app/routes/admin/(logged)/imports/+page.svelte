<script lang="ts">
	import { Heading, Fileupload, Label, Helper, Button, P } from 'flowbite-svelte';
	import { CheckCircleSolid, DownloadSolid } from 'flowbite-svelte-icons';
	import type { ActionData } from './$types';
	import { enhance } from '$app/forms';
	export let form: ActionData;
	$: configuration = form?.errors.configuration;
	$: notes = form?.errors.notes;
</script>

<div class="flex w-screen justify-center">
	<div class="flex w-2/3 flex-col gap-4">
		<Heading>Import de donnees</Heading>
		<form method="post" class="grid gap-3" use:enhance enctype="multipart/form-data">
			<div class="grid gap-1">
				<Label class="text-xl">Configuration notes</Label>
				<Fileupload name="configuration" />
				{#if configuration}
					<Helper color="red">
						{configuration}
					</Helper>
				{:else}
					<Helper color="green">Imported configuration</Helper>
				{/if}
			</div>
			<div class="grid gap-1">
				<Label class="text-xl">Notes</Label>
				<Fileupload name="notes" />
				{#if notes}
					<Helper color="red">
						{notes}
					</Helper>
				{:else}
					<Helper color="green">Imported notes</Helper>
				{/if}
			</div>
			<div class="mt-2 flex justify-end">
				<Button type="submit">
					<div class="flex flex-row items-center justify-center gap-2">
						<DownloadSolid />
						<P>Importer</P>
					</div>
				</Button>
			</div>
		</form>
	</div>
</div>
