<script lang="ts">
	import { commonDateToDate } from '$lib';
	import { Heading, Hr, P } from 'flowbite-svelte';
	import type { LayoutServerData } from './$types';
	import { Genre } from '$lib/protos/commons';

	export let data: LayoutServerData;
	$: etudiant = data.etudiant;
	$: isAges = etudiant.age > 1;
</script>

<div class="mx-10">
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
			{/if}</P
		>
	</div>
	<slot />
</div>
