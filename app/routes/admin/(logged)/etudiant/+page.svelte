<script lang="ts" context="module">
	import { Button, Input, Label } from 'flowbite-svelte';
</script>

<script lang="ts">
	import EtudiantTable from '$lib/admin/components/etudiant-page/EtudiantTable.svelte';
	import { type Etudiant } from '$lib/protos/commons';
	import { Heading } from 'flowbite-svelte';
	import { SearchSolid } from 'flowbite-svelte-icons';
	import type { PageServerData, PageServerLoad } from './$types';

	export let data: PageServerData;
	$: etudiants = data.res.list;
</script>

<svelte:head>
	<title>Liste des etudiants | Admin</title>
</svelte:head>

<div class="content z-0 mx-8">
	<Heading tag="h1" class="mb-4 text-2xl font-bold">Liste des etudiants</Heading>
	<section>
		<form method="get">
			<div class="mb-6 grid gap-6 md:grid-cols-2">
				<div>
					<Label for="nom" class="mb-2">Nom</Label>
					<Input type="text" id="nom" name="nom" placeholder="John" />
				</div>
				<div>
					<Label for="nom" class="mb-2">Promotion</Label>
					<Input type="text" id="prenom" placeholder="P5, P4,..." />
				</div>
			</div>
			<Button>
				<SearchSolid />
				Rechercher
			</Button>
		</form>
	</section>
	<section class="mt-10">
		<EtudiantTable {etudiants} />
	</section>
</div>
