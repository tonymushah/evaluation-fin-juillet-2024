<script lang="ts">
	import { Heading, P } from 'flowbite-svelte';
	import type { PageServerData } from './$types';
	import ConfigsTable from '$lib/admin/components/config/ConfigsTable.svelte';

	export let data: PageServerData;
</script>

<svelte:head>
	<title>Configuration notes</title>
</svelte:head>

<Heading tag="h2">Configuration</Heading>

<P>Commancer a configurer les donnes des maintenant</P>

<ConfigsTable data={data.list} />
