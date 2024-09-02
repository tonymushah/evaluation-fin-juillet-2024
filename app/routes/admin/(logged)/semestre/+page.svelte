<script lang="ts">
	import {
		Heading,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';
	import type { PageServerData } from './$types';
	import { goto } from '$app/navigation';
	import { route } from '$lib/ROUTES';

	export let data: PageServerData;
</script>

<svelte:head>
	<title>Liste des semestres</title>
</svelte:head>

<Heading tag="h2">Liste des semestres</Heading>

<Table>
	<TableHead>
		<TableHeadCell>Semetres</TableHeadCell>
	</TableHead>
	<TableBody>
		{#each data.semetres as semestre}
			<TableBodyRow
				class="hover:bg-slate-300 dark:hover:bg-slate-700"
				on:click={() => {
					goto(
						route('/admin/semestre/[sem]', {
							sem: semestre.numero
						})
					);
				}}
			>
				<TableBodyCell>
					{semestre.numero}
				</TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>
