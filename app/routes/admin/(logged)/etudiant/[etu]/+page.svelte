<script lang="ts">
	import { page } from '$app/stores';
	import SemestreTable from '$lib/admin/components/etudiant-page/semetres/SemestreTable.svelte';
	import { Heading } from 'flowbite-svelte';
	import type { PageServerData } from './$types';

	export let data: PageServerData;
	$: semestres = data.semestres;
</script>

<svelte:head>
	<title>Etudiant {$page.params.etu}</title>
</svelte:head>

<div class="my-2">
	<Heading class="text-4xl underline">Semestres</Heading>
</div>

<SemestreTable data={semestres} />
