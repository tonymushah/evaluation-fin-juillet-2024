<script lang="ts">
	import ReleveNoteTable from '$lib/common/component/ReleveNoteTable.svelte';
	import type { PageServerData } from './$types';
	export let data: PageServerData;
</script>

<svelte:head>
	<title>Semestre {data.releve.semestre}</title>
</svelte:head>

<ReleveNoteTable data={data.releve} />
