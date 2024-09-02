<script lang="ts">
	import {
		Heading,
		Table,
		TableBody,
		TableBodyCell,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';
	import type { PageServerData } from './$types';
	import { goto } from '$app/navigation';
	import { route } from '$lib/ROUTES';

	export let data: PageServerData;
</script>

<Heading tag="h2">Liste des semestres</Heading>

<Table>
	<TableHead>
		<TableHeadCell>Semetres</TableHeadCell>
	</TableHead>
	<TableBody>
		{#each data.semetres as semestre}
			<TableBodyCell
				on:click={() => {
					goto(
						route('/admin/semestre/[sem]', {
							sem: semestre.numero
						})
					);
				}}
				class="hover:bg-slate-300 dark:hover:bg-slate-700"
			>
				{semestre.numero}
			</TableBodyCell>
		{/each}
	</TableBody>
</Table>
