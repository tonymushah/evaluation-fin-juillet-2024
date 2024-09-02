<script lang="ts">
	import {
		Heading,
		P,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';
	import type { PageServerData } from './$types';

	export let data: PageServerData;
	$: hasRatrapages = data.ratrapages.length > 0;
</script>

<svelte:head>
	<title>Liste des ratrapages</title>
</svelte:head>

<Heading tag="h2">Liste des ratrapages</Heading>

<P>Montant total: {data.total_montant} Ariary</P>

<Table>
	<TableHead>
		<TableHeadCell>Semestre</TableHeadCell>
		<TableHeadCell>Montant</TableHeadCell>
	</TableHead>
	<TableBody>
		{#each data.ratra_sem as ratra}
			<TableBodyRow color={ratra.montant >= 50000 ? 'green' : 'red'}>
				<TableBodyCell>
					{ratra.sem}
				</TableBodyCell>
				<TableBodyCell>
					{ratra.montant}
				</TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>

<Heading tag="h3">Liste par matiere</Heading>

<Table>
	<TableHead>
		<TableHeadCell>Semestre</TableHeadCell>
		<TableHeadCell>Matiere</TableHeadCell>
		<TableHeadCell>Note d'origine</TableHeadCell>
		<TableHeadCell>Montant</TableHeadCell>
	</TableHead>
	<TableBody>
		{#each data.ratrapages as ratra}
			<TableBodyRow>
				<TableBodyCell>
					{ratra.semestre}
				</TableBodyCell>
				<TableBodyCell>
					{ratra.matiere}
				</TableBodyCell>
				<TableBodyCell>
					{ratra.note ?? 0}
				</TableBodyCell>
				<TableBodyCell>
					{ratra.montant}
				</TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</Table>
