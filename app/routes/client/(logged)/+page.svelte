<script lang="ts">
	import { Heading, P } from 'flowbite-svelte';
	import type { PageServerData } from './$types';
	import { commonDateToDate } from '$lib';
	import { Etudiant, EtudiantStatus, Genre } from '$lib/protos/commons';
	import SemestreTable from '$lib/client/components/SemestreTable.svelte';

	export let data: PageServerData;
	$: etudiant = data.etudiant;
	$: isAges = etudiant.age > 1;
	$: semestres = data.semestres;
</script>

<svelte:head>
	<title>{etudiant.numero} | Etudiant</title>
</svelte:head>

<div>
	<Heading tag="h1">
		{etudiant.nom}
		{etudiant.prenom}
	</Heading>
	<P size="xl">Numero etudiant: {etudiant.numero}</P>
	{#if etudiant.dateNaissance}
		<P size="xl">
			Date de naissance : {commonDateToDate(etudiant.dateNaissance).toDateString()} ({etudiant.age}
			{#if isAges}ans{:else}an{/if})
		</P>
	{/if}
	<P size="xl">Promotion: {etudiant.promotion}</P>
	<P size="xl"
		>Genre: {#if etudiant.genre == Genre.G_FEMININ}
			Feminin
		{:else if etudiant.genre == Genre.G_MASCULIN}
			Masculin
		{:else}
			Autre
		{/if}
	</P>
	<P>Moyenne: {etudiant.moyenne}</P>
	<P
		>Status: {#if etudiant.status == EtudiantStatus.E_AJOURNEE}
			Ajournee
		{:else}
			Admis
		{/if}
	</P>
	<SemestreTable data={semestres} />
</div>
