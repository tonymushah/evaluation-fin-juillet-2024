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
	$: etudiants = data.etudiants;
</script>

<svelte:head>
	<title>Semestre {data.semestre}</title>
</svelte:head>

<Heading tag={'h2'}>Semestre {data.semestre}</Heading>

<Table>
	<TableHead>
		<TableHeadCell>Rang</TableHeadCell>
		<TableHeadCell>Numero Etudiant</TableHeadCell>
		<TableHeadCell>Nom</TableHeadCell>
		<TableHeadCell>Prenom</TableHeadCell>
		<TableHeadCell>Moyenne</TableHeadCell>
	</TableHead>
	<TableBody>
		{#each etudiants as etudiant, rang}
			<TableBodyRow
				class="hover:bg-slate-300 dark:hover:bg-slate-700"
				on:click={() => {
					goto(
						route('/admin/etudiant/[etu]', {
							etu: etudiant.etu
						})
					);
				}}
			>
				<TableBodyCell>
					{rang + 1}
				</TableBodyCell>
				<TableBodyCell>
					{etudiant.etu}
				</TableBodyCell>
				<TableBodyCell>
					{etudiant.nom}
				</TableBodyCell>
				<TableBodyCell>
					{etudiant.prenom}
				</TableBodyCell>
				<TableBodyCell>
					{etudiant.moyenne}
				</TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>
