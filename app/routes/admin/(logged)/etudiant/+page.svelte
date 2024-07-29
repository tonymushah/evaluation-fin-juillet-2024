<script lang="ts" context="module">
	import dayjs from 'dayjs';
	function generateEtu(limit = 10): Etudiant[] {
		const rand = random(0, limit, false);
		let res: Etudiant[] = [];
		for (let index = 0; index < rand; index++) {
			const birthdate = faker.date.birthdate();
			res.push({
				nom: faker.person.firstName(),
				prenom: faker.person.lastName(),
				numero: `ETU${index}`,
				dateNaissance: {
					jour: birthdate.getDate(),
					mois: birthdate.getMonth(),
					annee: birthdate.getFullYear()
				},
				age: dayjs(now()).diff(birthdate, 'years'),
				promotion: 'Some promotion',
				genre: Genre.G_AUTRE
			});
		}
		return res;
	}
</script>

<script lang="ts">
	import EtudiantTable from '$lib/admin/components/etudiant-page/EtudiantTable.svelte';
	import { Genre, type Etudiant } from '$lib/protos/commons';
	import { faker } from '@faker-js/faker';
	import { now, random } from 'lodash-es';
	import { writable } from 'svelte/store';
	import { Heading } from 'flowbite-svelte';

	const etudiants = writable<Etudiant[]>(generateEtu());
</script>

<svelte:head>
	<title>Liste des etudiants | Admin</title>
</svelte:head>

<div class="content z-0 mx-8">
	<Heading tag="h1" class="mb-4 text-2xl font-bold">Liste des etudiants</Heading>

	<EtudiantTable etudiants={$etudiants} />
</div>
