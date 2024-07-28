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
				age: dayjs(now()).diff(birthdate, 'years')
			});
		}
		return res;
	}
</script>

<script lang="ts">
	import EtudiantTable from '$lib/admin/components/etudiant-page/EtudiantTable.svelte';
	import type { Etudiant } from '$lib/protos/commons';
	import { faker } from '@faker-js/faker';
	import { now, random } from 'lodash-es';
	import { writable } from 'svelte/store';

	const etudiants = writable<Etudiant[]>(generateEtu());
</script>

<svelte:head>
	<title>Liste des etudiants | Admin</title>
</svelte:head>

<div class="content mx-8">
	<h2 class="mb-4 text-xl font-bold">Liste des etudiants</h2>

	<EtudiantTable etudiants={$etudiants} />
</div>
